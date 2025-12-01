use crate::node::mouse_move::node::MouseMoveNode;
use crate::node::mouse_move::runner::MouseMoveNodeFactory;
use crate::node::start::node::StartNode;
use crate::node::start::runner::StartRunnerFactory;
use crate::types::node::{NodeDefine, NodeRunner, NodeRunnerFactory};
use std::{collections::HashMap, sync::Arc};

pub struct NodeRegisterBus {
    nodes: HashMap<String, Arc<Box<dyn NodeDefine>>>,
    runner_factories: HashMap<String, Arc<Box<dyn NodeRunnerFactory>>>,
}

impl Default for NodeRegisterBus {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            runner_factories: HashMap::new(),
        }
    }
}

impl NodeRegisterBus {
    pub fn new() -> Self {
        let bus = Self {
            nodes: HashMap::new(),
            runner_factories: HashMap::new(),
        };

        bus
    }

    pub fn with_internal_nodes(mut self) -> NodeRegisterBus {
        self.register(
            Box::new(StartNode::new()),
            Box::new(StartRunnerFactory::new()),
        );
        self.register(
            Box::new(MouseMoveNode::new()),
            Box::new(MouseMoveNodeFactory::new()),
        );
        self
    }

    pub fn register(&mut self, node: Box<dyn NodeDefine>, factory: Box<dyn NodeRunnerFactory>) {
        let key = node.action_type();

        self.nodes.insert(key.clone(), Arc::new(node));
        self.runner_factories.insert(key, Arc::new(factory));
    }

    pub fn register_runner(&mut self, action_type: String, factory: Box<dyn NodeRunnerFactory>) {
        let key = action_type.clone();
        self.runner_factories.insert(key, Arc::new(factory));
    }

    pub fn register_node(&mut self, action_type: String, node: Box<dyn NodeDefine>) {
        let key = action_type.clone();
        self.nodes.insert(key, Arc::new(node));
    }

    pub fn list_nodes(&self) -> Vec<Arc<Box<dyn NodeDefine>>> {
        let mut res = vec![];
        for (_key, value) in self.nodes.iter() {
            res.push(Arc::clone(value));
        }
        res
    }

    pub fn load_node(&self, action_type: &str) -> Option<Arc<Box<dyn NodeDefine>>> {
        Some(self.nodes.get(action_type)?.clone())
    }

    pub fn create_runner(&self, key: &str) -> Option<Box<dyn NodeRunner>> {
        let factory = self.runner_factories.get(key)?.clone();
        Some(factory.create())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::types::node::{NodeDefine, NodeName, NodeRunner, NodeRunnerFactory};
    use schemars::Schema;
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestNodeDefine {
        action: String,
    }

    impl TestNodeDefine {
        fn new(action: &str) -> Self {
            Self {
                action: action.to_string(),
            }
        }
    }

    impl NodeDefine for TestNodeDefine {
        fn action_type(&self) -> String {
            self.action.clone()
        }

        fn name(&self) -> NodeName {
            NodeName {
                zh: "测试节点".to_string(),
                en: "test node".to_string(),
            }
        }

        fn icon(&self) -> String {
            "icon".to_string()
        }

        fn output_schema(&self) -> HashMap<String, String> {
            Default::default()
        }

        fn input_schema(&self) -> HashMap<String, String> {
            Default::default()
        }
    }

    struct TestRunnerFactory {
        counter: Arc<AtomicUsize>,
    }

    impl TestRunnerFactory {
        fn new() -> Self {
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn with_counter(counter: Arc<AtomicUsize>) -> Self {
            Self { counter }
        }
    }

    impl NodeRunnerFactory for TestRunnerFactory {
        fn create(&self) -> Box<dyn NodeRunner> {
            Box::new(TestRunner {
                counter: Arc::clone(&self.counter),
            })
        }
    }

    struct TestRunner {
        counter: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl NodeRunner for TestRunner {
        async fn run(&self, _ctx: &Context, _param: serde_json::Value) -> Result<(), String> {
            self.counter.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn register_and_list_nodes() {
        let mut bus = NodeRegisterBus::new();
        bus.register(
            Box::new(TestNodeDefine::new("action_a")),
            Box::new(TestRunnerFactory::new()),
        );

        let nodes = bus.list_nodes();
        assert_eq!(nodes.len(), 1);

        let node = nodes[0].clone();
        assert_eq!(node.action_type(), "action_a");
    }

    #[tokio::test]
    async fn create_runner_from_factory() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut bus = NodeRegisterBus::new();
        bus.register(
            Box::new(TestNodeDefine::new("action_b")),
            Box::new(TestRunnerFactory::with_counter(Arc::clone(&counter))),
        );

        let runner = bus
            .create_runner("action_b")
            .expect("runner should be created");

        let ctx = Context::new(PathBuf::new());
        runner
            .run(&ctx, json!({}))
            .await
            .expect("runner should execute successfully");

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn create_runner_returns_none_for_unknown_key() {
        let bus = NodeRegisterBus::new();
        assert!(bus.create_runner("unknown").is_none());
    }
}
