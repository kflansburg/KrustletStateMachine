pub mod handle;
pub mod status;
use k8s_openapi::api::core::v1::Pod as KubePod; 
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::BTreeMap;
use crate::pod::status::StatusWrapper;

type Namespace<'a> = &'a str;
type Name<'a> = &'a str;

#[derive(Clone)]
/// Manages all of our Pod handles, intended to be cloned and shared between tasks. 
pub struct Manager {
    handles: Arc<dashmap::DashMap<(String, String), Arc<Mutex<handle::Handle>>>>
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            handles: Arc::new(dashmap::DashMap::new())
        }
    }

    pub fn register_pod(&mut self, pod: KubePod) {
        let namespace = pod.metadata.as_ref().unwrap().namespace.clone().unwrap();
        let name = pod.metadata.as_ref().unwrap().name.clone().unwrap();
        self.handles.insert((namespace, name), Arc::new(Mutex::new(handle::Handle::new(pod))));
    }

    pub async fn update_status<'a, F>(&self, namespace: Namespace<'a>, name: Name<'a>, f: F) 
        where F: Fn(StatusWrapper) -> StatusWrapper {
        let mutex = self.handles.get(&(namespace.to_string(), name.to_string())).unwrap().clone();
        let mut handle = mutex.lock().await;
        (*handle).update_status(f);
    }
}
