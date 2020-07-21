use k8s_openapi::api::core::v1::Container as KubeContainer;

pub struct Handle {
    container: KubeContainer
}
