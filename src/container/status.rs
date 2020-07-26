use k8s_openapi::api::core::v1::Container as KubeContainer;
use k8s_openapi::api::core::v1::ContainerStatus as KubeStatus;

/// This allows us to store the variable-sized Status. The user would probably replace this type when implementing custome states.
pub enum StatusWrapper {
    Waiting(Status<Waiting>),
    Running(Status<Running>),
    Terminated(Status<Terminated>),
}

impl StatusWrapper {
    pub fn new(container: &KubeContainer) -> Self {
        let inner = Default::default();
        StatusWrapper::Waiting(Status {
            state: Waiting,
            inner,
        })
    }
}

pub struct Status<S> {
    state: S,
    inner: KubeStatus,
}

/// The Kubelet is aware of the container.
#[derive(Default)]
pub struct Waiting;

/// The container is running.
#[derive(Default)]
pub struct Running;

/// The container has exited.
#[derive(Default)]
pub struct Terminated;

//
// Implement outgoing edges for each state.
//

macro_rules! edge {
    ($start:ty,$end:ty) => {
        impl From<Status<$start>> for Status<$end> {
            fn from(start: Status<$start>) -> Status<$end> {
                Status {
                    inner: start.inner,
                    state: <$end as Default>::default(),
                }
            }
        }
    };
}

edge!(Waiting, Running);
edge!(Running, Terminated);
