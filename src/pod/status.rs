use k8s_openapi::api::core::v1::PodStatus as KubeStatus;

pub enum StatusWrapper {
    Pending(Status<Pending>),
    Running(Status<Running>),
    Failed(Status<Failed>),
    Succeeded(Status<Succeeded>),
}

impl StatusWrapper {
    pub fn new(inner: KubeStatus) -> Self {
        StatusWrapper::Pending(Status {
            inner,
            state: Pending,
        })
    }
}

pub struct Status<S> {
    state: S,
    inner: KubeStatus,
}

/// The Kubelet is aware of the pod.
/// The Pod is being provisioned.
#[derive(Default)]
pub struct Pending;

/// The Pod is running.
#[derive(Default)]
pub struct Running;

/// The Pod run failed.
#[derive(Default)]
pub struct Failed;

/// The Pod run failed.
#[derive(Default)]
pub struct Succeeded;

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

edge!(Pending, Running);
edge!(Pending, Failed);
edge!(Running, Failed);
edge!(Running, Succeeded);
