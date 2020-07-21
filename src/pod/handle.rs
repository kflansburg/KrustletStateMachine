use crate::container::handle::Handle as ContainerHandle;
use crate::pod::status::StatusWrapper;
use k8s_openapi::api::core::v1::Pod as KubePod;
use std::collections::BTreeMap;

pub struct Handle {
    pod: KubePod,
    containers: BTreeMap<String, ContainerHandle>,
    status: StatusWrapper,
}
