use async_trait::async_trait;
use k8s_openapi::api::core::v1::Pod as KubePod;
use k8s_openapi::api::core::v1::PodStatus as KubeStatus;

#[derive(Debug)]
pub struct Status<S: State> {
    state: S,
    inner: KubeStatus,
}

// Placeholder config stub
pub struct Config;

pub enum Transition<S, E> {
    Advance(S),
    Error(E),
    Completed(anyhow::Result<()>)
}

#[async_trait]
pub trait State: Sync + Send + 'static {
    type Success: State;
    type Error: State;

    async fn next(self, pod: KubePod) -> anyhow::Result<Transition<Self::Success, Self::Error>>;
}

#[async_recursion::async_recursion]
pub async fn run(state: impl State, pod: KubePod) -> anyhow::Result<()> {
    match state.next(pod.clone()).await? {
        Transition::Advance(s) => run(s, pod).await,
        Transition::Error(s) => run(s, pod).await,
        Transition::Completed(result) => result
    }
}

macro_rules! state {
    (
        $(#[$meta:meta])*
        $name:ident,
        $success:ty,
        $error: ty,
        $work:block
    ) => {
        $(#[$meta])*
        #[derive(Default, Debug)]
        pub struct $name;

       
        #[async_trait]
        impl State for $name {
            type Success = $success;
            type Error = $error;
           async fn next(self, #[allow(unused_variables)] pod: KubePod) -> anyhow::Result<Transition<Self::Success, Self::Error>> {
                #[allow(unused_braces)]
                $work
            }
        }
    };

    (
        $(#[$meta:meta])*
        $name:ident,
        $success:ty,
        $error: ty,
        $work:path
    ) => {
        $(#[$meta])*
        #[derive(Default, Debug)]
        pub struct $name;

        #[async_trait]
        impl State for $name {
            type Success = $success;
            type Error = $error;
           async fn next(self, #[allow(unused_variables)] pod: KubePod) -> anyhow::Result<Transition<Self::Success, Self::Error>> {
                $work(self, pod).await
            }
        }
    };
}


state!(Registered, ImagePull, Failed, {
    println!("{:?} -> {:?}", self, ImagePull);
    Ok(Transition::Advance(ImagePull))
});

state!(ImagePull, Starting, ImagePullBackoff, {
    println!("{:?} -> {:?}", self, Starting);
    Ok(Transition::Advance(Starting))
});

state!(ImagePullBackoff, ImagePull, ImagePullBackoff, {
    println!("{:?} -> {:?}", self, ImagePull);
    Ok(Transition::Advance(ImagePull))
});

async fn failed(self_ref: Failed, _pod: KubePod) -> anyhow::Result<Transition<Failed, Failed>> {
    println!("{:?}", self_ref);
    Ok(Transition::Completed(Err(anyhow::anyhow!("Failed."))))
    
}

state!(Failed, Failed, Failed, failed);

state!(Starting, Running, Failed, {
    println!("{:?} -> {:?}", self, Running);
    Ok(Transition::Advance(Running))
});

state!(Running, Completed, Failed, {
    println!("{:?} -> {:?}", self, Completed);
    Ok(Transition::Advance(Completed))
});

state!(Completed, Completed, Completed, {
    println!("{:?}", self);
    Ok(Transition::Completed(Ok(())))
});



mod test {
    use super::*;

    #[tokio::test]
    async fn run_next() {
        let pod = Default::default();
        let state = Registered;
        let result = run(state, pod).await;
        println!("{:?}", result); 
    }
}
