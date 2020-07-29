use k8s_openapi::api::core::v1::Pod as KubePod;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::state::State;

// Really basic Kubelet for driving provider.
pub struct Kubelet<F, S> {
    state_factory: F, 
    provider_state: Arc<Mutex<S>>,
}

// Stub function, will be a channel or something.
async fn next_pod() -> Result<Option<KubePod>, ()> {
    Ok(Some(Default::default()))
}

impl<F, S> Kubelet<F, S> {
    pub fn new(state_factory: F, provider_state: S) -> Self {
        Kubelet {
            state_factory,
            provider_state: Arc::new(Mutex::new(provider_state)),
        }
    }

    pub async fn run<InitialState>(&self) -> anyhow::Result<()>
    where
        F: FnMut() -> InitialState,
        S: 'static + std::marker::Send,
    {
        while let Ok(Some(pod)) = next_pod().await {
            // let state = (self.state_factory)();
            // Not sure what to do here. 
        }
        Ok(())
    }
}
