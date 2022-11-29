use anyhow::Result;
use async_trait::async_trait;
use crate::{Demand, Producer, Consumer};

#[async_trait]
trait Sink: Consumer {
    type Source<Input>: Producer<Output=Input>;

    async fn consume(&mut self, items: Vec<Self::Source::Output>) -> Result<Vec<Self::Source::Output>>;
}