pub mod handle;
pub mod status;
use k8s_openapi::api::core::v1::Pod as KubePod; 
use crate::pod::status::Wrapper as PodStatusWrapper;
use crate::container::status::Wrapper as ContainerStatusWrapper;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::BTreeMap;

type Namespace<'a> = &'a str;
type Name<'a> = &'a str;

#[derive(Clone)]
/// Manages all of our Pod handles, intended to be cloned and shared between tasks. 
pub struct Manager<P, C> {
    handles: Arc<dashmap::DashMap<(String, String), Arc<Mutex<handle::Handle<P, C>>>>>
}

impl <P, C> Manager<P, C> {
    pub fn new() -> Self {
        Manager {
            handles: Arc::new(dashmap::DashMap::new())
        }
    }

    pub fn register_pod(&mut self, pod: KubePod) 
        where P: PodStatusWrapper, C: ContainerStatusWrapper {
        let namespace = pod.metadata.as_ref().unwrap().namespace.clone().unwrap();
        let name = pod.metadata.as_ref().unwrap().name.clone().unwrap();
        self.handles.insert((namespace, name), Arc::new(Mutex::new(handle::Handle::new(pod))));
    }

    pub async fn update_status<'a, F>(&self, namespace: Namespace<'a>, name: Name<'a>, f: F) 
        where P: PodStatusWrapper, F: Fn(P) -> P {
        let mutex = self.handles.get(&(namespace.to_string(), name.to_string())).unwrap().clone();
        let mut handle = mutex.lock().await;
        (*handle).update_status(f);
    }
}
