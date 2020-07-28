use crate::state::*;
use crate::state_machine::StateMachine;
use async_trait::async_trait;
use either::Either;
use k8s_openapi::api::core::v1::Pod as KubePod;
use std::sync::Arc;
use tokio::sync::Mutex;

//
// This is an example of our default implementation of the state graph.
//

pub struct StateMachineTest;

#[async_trait]
// Associated provider trait is bound to concrete statemachine here.
impl<P: 'static + Provider> StateMachine<P> for StateMachineTest {
    /// This actually walks the graph with relevant business logic.
    /// Could potentially be implemented in a more general way.
    /// But generally, this should be logic that providers dont have to reimplement every time.
    /// This will probably be broken up into methods on the struct.
    async fn run_to_completion(provider: Arc<Mutex<P>>, pod: KubePod) -> Result<(), ()> {
        let state: Status<Registered> = Status::default();

        // Image pull
        // TODO: Could this be made parallel?
        let state: Status<ImagePull> = state.into();
        for container in pod.spec.clone().unwrap().containers {
            // TODO: Apply pull policy
            let mut failures: u8 = 0;
            loop {
                {
                    // I'm thinking there may be a better way to do this that allows the provider
                    // to serve other pods while the image is pulling. (or especially in image
                    // pull backoff)
                    let mut p = provider.lock().await;
                    match p.image_pull(container.image.clone().unwrap()).await {
                        Ok(Either::Left(_)) => break,
                        Ok(Either::Right(_)) => failures += 1,
                        Err(e) => {
                            // Example of using the ToError trait rather than From.
                            // I initially tried returning the state so it isnt discarded, but
                            // the kubelet doesnt really know what to do with these generic types.
                            state
                                .to_error(&format!("Volume mount backoff failed: {:?}", e))
                                .await;
                            return Err(());
                        }
                    }
                }
                if failures > 3 {
                    // TODO Not sure of the best way to update the state within loops like this.
                    // Might have to use the wrapper again.
                    // state: Status<ImagePullBackoff> = state.into();
                    let mut p = provider.lock().await;
                    match p.image_pull_backoff().await {
                        Ok(_) => (),
                        Err(e) => {
                            state
                                .to_error(&format!("Volume mount backoff failed: {:?}", e))
                                .await;
                            return Err(());
                        }
                    }
                    failures = 0;
                }
            }
        }

        let state: Status<VolumeMount> = state.into();

        for container in pod.spec.clone().unwrap().containers {
            for volume_mount in container.volume_mounts.clone().unwrap_or_else(|| vec![]) {
                let mut failures: u8 = 0;
                loop {
                    {
                        let mut p = provider.lock().await;
                        match p.volume_mount(&volume_mount).await {
                            Ok(Either::Left(_)) => break,
                            Ok(Either::Right(_)) => failures += 1,
                            Err(e) => {
                                state
                                    .to_error(&format!("Volume mount backoff failed: {:?}", e))
                                    .await;
                                return Err(());
                            }
                        }
                    }
                    if failures > 3 {
                        // state: Status<VolumePullBackoff> = state.into();
                        let mut p = provider.lock().await;
                        match p.volume_mount_backoff().await {
                            Ok(_) => (),
                            Err(e) => {
                                state
                                    .to_error(&format!("Volume mount backoff failed: {:?}", e))
                                    .await;
                                return Err(());
                            }
                        }
                        failures = 0;
                    }
                }
            }
        }

        let state: Status<Starting> = state.into();
        let state: Status<Running> = state.into();

        // ... etc.

        let _state: Status<Completed> = state.into();
        Ok(())
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    //
    // Thes are methods the provider will need to implement.
    // TODO Would be nice to have something better than units returned in the Either.
    //
    async fn image_pull(&mut self, image: String) -> Result<Either<(), ()>, ()>;
    async fn volume_mount(
        &mut self,
        volume_mount: &k8s_openapi::api::core::v1::VolumeMount,
    ) -> Result<Either<(), ()>, ()>;

    //
    // We can provide sane defaults for these methods.
    //
    async fn image_pull_backoff(&mut self) -> Result<(), ()> {
        tokio::time::delay_for(std::time::Duration::from_secs(30)).await;
        Ok(())
    }

    async fn volume_mount_backoff(&mut self) -> Result<(), ()> {
        tokio::time::delay_for(std::time::Duration::from_secs(30)).await;
        Ok(())
    }
}
