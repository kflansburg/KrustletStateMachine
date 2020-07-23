use k8s_openapi::api::core::v1::PodStatus as KubeStatus;

pub trait Wrapper {
    fn new(inner: KubeStatus) -> Self;

    /// Transitions that must be supported outside of provider need to be implemented to satisfy trait.
    fn to_error(self, msg: &str) -> Self;
}

pub enum StatusWrapper {
    Registered(Status<Registered>),
    Pending(Status<Pending>),
    Running(Status<Running>),
    Error(Status<Error>),
    CrashLoopBackoff(Status<CrashLoopBackoff>),
    Completed(Status<Completed>),
}

impl Wrapper for StatusWrapper {
    fn new(inner: KubeStatus) -> Self {
        StatusWrapper::Registered(Status { inner, state: Registered })
    }

    fn to_error(self, msg: &str) -> Self {
        match self {
            StatusWrapper::Registered(mut status) => {
                status.inner.phase = Some("Failed".to_string());
                status.inner.message = Some(msg.to_string());
                StatusWrapper::Error(status.into())
            },
            _ => unimplemented!()
        }
    }
}

/// Marks valid pod states in our graph, not necessarily in Kubernetes spec.
pub trait State {}

pub struct Status<S: State> {
    state: S,
    inner: KubeStatus,
}

/// The Kubelet is aware of the pod.
pub struct Registered;
impl State for Registered {}

/// The Pod is being provisioned.
pub struct Pending;
impl State for Pending {}

/// The Pod is running.
pub struct Running;
impl State for Running {}

/// The Pod run failed.
pub struct Error;
impl State for Error {}

/// The Pod has failed several times.
pub struct CrashLoopBackoff;
impl State for CrashLoopBackoff {}

/// The Pod exited without error.
pub struct Completed;
impl State for Completed {}

impl From<Status<Registered>> for Status<Pending> {
    fn from(status: Status<Registered>) -> Status<Pending> {
        Status {
            state: Pending,
            inner: status.inner,
        }
    }
}

impl From<Status<Registered>> for Status<Error> {
    fn from(status: Status<Registered>) -> Status<Error> {
        Status {
            state: Error,
            inner: status.inner,
        }
    }
}


impl From<Status<Pending>> for Status<Running> {
    fn from(status: Status<Pending>) -> Status<Running> {
        Status {
            state: Running,
            inner: status.inner,
        }
    }
}

impl From<Status<Pending>> for Status<Error> {
    fn from(status: Status<Pending>) -> Status<Error> {
        Status {
            state: Error,
            inner: status.inner,
        }
    }
}

impl From<Status<Running>> for Status<Completed> {
    fn from(status: Status<Running>) -> Status<Completed> {
        Status {
            state: Completed,
            inner: status.inner,
        }
    }
}

impl From<Status<Running>> for Status<Error> {
    fn from(status: Status<Running>) -> Status<Error> {
        Status {
            state: Error,
            inner: status.inner,
        }
    }
}

impl From<Status<CrashLoopBackoff>> for Status<Error> {
    fn from(status: Status<CrashLoopBackoff>) -> Status<Error> {
        Status {
            state: Error,
            inner: status.inner,
        }
    }
}

impl From<Status<Error>> for Status<Pending> {
    fn from(status: Status<Error>) -> Status<Pending> {
        Status {
            state: Pending,
            inner: status.inner,
        }
    }
}

impl From<Status<Error>> for Status<CrashLoopBackoff> {
    fn from(status: Status<Error>) -> Status<CrashLoopBackoff> {
        Status {
            state: CrashLoopBackoff,
            inner: status.inner,
        }
    }
}
