pub mod container;
pub mod pod;


#[cfg(test)]
mod tests {
    use k8s_openapi::api::core::v1::Pod as KubePod;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use k8s_openapi::api::core::v1::PodStatus as KubePodStatus;
    use k8s_openapi::api::core::v1::PodSpec as KubePodSpec;
    use k8s_openapi::api::core::v1::Container as KubeContainer;
    use super::*;
    use super::pod::status::StatusWrapper as PodStatusWrapper;

    fn setup() -> pod::Manager {
        let mut manager = pod::Manager::new();

        let mut metadata: ObjectMeta = Default::default();
        metadata.namespace = Some("default".to_string());
        metadata.name = Some("test".to_string());

        let mut spec: KubePodSpec = Default::default();
        spec.containers.push(Default::default());
        spec.containers[0].name = "test".to_string();        
 
        let pod = KubePod {
            metadata: Some(metadata),
            spec: Some(spec),
            status: Some(Default::default())
        };

        manager.register_pod(pod); 
        manager
    }

    #[tokio::test]
    async fn error_handler() {
        // Consumes errors from channel and marks pods as in error state.
        let mut manager = setup();

        // This is kind of verbose but it forces us to deal with impossible state transitions in
        // the context of when they occur. 
        manager.update_status("default", "test", |pod_status| {
            match pod_status {
                PodStatusWrapper::Pending(status) => {
                    PodStatusWrapper::Failed(status.into())
                },
                PodStatusWrapper::Running(status) => {
                    PodStatusWrapper::Failed(status.into())
                },
                PodStatusWrapper::Succeeded(status) => {
                    // We have to explicitly handle impossible state transitions. 
                    panic!()
                },
                // Do we want some methods to overwrite metadata like "message" or "reason"
                // even when no state transition happens. 
                PodStatusWrapper::Failed(status) => {
                    PodStatusWrapper::Failed(status)
                }
            }
        }).await;
    }

    #[tokio::test]
    async fn eviction() {
        // Marks static pods as evicted on shutdown. Perhaps a better way to do this.
        let mut manager = setup();
    }

    #[tokio::test]
    async fn status_handler() {
        // Pod Handle consumes status channel and updates status.
        let mut manager = setup();
    }

    #[tokio::test]
    async fn provider_error() {
        // The provider may wish to indicate that a Pod has encountered an error. 
        let mut manager = setup();
    }

    #[tokio::test]
    async fn provider_stopped() {
        // The provider may wish to indicate that a Pod has stopped. 
        let mut manager = setup();
    }
}
