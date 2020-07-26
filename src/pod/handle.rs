use crate::container::handle::Handle as ContainerHandle;
use crate::pod::status::StatusWrapper;
use k8s_openapi::api::core::v1::Pod as KubePod;
use std::collections::BTreeMap;

pub struct Handle {
    pod: KubePod,
    containers: BTreeMap<String, ContainerHandle>,
    status: StatusWrapper,
}

impl Handle {
    pub fn new(pod: KubePod) -> Self {
        let mut containers = BTreeMap::new();
        for container in &pod.spec.as_ref().unwrap().containers {
            containers.insert(
                container.name.clone(),
                ContainerHandle::new(container.clone()),
            );
        }

        let status = StatusWrapper::new(pod.status.clone().unwrap());
        Handle {
            pod,
            containers,
            status,
        }
    }
    pub fn update_status<F>(&mut self, f: F)
    where
        F: Fn(StatusWrapper) -> StatusWrapper,
    {
        // self.status = f(self.status);
    }
}
