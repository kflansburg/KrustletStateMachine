use k8s_openapi::api::core::v1::PodStatus as KubeStatus;

/// Marks valid pod states in our graph, not necessarily in Kubernetes spec.
trait State {}

struct Status<S: State> {
    state: S,
    inner: KubeStatus,
}

/// The Kubelet is aware of the pod.
struct Registered;
impl State for Registered {}

/// The Pod is being provisioned.
struct Pending;
impl State for Pending {}

/// The Pod is running.
struct Running;
impl State for Running {}

/// The Pod run failed.
struct Error;
impl State for Error {}

/// The Pod has failed several times.
struct CrashLoopBackoff;
impl State for CrashLoopBackoff {}

/// The Pod exited without error.
struct Completed;
impl State for Completed {}

impl Status<Registered> {
    fn into_pending(self) -> Status<Pending> {
        Status {
            state: Pending,
            inner: self.inner
        }
    }
}

impl Status<Pending> {
    fn into_running(self) -> Status<Running> {
        Status {
            state: Running,
            inner: self.inner
        }
    }
    fn into_error(self) -> Status<Error> {
        Status {
            state: Error,
            inner: self.inner
        }
    }
}

impl Status<Running> {
    fn into_error(self) -> Status<Error> {
        Status {
            state: Error,
            inner: self.inner
        }
    }
    fn into_completed(self) -> Status<Completed> {
        Status {
            state: Completed,
            inner: self.inner
        }
    }
}

impl Status<CrashLoopBackoff> {
    fn into_error(self) -> Status<Error> {
        Status {
            state: Error,
            inner: self.inner
        }
    }
}

impl Status<Error> {
    fn into_pending(self) -> Status<Pending> {
        Status {
            state: Pending,
            inner: self.inner
        }
    }
    fn into_crash_loop_backoff(self) -> Status<CrashLoopBackoff> {
        Status {
            state: CrashLoopBackoff,
            inner: self.inner
        }
    }
}
