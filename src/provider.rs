use async_trait::async_trait;
use either::Either;

#[async_trait]
pub trait Provider: Send + Sync {
    //
    // Thes are methods the provider will need to implement.
    // TODO Would be nice to have something better than units returned in the Either.
    //
    async fn image_pull(&mut self, image: String) -> Result<Either<(), ()>, ()>;
    async fn volume_mount(
        &mut self,
        volume_mount: &k8s_openapi::api::core::v1::VolumeMount,
    ) -> Result<Either<(), ()>, ()>;

    //
    // We can provide sane defaults for these methods.
    //
    async fn image_pull_backoff(&mut self) -> Result<(), ()> {
        tokio::time::delay_for(std::time::Duration::from_secs(30)).await;
        Ok(())
    }

    async fn volume_mount_backoff(&mut self) -> Result<(), ()> {
        tokio::time::delay_for(std::time::Duration::from_secs(30)).await;
        Ok(())
    }
}
