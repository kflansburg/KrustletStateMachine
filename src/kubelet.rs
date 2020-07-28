use crate::state_machine::StateMachine;
use k8s_openapi::api::core::v1::Pod as KubePod;
use std::sync::Arc;
use tokio::sync::Mutex;

// Really basic Kubelet for driving provider.

pub struct Kubelet<P> {
    provider: Arc<Mutex<P>>,
}

// Stub function, will be a channel or something.
async fn next() -> Result<Option<KubePod>, ()> {
    Ok(Some(Default::default()))
}

impl<P> Kubelet<P> {
    pub fn new(provider: P) -> Self {
        Kubelet {
            provider: Arc::new(Mutex::new(provider)),
        }
    }

    pub async fn run<S: StateMachine<P>>(&self)
    where
        P: 'static + std::marker::Send,
    {
        while let Ok(Some(pod)) = next().await {
            let provider = self.provider.clone();
            tokio::spawn(async move {
                S::run_to_completion(provider, pod).await.ok();
            });

            // Only run for one loop since this is a test.
            break;
        }
    }
}
