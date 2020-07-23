use crate::container::handle::Handle as ContainerHandle;
use crate::pod::status::Wrapper as PodStatusWrapper;
use crate::container::status::Wrapper as ContainerStatusWrapper;
use k8s_openapi::api::core::v1::Pod as KubePod;
use std::collections::BTreeMap;

pub struct Handle<P, C> {
    pod: KubePod,
    containers: BTreeMap<String, ContainerHandle<C>>,
    status: P,
}

impl <P, C> Handle<P, C> {
    pub fn new(pod: KubePod) -> Self 
        where P: PodStatusWrapper, C: ContainerStatusWrapper {
        let mut containers = BTreeMap::new();
        for container in &pod.spec.as_ref().unwrap().containers {
            containers.insert(container.name.clone(), ContainerHandle::new(container.clone()));
        }
     
        let status = P::new(pod.status.clone().unwrap());
        Handle {
            pod,
            containers,
            status,
        }
    }
    pub fn update_status<F>(&mut self, f: F) 
        where P: PodStatusWrapper, F: Fn(P) -> P {
        // self.status = f(self.status);
    }
}
