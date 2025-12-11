use std::collections::{HashMap, HashSet};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio::sync::{Mutex, RwLock};
use crate::schema::workflow::WorkflowSchema;
use crate::workflow::graph::{Graph, GraphNode};

fn build_graph(
    node_id: String,
    graph_nodes: HashMap<String, Arc<RwLock<GraphNode>>>,
    edges: Arc<HashMap<String, Vec<String>>>,
    visited: Arc<Mutex<HashSet<String>>>,
    visiting: Arc<Mutex<HashSet<String>>>,
) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
    Box::pin(async move {
        if visited.lock().await.contains(&node_id) {
            return Ok(());
        }

        if !visiting.lock().await.insert(node_id.to_string()) {
            return Err(format!("workflow contains a cycle at node '{node_id}'"));
        }

        let Some(next_edges) = edges.get(&node_id) else {
            visited.lock().await.insert(node_id.to_string());
            visiting.lock().await.remove(&node_id);
            return Ok(());
        };

        let mut next_nodes = vec![];
        for next_node_id in next_edges.iter() {
            let rc_node = graph_nodes
                .get(next_node_id)
                .ok_or_else(|| format!("connection references missing node '{next_node_id}'"))?
                .clone();

            {
                let mut node_writer = rc_node.write().await;
                node_writer
                    .wait_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                // log::info!({})
                node_writer.prev.push(node_id.to_string());
            }

            build_graph(next_node_id.to_string(), graph_nodes.clone(), edges.clone(), visited.clone(), visiting.clone()).await?;

            next_nodes.push(rc_node);
        }

        if let Some(node) = graph_nodes.get(&node_id) {
            let mut node = node.write().await;
            node.next = next_nodes;
        }

        visiting.lock().await.remove(&node_id);
        visited.lock().await.insert(node_id.to_string());
        Ok(())
    })
}

#[derive(Default)]
pub struct Builder {
    pub workflow: WorkflowSchema,
}

impl Builder {
    pub fn new(workflow: WorkflowSchema) -> Self {
        Self {workflow}
    }

    pub async fn build(self) -> Result<Graph, String> {
        let mut graph_nodes: HashMap<String, Arc<RwLock<GraphNode>>> = HashMap::new();
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let mut start_nodes = vec![];

        for node_context in self.workflow.nodes.into_iter() {
            let key = node_context.node_id.clone();
            if node_context.action_type == "Start" {
                start_nodes.push(key.clone());
            }

            graph_nodes.insert(
                key.clone(),
                Arc::new(RwLock::new(GraphNode {
                    node_id: key,
                    node_context,
                    next: vec![],
                    prev: vec![],
                    wait_count: Arc::new(AtomicUsize::new(0)),
                })),
            );
        }

        for edge in self.workflow.connections.into_iter() {
            for start in start_nodes.iter() {
                if start == &edge.to {
                    return Err(
                        "The [Start] node cannot be used as the next node in the connection."
                            .to_string(),
                    );
                }
            }

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

        let mut graph: Vec<Arc<RwLock<GraphNode>>> = vec![];
        let visited = HashSet::new();
        let visiting = HashSet::new();
        for key in start_nodes.iter() {
            build_graph(key.to_string(), graph_nodes.clone(), Arc::new(edges.clone()), Arc::new(Mutex::new(visited.clone())), Arc::new(Mutex::new(visiting.clone()))).await?;
            if let Some(node) = graph_nodes.get(key) {
                graph.push(node.clone());
            }
        }

        Ok(Graph { nodes: graph_nodes, starts: graph })
    }
}

