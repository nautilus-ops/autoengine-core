use crate::node::image_match::node::ImageMatchNode;
use crate::node::image_match::runner::ImageMatchRunnerFactory;
use crate::node::keyboard::node::KeyboardNode;
use crate::node::keyboard::runner::KeyboardNodeFactory;
use crate::node::mouse_click::node::MouseClickNode;
use crate::node::mouse_click::runner::MouseClickNodeFactory;
use crate::node::mouse_move::node::MouseMoveNode;
use crate::node::mouse_move::runner::MouseMoveNodeFactory;
use crate::node::start::node::StartNode;
use crate::node::start::runner::StartRunnerFactory;
use crate::types::node::{NodeDefine, NodeRunner, NodeRunnerControl, NodeRunnerFactory};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct NodeRegisterBus {
    nodes: HashMap<String, Arc<Box<dyn NodeDefine>>>,
    runner_factories: HashMap<String, Arc<Box<dyn NodeRunnerFactory>>>,
}

impl NodeRegisterBus {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            runner_factories: HashMap::new(),
        }
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
        self.register(
            Box::new(MouseClickNode::new()),
            Box::new(MouseClickNodeFactory::new()),
        );
        self.register(
            Box::new(KeyboardNode::new()),
            Box::new(KeyboardNodeFactory::new()),
        );
        self.register(
            Box::new(ImageMatchNode::new()),
            Box::new(ImageMatchRunnerFactory::new()),
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

    pub fn create_runner(&self, key: &str) -> Option<Box<dyn NodeRunnerControl>> {
        let factory = self.runner_factories.get(key)?.clone();
        Some(factory.create())
    }
}
