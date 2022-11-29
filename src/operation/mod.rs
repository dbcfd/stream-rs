use async_trait::async_trait;
use crate::{Consumer, Producer};

pub struct TransformFailure<T> {
    item: T,
    reason: Box<dyn std::error::Error>,
}

pub struct TransformResult<Input, Output> {
    success: Vec<Output>,
    error: Vec<TransformFailure<Input>>,
}

#[async_trait]
trait Operation: Producer<Output=Self::Output> + Consumer {
    type Source<Input>: Producer<Output=Input>;
    type Output;

    async fn transform(&mut self, input: Vec<Self::Source::Output>) -> TransformResult<Self::Source::Output, Self::Output>;
}