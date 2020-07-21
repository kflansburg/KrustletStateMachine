use k8s_openapi::api::core::v1::ContainerStatus as KubeStatus;

/// Marks valid container states in our graph, not necessarily in Kubernetes spec. 
trait State {}

struct Status<S: State> {
    state: S,
    inner: KubeStatus
}

/// The Kubelet is aware of the container.
struct Registered;
impl State for Registered {}

/// The container image is being pulled.
struct ImagePull;
impl State for ImagePull {}

/// The image pull failed. 
struct ImagePullError;
impl State for ImagePullError {}

/// The image pull failed several times. 
struct ImagePullBackoff;
impl State for ImagePullBackoff {}

/// The volume is being created / mounted.
struct Volume;
impl State for Volume {}

/// The volume creation / mount failed.
struct VolumeError;
impl State for VolumeError {}

/// The container is starting.
struct Starting;
impl State for Starting {}

/// The container is running.
struct Running;
impl State for Running {}

/// The container failed at runtime.
struct Error;
impl State for Error {}

/// The container failed several times.
struct CrashLoopBackoff;
impl State for CrashLoopBackoff {}

/// The container failed several times.
struct Completed;
impl State for Completed {}

/// 
/// Implement outgoing edges for each state.
/// 

impl Status<Registered> {
    /// Clippy doesnt like these to_ names. 
    fn to_image_pull(self) -> Status<ImagePull> {
        Status {
            state: ImagePull,
            inner: self.inner
        }
    }
}

impl Status<ImagePull> {
    fn to_image_pull_error(self) -> Status<ImagePullError> {
        Status {
            state: ImagePullError,
            inner: self.inner
        }
    }
    fn to_volume(self) -> Status<Volume> {
        Status {
            state: Volume,
            inner: self.inner
        }
    }
}

impl Status<ImagePullError> {
    fn to_image_pull(self) -> Status<ImagePull> {
        Status {
            state: ImagePull,
            inner: self.inner
        }
    }
    fn to_image_pull_backoff(self) -> Status<ImagePullBackoff> {
        Status {
            state: ImagePullBackoff,
            inner: self.inner
        }
    }
}

impl Status<ImagePullBackoff> {
    fn to_image_pull(self) -> Status<ImagePull> {
        Status {
            state: ImagePull,
            inner: self.inner
        }
    }
}


impl Status<Volume> {
    fn to_volume_error(self) -> Status<VolumeError> {
        Status {
            state: VolumeError,
            inner: self.inner
        }
    }
    fn to_starting(self) -> Status<Starting> {
        Status {
            state: Starting,
            inner: self.inner
        }
    }
}

impl Status<Running> {
    fn to_error(self) -> Status<Error> {
        Status {
            state: Error,
            inner: self.inner
        }
    }
    fn to_completed(self) -> Status<Completed> {
        Status {
            state: Completed,
            inner: self.inner
        }
    }
}


impl Status<Error> {
    fn to_starting(self) -> Status<Starting> {
        Status {
            state: Starting,
            inner: self.inner
        }
    }
    fn to_crash_loop_backoff(self) -> Status<CrashLoopBackoff> {
        Status {
            state: CrashLoopBackoff,
            inner: self.inner
        }
    }
}

impl Status<CrashLoopBackoff> {
    fn to_starting(self) -> Status<Starting> {
        Status {
            state: Starting,
            inner: self.inner
        }
    }
}
