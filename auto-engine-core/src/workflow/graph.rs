use crate::register::bus::NodeRegisterBus;
use crate::schema::node::NodeSchema;
use crate::types::node::SchemaField;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub node_id: String,
    pub node_context: NodeSchema,
    pub next: Vec<Arc<RwLock<GraphNode>>>,
    pub prev: Vec<String>,
    pub wait_count: Arc<AtomicUsize>,
}
pub struct Graph {
    pub nodes: HashMap<String, Arc<RwLock<GraphNode>>>,
    pub starts: Vec<Arc<RwLock<GraphNode>>>,
}

impl Graph {
    pub fn new(
        nodes: HashMap<String, Arc<RwLock<GraphNode>>>,
        starts: Vec<Arc<RwLock<GraphNode>>>,
    ) -> Graph {
        Graph { nodes, starts }
    }

    pub async fn node_params_from_ctx(
        &self,
        id: String,
        bus: Arc<RwLock<NodeRegisterBus>>,
    ) -> Result<HashMap<String, Vec<SchemaField>>, String> {
        let graph_node = {
            let option = self.nodes.get(&id);
            match option {
                None => return Ok(Default::default()),
                Some(n) => n.clone(),
            }
        };

        let prev_nodes = graph_node.read().await.prev.clone();
        get_prev_node_outputs(self.nodes.clone(), bus, prev_nodes).await
    }
}

fn get_prev_node_outputs(
    nodes: HashMap<String, Arc<RwLock<GraphNode>>>,
    bus: Arc<RwLock<NodeRegisterBus>>,
    prev_nodes: Vec<String>,
) -> Pin<Box<dyn Future<Output = Result<HashMap<String, Vec<SchemaField>>, String>> + Send>> {
    Box::pin(async move {
        let mut params = HashMap::new();
        for node_id in prev_nodes {
            let node = {
                let node = nodes.get(&node_id);
                match node {
                    None => return Err(format!("connection references missing node '{node_id}'")),
                    Some(n) => n.clone(),
                }
            };

            let (action, name,input_data, prev_nodes) = {
                let node_reader = node.read().await;
                (
                    node_reader.node_context.action_type.clone(),
                    node_reader.node_context.metadata.name.clone(),
                    node_reader.node_context.input_data.clone().unwrap_or_default(),
                    node_reader.prev.clone(),
                )
            };

            let node_define = {
                let bus = bus.read().await;
                let node_define = bus.load_node(&action);
                match node_define {
                    None => {
                        return Err(format!("Node {} not found", action));
                    }
                    Some(define) => define,
                }
            };
            params.insert(name.clone(), node_define.output_schema(input_data).clone());

            log::info!("params: {:?}", params);

            let result = get_prev_node_outputs(nodes.clone(), bus.clone(), prev_nodes).await;

            if let Ok(res) = result {
                params.extend(res);
            }
        }

        Ok(params)
    })
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::schema::node::Position;
    use crate::types::MetaData;
    use crate::types::node::{FieldType, I18nValue, NodeDefine};

    fn create_test_metadata(name: &str) -> MetaData {
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

    fn create_test_schema_field(name: &str, field_type: FieldType) -> SchemaField {
        SchemaField {
            name: name.to_string(),
            field_type,
            item_type: None,
            description: None,
            enums: vec![],
            default: None,
        }
    }

    struct MockNodeDefine {
        action_type: String,
        output_schema: Vec<SchemaField>,
    }

    impl NodeDefine for MockNodeDefine {
        fn action_type(&self) -> String {
            self.action_type.clone()
        }

        fn name(&self) -> I18nValue {
            I18nValue {
                zh: "测试节点".to_string(),
                en: "Test Node".to_string(),
            }
        }

        fn icon(&self) -> String {
            "test-icon".to_string()
        }

        fn category(&self) -> Option<I18nValue> {
            None
        }

        fn description(&self) -> Option<I18nValue> {
            None
        }

        fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
            self.output_schema.clone()
        }

        fn input_schema(&self) -> Vec<SchemaField> {
            vec![]
        }
    }

    #[tokio::test]
    async fn test_prev_parameters_with_nonexistent_node() {
        let nodes: HashMap<String, Arc<RwLock<GraphNode>>> = HashMap::new();
        let bus = Arc::new(RwLock::new(NodeRegisterBus::new()));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("nonexistent".to_string(), bus)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_prev_parameters_with_single_node_no_prev() {
        let mut bus = NodeRegisterBus::new();
        let output_fields = vec![
            create_test_schema_field("result", FieldType::String),
            create_test_schema_field("count", FieldType::Number),
        ];

        bus.register_node(
            "TestAction".to_string(),
            Box::new(MockNodeDefine {
                action_type: "TestAction".to_string(),
                output_schema: output_fields.clone(),
            }),
        );

        let node = GraphNode {
            node_id: "node-1".to_string(),
            node_context: NodeSchema {
                node_id: "node-1".to_string(),
                action_type: "TestAction".to_string(),
                metadata: create_test_metadata("test_node"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let mut nodes = HashMap::new();
        nodes.insert("node-1".to_string(), Arc::new(RwLock::new(node)));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("node-1".to_string(), Arc::new(RwLock::new(bus)))
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_prev_parameters_with_node_not_registered() {
        let bus = NodeRegisterBus::new();

        let prev_node = GraphNode {
            node_id: "node-1".to_string(),
            node_context: NodeSchema {
                node_id: "node-1".to_string(),
                action_type: "UnknownAction".to_string(),
                metadata: create_test_metadata("test_node"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let node = GraphNode {
            node_id: "node-2".to_string(),
            node_context: NodeSchema {
                node_id: "node-2".to_string(),
                action_type: "SecondAction".to_string(),
                metadata: create_test_metadata("consumer_node"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec!["node-1".to_string()],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let mut nodes = HashMap::new();
        nodes.insert("node-1".to_string(), Arc::new(RwLock::new(prev_node)));
        nodes.insert("node-2".to_string(), Arc::new(RwLock::new(node)));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("node-2".to_string(), Arc::new(RwLock::new(bus)))
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Node UnknownAction not found"));
    }

    #[tokio::test]
    async fn test_prev_parameters_with_prev_nodes() {
        let mut bus = NodeRegisterBus::new();

        // Register first node type
        bus.register_node(
            "FirstAction".to_string(),
            Box::new(MockNodeDefine {
                action_type: "FirstAction".to_string(),
                output_schema: vec![create_test_schema_field("output1", FieldType::String)],
            }),
        );

        // Register second node type
        bus.register_node(
            "SecondAction".to_string(),
            Box::new(MockNodeDefine {
                action_type: "SecondAction".to_string(),
                output_schema: vec![create_test_schema_field("output2", FieldType::Number)],
            }),
        );

        // Create first node (predecessor)
        let node1 = Arc::new(RwLock::new(GraphNode {
            node_id: "node-1".to_string(),
            node_context: NodeSchema {
                node_id: "node-1".to_string(),
                action_type: "FirstAction".to_string(),
                metadata: create_test_metadata("first_node"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        }));

        // Create second node with node1 as predecessor
        let node2 = GraphNode {
            node_id: "node-2".to_string(),
            node_context: NodeSchema {
                node_id: "node-2".to_string(),
                action_type: "SecondAction".to_string(),
                metadata: create_test_metadata("second_node"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec!["node-1".to_string()],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let mut nodes = HashMap::new();
        nodes.insert("node-1".to_string(), node1);
        nodes.insert("node-2".to_string(), Arc::new(RwLock::new(node2)));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("node-2".to_string(), Arc::new(RwLock::new(bus)))
            .await;

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.len(), 1);
        assert!(params.contains_key("first_node"));
        assert_eq!(params["first_node"].len(), 1);
    }

    #[tokio::test]
    async fn test_prev_parameters_with_multiple_prev_nodes() {
        let mut bus = NodeRegisterBus::new();

        bus.register_node(
            "Action1".to_string(),
            Box::new(MockNodeDefine {
                action_type: "Action1".to_string(),
                output_schema: vec![create_test_schema_field("out1", FieldType::String)],
            }),
        );

        bus.register_node(
            "Action2".to_string(),
            Box::new(MockNodeDefine {
                action_type: "Action2".to_string(),
                output_schema: vec![create_test_schema_field("out2", FieldType::Number)],
            }),
        );

        bus.register_node(
            "Action3".to_string(),
            Box::new(MockNodeDefine {
                action_type: "Action3".to_string(),
                output_schema: vec![create_test_schema_field("out3", FieldType::Boolean)],
            }),
        );

        let node1 = Arc::new(RwLock::new(GraphNode {
            node_id: "node-1".to_string(),
            node_context: NodeSchema {
                node_id: "node-1".to_string(),
                action_type: "Action1".to_string(),
                metadata: create_test_metadata("node1"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        }));

        let node2 = Arc::new(RwLock::new(GraphNode {
            node_id: "node-2".to_string(),
            node_context: NodeSchema {
                node_id: "node-2".to_string(),
                action_type: "Action2".to_string(),
                metadata: create_test_metadata("node2"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        }));

        let node3 = GraphNode {
            node_id: "node-3".to_string(),
            node_context: NodeSchema {
                node_id: "node-3".to_string(),
                action_type: "Action3".to_string(),
                metadata: create_test_metadata("node3"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec!["node-1".to_string(), "node-2".to_string()],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let mut nodes = HashMap::new();
        nodes.insert("node-1".to_string(), node1);
        nodes.insert("node-2".to_string(), node2);
        nodes.insert("node-3".to_string(), Arc::new(RwLock::new(node3)));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("node-3".to_string(), Arc::new(RwLock::new(bus)))
            .await;

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.len(), 2);
        assert!(params.contains_key("node1"));
        assert!(params.contains_key("node2"));
    }

    #[tokio::test]
    async fn test_prev_parameters_with_chain() {
        let mut bus = NodeRegisterBus::new();

        for i in 1..=3 {
            bus.register_node(
                format!("Action{}", i),
                Box::new(MockNodeDefine {
                    action_type: format!("Action{}", i),
                    output_schema: vec![create_test_schema_field(
                        &format!("out{}", i),
                        FieldType::String,
                    )],
                }),
            );
        }

        let node1 = Arc::new(RwLock::new(GraphNode {
            node_id: "node-1".to_string(),
            node_context: NodeSchema {
                node_id: "node-1".to_string(),
                action_type: "Action1".to_string(),
                metadata: create_test_metadata("node1"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        }));

        let node2 = Arc::new(RwLock::new(GraphNode {
            node_id: "node-2".to_string(),
            node_context: NodeSchema {
                node_id: "node-2".to_string(),
                action_type: "Action2".to_string(),
                metadata: create_test_metadata("node2"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec!["node-1".to_string()],
            wait_count: Arc::new(AtomicUsize::new(0)),
        }));

        let node3 = GraphNode {
            node_id: "node-3".to_string(),
            node_context: NodeSchema {
                node_id: "node-3".to_string(),
                action_type: "Action3".to_string(),
                metadata: create_test_metadata("node3"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec!["node-2".to_string()],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let mut nodes = HashMap::new();
        nodes.insert("node-1".to_string(), node1);
        nodes.insert("node-2".to_string(), node2);
        nodes.insert("node-3".to_string(), Arc::new(RwLock::new(node3)));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("node-3".to_string(), Arc::new(RwLock::new(bus)))
            .await;

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.len(), 2);
        assert!(params.contains_key("node1"));
        assert!(params.contains_key("node2"));
    }

    #[tokio::test]
    async fn test_prev_parameters_output_schema_content() {
        let mut bus = NodeRegisterBus::new();

        let output_fields = vec![
            create_test_schema_field("field1", FieldType::String),
            create_test_schema_field("field2", FieldType::Number),
            create_test_schema_field("field3", FieldType::Boolean),
        ];

        bus.register_node(
            "TestAction".to_string(),
            Box::new(MockNodeDefine {
                action_type: "TestAction".to_string(),
                output_schema: output_fields.clone(),
            }),
        );

        let node = GraphNode {
            node_id: "node-1".to_string(),
            node_context: NodeSchema {
                node_id: "node-1".to_string(),
                action_type: "TestAction".to_string(),
                metadata: create_test_metadata("test_node"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec![],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        let mut nodes = HashMap::new();
        nodes.insert("node-1".to_string(), Arc::new(RwLock::new(node)));

        let consumer = GraphNode {
            node_id: "node-2".to_string(),
            node_context: NodeSchema {
                node_id: "node-2".to_string(),
                action_type: "ConsumerAction".to_string(),
                metadata: create_test_metadata("consumer"),
                params: None,
                input_data: None,
                position: Position::default(),
                icon: None,
                type_define: None,
            },
            next: vec![],
            prev: vec!["node-1".to_string()],
            wait_count: Arc::new(AtomicUsize::new(0)),
        };

        nodes.insert("node-2".to_string(), Arc::new(RwLock::new(consumer)));
        let graph = Graph::new(nodes, vec![]);

        let result = graph
            .node_params_from_ctx("node-2".to_string(), Arc::new(RwLock::new(bus)))
            .await;

        assert!(result.is_ok());
        let params = result.unwrap();
        let schema = &params["test_node"];
        assert_eq!(schema.len(), 3);
        assert_eq!(schema[0].name, "field1");
        assert_eq!(schema[1].name, "field2");
        assert_eq!(schema[2].name, "field3");
    }
}
