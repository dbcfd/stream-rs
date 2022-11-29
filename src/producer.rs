use async_trait::async_trait;

#[async_trait]
trait Producer {
    type Output;

    async fn produce(&mut self) -> Vec<Output>;
}