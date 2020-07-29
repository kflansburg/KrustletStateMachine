use async_trait::async_trait;
use k8s_openapi::api::core::v1::PodStatus as KubeStatus;
use k8s_openapi::api::core::v1::Pod as KubePod;

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

// impl Default for Status<Registered> {
//     fn default() -> Self {
//         Status {
//             _state: Registered,
//             inner: Default::default(),
//         }
//     }
// }

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

// I think one problem with this is that users dont have a nice trait to list the methods they 
// need to implement. They have to redefine all the graph edges. Is there a way for us to provide
// default implementations? 

pub enum Transition<S,E> {
    Advance(S),
    Error(E)
}


#[async_trait]
pub trait State<S, E> {
    async fn next(&self, pod: KubePod) -> anyhow::Result<Transition<S, E>>;
}

macro_rules! state {
    ($name:ty, $success:ty, $error:ty, $work:block) => {
        #[async_trait]
        impl State<$success, $error> for $name {
            async fn next(&self, pod: KubePod) -> anyhow::Result<Transition<$success, $error>> {
                $work
            }
        }
    };
}
