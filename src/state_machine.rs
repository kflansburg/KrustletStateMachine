use async_trait::async_trait;
use k8s_openapi::api::core::v1::Pod as KubePod;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait StateMachine<P> {
    // Drive the state machine to completion.
    // We had discussed an iterative method like next(),
    // but futures already do this, so I think by implementing this method using
    // async function calls, you get that behavior.
    async fn run_to_completion(provider: Arc<Mutex<P>>, pod: KubePod) -> anyhow::Result<()>;
}
