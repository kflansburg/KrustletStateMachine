pub mod concrete_provider;
pub mod concrete_state_machine;
pub mod kubelet;
pub mod provider;
pub mod state;
pub mod state_machine;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let provider = concrete_provider::ProviderTest;
        let kubelet = kubelet::Kubelet::new(provider);
        kubelet
            .run::<concrete_state_machine::StateMachineTest>()
            .await;
    }
}
