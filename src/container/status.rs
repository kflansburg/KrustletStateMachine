use k8s_openapi::api::core::v1::ContainerStatus as KubeStatus;

/// Marks valid container states in our graph, not necessarily in Kubernetes spec.
pub trait State {}

pub struct Status<S: State> {
    state: S,
    inner: KubeStatus,
}

/// The Kubelet is aware of the container.
pub struct Registered;
impl State for Registered {}

/// The container image is being pulled.
pub struct ImagePull;
impl State for ImagePull {}

/// The image pull failed.
pub struct ImagePullError;
impl State for ImagePullError {}

/// The image pull failed several times.
pub struct ImagePullBackoff;
impl State for ImagePullBackoff {}

/// The volume is being created / mounted.
pub struct Volume;
impl State for Volume {}

/// The volume creation / mount failed.
pub struct VolumeError;
impl State for VolumeError {}

/// The container is starting.
pub struct Starting;
impl State for Starting {}

/// The container is running.
pub struct Running;
impl State for Running {}

/// The container failed at runtime.
pub struct Error;
impl State for Error {}

/// The container failed several times.
pub struct CrashLoopBackoff;
impl State for CrashLoopBackoff {}

/// The container failed several times.
pub struct Completed;
impl State for Completed {}

///
/// Implement outgoing edges for each state.
///

impl From<Status<Registered>> for Status<ImagePull> {
    fn from(status: Status<Registered>) -> Status<ImagePull> {
        Status {
            state: ImagePull,
            inner: status.inner
        }
    }
}

impl From<Status<ImagePull>> for Status<ImagePullError> {
    fn from(status: Status<ImagePull>) -> Status<ImagePullError> {
        Status {
            state: ImagePullError,
            inner: status.inner
        }
    }
}

impl From<Status<ImagePull>> for Status<Volume> {
    fn from(status: Status<ImagePull>) -> Status<Volume> {
        Status {
            state: Volume,
            inner: status.inner
        }
    }
}

impl From<Status<ImagePullError>> for Status<ImagePull> {
    fn from(status: Status<ImagePullError>) -> Status<ImagePull> {
        Status {
            state: ImagePull,
            inner: status.inner
        }
    }
}

impl From<Status<ImagePullError>> for Status<ImagePullBackoff> {
    fn from(status: Status<ImagePullError>) -> Status<ImagePullBackoff> {
        Status {
            state: ImagePullBackoff,
            inner: status.inner
        }
    }
}

impl From<Status<ImagePullBackoff>> for Status<ImagePull> {
    fn from(status: Status<ImagePullBackoff>) -> Status<ImagePull> {
        Status {
            state: ImagePull,
            inner: status.inner
        }
    }
}

impl From<Status<Volume>> for Status<VolumeError> {
    fn from(status: Status<Volume>) -> Status<VolumeError> {
        Status {
            state: VolumeError,
            inner: status.inner
        }
    }
}

impl From<Status<Volume>> for Status<Starting> {
    fn from(status: Status<Volume>) -> Status<Starting> {
        Status {
            state: Starting,
            inner: status.inner
        }
    }
}

impl From<Status<Running>> for Status<Error> {
    fn from(status: Status<Running>) -> Status<Error> {
        Status {
            state: Error,
            inner: status.inner
        }
    }
}

impl From<Status<Running>> for Status<Completed> {
    fn from(status: Status<Running>) -> Status<Completed> {
        Status {
            state: Completed,
            inner: status.inner
        }
    }
}

impl From<Status<Error>> for Status<Starting> {
    fn from(status: Status<Error>) -> Status<Starting> {
        Status {
            state: Starting,
            inner: status.inner
        }
    }
}

impl From<Status<Error>> for Status<CrashLoopBackoff> {
    fn from(status: Status<Error>) -> Status<CrashLoopBackoff> {
        Status {
            state: CrashLoopBackoff,
            inner: status.inner
        }
    }
}

impl From<Status<CrashLoopBackoff>> for Status<Starting> {
    fn from(status: Status<CrashLoopBackoff>) -> Status<Starting> {
        Status {
            state: Starting,
            inner: status.inner
        }
    }
}
