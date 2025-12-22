use std::pin::Pin;

pub mod builder;
pub mod graph;
pub mod runner;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
