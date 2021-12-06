mod impls;

use std::sync::Arc;

use crate::network::{Network, Sandbox, Testnet};

pub struct Worker<T> {
    workspace: Arc<T>,
}

impl<T> Worker<T>
where
    T: Network,
{
    pub(crate) fn new(network: T) -> Self {
        Self {
            workspace: Arc::new(network),
        }
    }
}

pub fn sandbox() -> Worker<Sandbox> {
    Worker::new(Sandbox::new())
}

pub fn testnet() -> Worker<Testnet> {
    Worker::new(Testnet::new())
}

pub async fn with_sandbox<F, T>(task: F) -> T::Output
where
    F: Fn(Worker<Sandbox>) -> T,
    T: core::future::Future,
{
    task(sandbox()).await
}

pub async fn with_testnet<F, T>(task: F) -> T::Output
where
    F: Fn(Worker<Testnet>) -> T,
    T: core::future::Future,
{
    task(testnet()).await
}
