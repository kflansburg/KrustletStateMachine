pub mod container;
pub mod pod;


#[cfg(test)]
mod tests {
    use k8s_openapi::api::core::v1::Pod as KubePod;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use k8s_openapi::api::core::v1::PodStatus as KubePodStatus;
    use k8s_openapi::api::core::v1::PodSpec as KubePodSpec;
    use k8s_openapi::api::core::v1::Container as KubeContainer;
    use crate::pod::status::Wrapper;
    use super::*;

    fn setup() -> pod::Manager<pod::status::StatusWrapper, container::status::StatusWrapper> {
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

        manager.update_status("default", "test", |pod_status| {
            pod_status.to_error("Foo error message")
        });
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
