use async_trait::async_trait;
use k8s_openapi::api::core::v1::Pod as KubePod;
use k8s_openapi::api::core::v1::PodStatus as KubeStatus;

#[derive(Debug)]
pub struct Status<S: Work> {
    state: S,
    inner: KubeStatus,
}

// TODO: A nice top level API like this
// graph!(
//     ImagePull => {Ok(())},
//     Volumes => another_func,
//     // etc...
// );

// Placeholder config stub
pub struct Config;

#[async_trait]
pub trait Work {
    // TODO: not sure if we actually need a generic config type, but putting it here in case
    fn new<C>(config: C) -> Self;
    async fn work(&self, pod: KubePod) -> anyhow::Result<()>;
}

// Required to implement edge traits.
// pub trait StatusTrait: std::marker::Sized {
//     fn into_inner(self) -> KubeStatus;
// }

// impl<T> StatusTrait for Status<T> {
//     fn into_inner(self) -> KubeStatus {
//         self.inner
//     }
// }

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
        $name:ident,
        $work:block
    ) => {
        $(#[$meta])*
        #[derive(Default, Debug)]
        pub struct $name;

        #[async_trait]
        impl Work for $name {
            fn new<C>(_config: C) -> Self {
                $name
            }

            async fn work(&self, #[allow(unused_variables)] pod: KubePod) -> anyhow::Result<()> {
                #[allow(unused_braces)]
                $work
            }
        }
    };

    (
        $(#[$meta:meta])*
        $name:ident,
        $work:path
    ) => {
        $(#[$meta])*
        #[derive(Default, Debug)]
        pub struct $name;

        #[async_trait]
        impl Work for $name {
            fn new<C>(_config: C) -> Self {
                $name
            }

            async fn work(&self, #[allow(unused_variables)] pod: KubePod) -> anyhow::Result<()> {
                $work(self, pod).await
            }
        }
    };
}

node!(
    /// The Kubelet is aware of the Pod.
    Registered,
    { Ok(()) }
);

node!(
    /// A container image is being pulled.
    ImagePull,
    { Ok(()) }
);

node!(
    /// A container image has failed several times.
    ImagePullBackoff,
    pull_backoff
);

node!(
    /// A container volume is being provisioned.
    VolumeMount,
    { Ok(()) }
);

node!(
    /// A container volume has failed several times.
    VolumeMountBackoff,
    { Ok(()) }
);

node!(
    /// The Pod is starting.
    Starting,
    { Ok(()) }
);

node!(
    /// The Pod is running.
    Running,
    { Ok(()) }
);

node!(
    /// Pod execution failed.
    RunError,
    { Ok(()) }
);

node!(
    /// The Pod has failed several times.
    CrashLoopBackoff,
    { Ok(()) }
);

node!(
    /// The Pod exited without error.
    Completed,
    { Ok(()) }
);

// I think one problem with this is that users dont have a nice trait to list the methods they
// need to implement. They have to redefine all the graph edges. Is there a way for us to provide
// default implementations?

pub enum Transition<S, E> {
    Advance(S),
    Error(E),
}

// #[async_trait]
// pub trait State<S, E>
// where
//     S: State<S, E>,
//     E: State<S, E>,
// {
//     async fn next(&self, pod: KubePod) -> anyhow::Result<Transition<S, E>>;
// }

#[async_trait]
pub trait State {
    type Success: Work + Send + Sync;
    type Error: Work + Send + Sync;

    async fn next(
        self,
        pod: KubePod,
    ) -> anyhow::Result<Transition<Status<Self::Success>, Status<Self::Error>>>
    where
        Status<Self::Success>: State,
        Status<Self::Error>: State;
}

use Transition::Advance;
use Transition::Error;

macro_rules! state {
    ($name:ty, $success:ty, $error:ty) => {
        #[async_trait]
        impl State for Status<$name> {
            type Success = $success;
            type Error = $error;
            async fn next(
                self,
                pod: KubePod,
            ) -> anyhow::Result<Transition<Status<Self::Success>, Status<Self::Error>>> {
                Ok(match self.state.work(pod).await {
                    Ok(_) => Advance(Status {
                        state: <$success>::new(Config),
                        inner: self.inner,
                    }),
                    Err(_) => Error(Status {
                        state: <$error>::new(Config),
                        inner: self.inner,
                    }),
                })
            }
        }
    };
}

state!(Registered, ImagePull, ImagePull);
state!(ImagePull, VolumeMount, ImagePullBackoff);
state!(ImagePullBackoff, ImagePull, ImagePullBackoff);

async fn pull_backoff(_self_ref: &ImagePullBackoff, _pod: KubePod) -> anyhow::Result<()> {
    Ok(())
}

async fn walk<S: Work>(state: Status<S>, pod: KubePod) -> anyhow::Result<()>
where
    Status<S>: State,
{
    match state.next(pod.clone()).await.unwrap() {
        Advance(s) => walk(s, pod).await,
        Error(s) => walk(s, pod).await,
    }
}

// mod test {
//     use super::*;

//     #[tokio::test]
//     async fn run_next() {
//         let work = Registered::new(Config);
//         let state = Status {
//             state: work,
//             inner: KubeStatus::default(),
//         };
//         let pod = KubePod::default();
//         let state: Box<dyn State> = match state.next(pod.clone()).await.unwrap() {
//             Advance(s) => Box::new(s as State),
//             Error(s) => Box::new(s as State),
//         };
//         println!("{:?}", state);
//         let state = match state.next(pod.clone()).await.unwrap() {
//             Advance(s) | Error(s) => s,
//         };
//         println!("{:?}", state);
//     }
// }
