use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use futures::future::join_all;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::types::node::NodeDefine;
use crate::{
    context::Context,
    event::{NODE_EVENT, NodeEventPayload, WORKFLOW_EVENT, WorkflowEventPayload, WorkflowStatus},
    notification::emitter::NotificationEmitter,
    register::bus::NodeRegisterBus,
    schema::{node::NodeSchema, workflow::WorkflowSchema},
};

#[derive(Debug, Clone)]
struct GraphNode {
    pub node_id: String,
    pub node_context: NodeSchema,
    pub next: Vec<Arc<std::sync::RwLock<GraphNode>>>,
}

fn build_graph(
    node_id: &str,
    graph_nodes: &HashMap<String, Arc<std::sync::RwLock<GraphNode>>>,
    edges: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
) -> Result<(), String> {
    if visited.contains(node_id) {
        return Ok(());
    }

    if !visiting.insert(node_id.to_string()) {
        return Err(format!("workflow contains a cycle at node '{node_id}'"));
    }

    let Some(next_edges) = edges.get(node_id) else {
        visited.insert(node_id.to_string());
        visiting.remove(node_id);
        return Ok(());
    };

    let mut next_nodes = vec![];
    for next_node_id in next_edges.iter() {
        let rc_node = graph_nodes
            .get(next_node_id)
            .ok_or_else(|| format!("connection references missing node '{next_node_id}'"))?
            .clone();

        build_graph(next_node_id, graph_nodes, edges, visited, visiting)?;

        next_nodes.push(rc_node);
    }

    if let Some(node) = graph_nodes.get(node_id) {
        let mut node = node.write().unwrap();
        node.next = next_nodes;
    }

    visiting.remove(node_id);
    visited.insert(node_id.to_string());
    Ok(())
}

#[derive(Debug)]
pub struct WorkflowRunner {
    graph: Vec<Arc<std::sync::RwLock<GraphNode>>>,
}

impl WorkflowRunner {
    pub fn create(workflow: WorkflowSchema) -> Result<Self, String> {
        let mut graph_nodes: HashMap<String, Arc<std::sync::RwLock<GraphNode>>> = HashMap::new();
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let mut start_nodes = vec![];

        for (i, node_context) in workflow.nodes.into_iter().enumerate() {
            let key = format!("node-{i}");
            if node_context.action_type == "Start" {
                start_nodes.push(key.clone());
            }

            graph_nodes.insert(
                key.clone(),
                Arc::new(std::sync::RwLock::new(GraphNode {
                    node_id: key,
                    node_context,
                    next: vec![],
                })),
            );
        }

        for edge in workflow.connections.into_iter() {
            if !graph_nodes.contains_key(&edge.from) {
                return Err(format!(
                    "connection references missing node '{}'",
                    edge.from
                ));
            }
            if !graph_nodes.contains_key(&edge.to) {
                return Err(format!("connection references missing node '{}'", edge.to));
            }

            let entry = edges.entry(edge.from).or_default();
            entry.push(edge.to);
        }

        if start_nodes.is_empty() {
            return Err("workflow missing Start node".to_string());
        }

        let mut graph: Vec<Arc<std::sync::RwLock<GraphNode>>> = vec![];
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        for key in start_nodes.iter() {
            build_graph(key, &graph_nodes, &edges, &mut visited, &mut visiting)?;
            if let Some(node) = graph_nodes.get(key) {
                graph.push(node.clone());
            }
        }

        Ok(Self { graph })
    }

    pub async fn run(
        &self,
        ctx: Arc<Context>,
        token: CancellationToken,
        bus: Arc<RwLock<NodeRegisterBus>>,
        emitter: Arc<NotificationEmitter>,
    ) -> Result<(), String> {
        emitter.clone().emit(
            WORKFLOW_EVENT,
            WorkflowEventPayload {
                status: WorkflowStatus::Running,
            },
        )?;

        let res = handle_node(self.graph.clone(), ctx, token, bus, emitter.clone()).await;

        emitter.emit(
            WORKFLOW_EVENT,
            WorkflowEventPayload {
                status: WorkflowStatus::Finished,
            },
        )?;

        res
    }
}

async fn handle_node(
    graph: Vec<Arc<std::sync::RwLock<GraphNode>>>,
    ctx: Arc<Context>,
    token: CancellationToken,
    bus: Arc<RwLock<NodeRegisterBus>>,
    emitter: Arc<NotificationEmitter>,
) -> Result<(), String> {
    let mut tasks = Vec::new();

    for node in graph.iter() {
        let bus = bus.clone();

        let node = node.clone();
        let ctx = ctx.clone();
        let token_clone = token.clone();
        let emitter_clone = emitter.clone();

        let (node_id, action, node_context, next_nodes) = {
            let node_reader = node.read().unwrap();
            (
                node_reader.node_id.clone(),
                node_reader.node_context.action_type.clone(),
                node_reader.node_context.clone(),
                node_reader.next.clone(),
            )
        };

        let node_id_clone = node_id.clone();
        let handle = async move {
            let emitter = emitter_clone;
            emitter
                .emit(NODE_EVENT, NodeEventPayload::running(node_id_clone.clone()))
                .unwrap_or_default();

            let (node, mut runner) = {
                let locked_bus = bus.read().await;
                let node = match locked_bus.load_node(&action) {
                    None => {
                        return Err(format!("Can't find node : {}", action));
                    }
                    Some(node) => node,
                };
                let runner = match locked_bus.create_runner(&action) {
                    Some(runner) => runner,
                    None => {
                        return Err(format!("Can't find action runner for node: {}", action));
                    }
                };
                (node, runner)
            };

            let input_data = node_context.input_data.clone().unwrap_or_default();
            let res = runner
                .run(
                    &ctx,
                    &node_context.metadata.name,
                    input_data,
                    node.input_schema(),
                )
                .await
                .inspect_err(|_e| {
                    emitter
                        .emit(
                            NODE_EVENT,
                            NodeEventPayload::error::<String>(node_id_clone.clone(), None),
                        )
                        .unwrap_or_default();
                })?;

            emitter
                .emit(
                    NODE_EVENT,
                    NodeEventPayload::success(node_id_clone, res),
                )
                .unwrap_or_default();

            handle_node(next_nodes, ctx, token_clone, bus, emitter).await?;

            Ok(())
        };

        let cancel_token = token.clone();

        let emitter = emitter.clone();
        tasks.push(async move {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    log::info!("Pipeline terminated, exiting loop");
                    emitter.emit(NODE_EVENT, NodeEventPayload::cancel()).unwrap_or_default();
                    Ok(())
                }
                res = handle => res,
            }
        });
    }
    for res in join_all(tasks).await {
        res?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::node::{NodeRunnerControl, NodeRunnerController};
    use crate::{
        register::bus::NodeRegisterBus,
        schema::{node::Position, workflow::Connection},
        types::{
            MetaData,
            node::{NodeRunner, NodeRunnerFactory},
        },
    };
    use serde_json::Value as JsonValue;
    use serde_yaml::Value;
    use std::path::PathBuf;
    use std::sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    };
    use tokio_util::sync::CancellationToken;

    fn metadata(name: &str) -> MetaData {
        MetaData {
            name: name.to_string(),
            description: None,
            duration: None,
            retry: None,
            interval: None,
            conditions: None,
            err_return: None,
        }
    }

    struct TestRunnerFactory {
        counter: Arc<AtomicUsize>,
        params: Arc<Mutex<Option<JsonValue>>>,
    }

    impl TestRunnerFactory {
        fn new(counter: Arc<AtomicUsize>, params: Arc<Mutex<Option<JsonValue>>>) -> Self {
            Self { counter, params }
        }
    }

    impl NodeRunnerFactory for TestRunnerFactory {
        fn create(&self) -> Box<dyn NodeRunnerControl> {
            Box::new(NodeRunnerController::new(TestRunner {
                counter: Arc::clone(&self.counter),
                params: Arc::clone(&self.params),
            }))
        }
    }

    struct TestRunner {
        counter: Arc<AtomicUsize>,
        params: Arc<Mutex<Option<JsonValue>>>,
    }

    #[async_trait::async_trait]
    impl NodeRunner for TestRunner {
        type ParamType = ();

        async fn run(
            &mut self,
            _ctx: &Context,
            param: Self::ParamType,
        ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
            let value: JsonValue = serde_json::to_value(param).map_err(|e| e.to_string())?;

            self.counter.fetch_add(1, Ordering::SeqCst);
            let mut guard = self
                .params
                .lock()
                .map_err(|e| format!("lock params failed: {e}"))?;
            *guard = Some(value);
            Ok(None)
        }
    }

    #[tokio::test]
    async fn run_executes_nodes_with_params() {
        let workflow = WorkflowSchema {
            nodes: vec![
                NodeSchema {
                    action_type: "Start".to_string(),
                    metadata: metadata("start"),
                    params: None,
                    input_data: None,
                    position: Position::default(),
                    icon: None,
                    type_define: None,
                },
                NodeSchema {
                    action_type: "Custom".to_string(),
                    metadata: metadata("custom"),
                    params: Some(HashMap::from([(
                        Value::String("foo".to_string()),
                        Value::String("bar".to_string()),
                    )])),
                    input_data: None,
                    position: Position::default(),
                    icon: None,
                    type_define: None,
                },
            ],
            connections: vec![Connection {
                from: "node-0".to_string(),
                to: "node-1".to_string(),
            }],
        };

        let runner = WorkflowRunner::create(workflow).expect("workflow should be valid");
        let counter = Arc::new(AtomicUsize::new(0));
        let params = Arc::new(Mutex::new(None));

        let mut bus = NodeRegisterBus::new().with_internal_nodes();
        bus.register_runner(
            "Custom".to_string(),
            Box::new(TestRunnerFactory::new(
                Arc::clone(&counter),
                Arc::clone(&params),
            )),
        );

        #[cfg(feature = "tauri")]
        let context = Context::new(PathBuf::new(), None);

        #[cfg(not(feature = "tauri"))]
        let context = Context::new(PathBuf::new());

        let ctx = Arc::new(context);

        let emitter = NotificationEmitter::new();

        runner
            .run(
                ctx,
                CancellationToken::new(),
                Arc::new(RwLock::new(bus)),
                Arc::new(emitter),
            )
            .await
            .expect("workflow should run successfully");

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn create_fails_on_cycle() {
        let workflow = WorkflowSchema {
            nodes: vec![
                NodeSchema {
                    action_type: "Start".to_string(),
                    metadata: metadata("start"),
                    params: None,
                    input_data: None,
                    position: Position::default(),
                    icon: None,
                    type_define: None,
                },
                NodeSchema {
                    action_type: "Custom".to_string(),
                    metadata: metadata("custom"),
                    params: None,
                    input_data: None,
                    position: Position::default(),
                    icon: None,
                    type_define: None,
                },
            ],
            connections: vec![
                Connection {
                    from: "node-0".to_string(),
                    to: "node-1".to_string(),
                },
                Connection {
                    from: "node-1".to_string(),
                    to: "node-0".to_string(),
                },
            ],
        };

        let err = WorkflowRunner::create(workflow).unwrap_err();
        assert!(err.contains("cycle"), "unexpected error message: {err}");
    }
}
