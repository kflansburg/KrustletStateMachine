use async_trait::async_trait;
use k8s_openapi::api::core::v1::PodStatus as KubeStatus;

pub struct Status<S> {
    _state: S,
    inner: KubeStatus,
}

// Required to implement edge traits.
pub trait StatusTrait: std::marker::Sized {
    fn into_inner(self) -> KubeStatus;
}

impl<T> StatusTrait for Status<T> {
    fn into_inner(self) -> KubeStatus {
        self.inner
    }
}

impl Default for Status<Registered> {
    fn default() -> Self {
        Status {
            _state: Registered,
            inner: Default::default(),
        }
    }
}

macro_rules! node {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $name;
    };
}

// Now that I think about it, these methods should probably be async so that
// we can make the call to Kubernetes, as well as have arguments for additional
// context such as error messages. This may require a trait for each transition.

#[async_trait]
pub trait ToError: StatusTrait {
    async fn to_error(self, message: &str) -> Status<Error> {
        let mut state = self.into_inner();

        state.message = Some(message.to_string());
        state.phase = Some("Failed".to_string());

        // TODO notify Kubernetes.

        Status {
            inner: state,
            _state: Error,
        }
    }
}

macro_rules! edge {
    ($start:ty,$end:ty) => {
        impl From<Status<$start>> for Status<$end> {
            fn from(start: Status<$start>) -> Status<$end> {
                Status {
                    inner: start.inner,
                    _state: <$end as Default>::default(),
                }
            }
        }
    };
}

node!(
    /// The Kubelet is aware of the Pod.
    Registered
);

node!(
    /// A container image is being pulled.
    ImagePull
);

node!(
    /// A container image has failed several times.
    ImagePullBackoff
);

node!(
    /// A container volume is being provisioned.
    VolumeMount
);

node!(
    /// A container volume has failed several times.
    VolumeMountBackoff
);

node!(
    /// The Pod is starting.
    Starting
);

node!(
    /// The Pod is running.
    Running
);

node!(
    /// Pod execution failed.
    Error
);

node!(
    /// The Pod has failed several times.
    CrashLoopBackoff
);

node!(
    /// The Pod exited without error.
    Completed
);

impl ToError for Status<VolumeMount> {}
impl ToError for Status<ImagePull> {}

// So the will probably be replaced with traits like above.
edge!(Registered, ImagePull);

edge!(ImagePull, ImagePullBackoff);
edge!(ImagePull, VolumeMount);

edge!(ImagePullBackoff, ImagePull);

edge!(VolumeMount, VolumeMountBackoff);
edge!(VolumeMount, Starting);

edge!(VolumeMountBackoff, VolumeMount);

edge!(Starting, Running);

edge!(Running, Error);
edge!(Running, Completed);

edge!(Error, CrashLoopBackoff);
edge!(Error, Starting);

edge!(CrashLoopBackoff, Starting);
