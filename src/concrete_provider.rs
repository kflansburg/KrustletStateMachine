use crate::provider::Provider;
use async_trait::async_trait;
use either::Either;

//
// This is all provider would implement unless they wish to override the state graph.
//

pub struct ProviderTest;

#[async_trait]
impl Provider for ProviderTest {
    async fn image_pull(&mut self, _image: String) -> Result<Either<(), ()>, ()> {
        unimplemented!()
    }
    async fn volume_mount(
        &mut self,
        _volume_mount: &k8s_openapi::api::core::v1::VolumeMount,
    ) -> Result<Either<(), ()>, ()> {
        unimplemented!()
    }
}
