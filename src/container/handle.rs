use crate::container::status::StatusWrapper;
use k8s_openapi::api::core::v1::Container as KubeContainer;
use crate::container::status::Wrapper as ContainerStatusWrapper;

pub struct Handle<C> {
    container: KubeContainer,
    status: C,
}

impl <C> Handle<C> {
    pub fn new(container: KubeContainer) -> Self 
        where C: ContainerStatusWrapper {
        let status = C::new(&container);
        Handle { container, status }
    }
}
