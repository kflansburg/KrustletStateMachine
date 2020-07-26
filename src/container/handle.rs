use crate::container::status::StatusWrapper;
use k8s_openapi::api::core::v1::Container as KubeContainer;

pub struct Handle {
    container: KubeContainer,
    status: StatusWrapper,
}

impl Handle {
    pub fn new(container: KubeContainer) -> Self {
        let status = StatusWrapper::new(&container);
        Handle { container, status }
    }
}
