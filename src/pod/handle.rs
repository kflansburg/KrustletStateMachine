use k8s_openapi::api::core::v1::Pod as KubePod;
use crate::pod::status::Status;
use crate::container::handle::Handle as ContainerHandle;
use std::collections::BTreeMap;

pub struct Handle {
    pod: KubePod,
    containers: BTreeMap<String, ContainerHandle>
}
