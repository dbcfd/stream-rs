mod map;

pub use map::Map;

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
pub trait Operation: Producer<Output=Self::Output> + Consumer {
    type Input;
    type Output;

    async fn transform(&mut self, input: Vec<Self::Input>) -> TransformResult<Self::Input, Self::Output>;
}

pub struct MaterializedOperation<Input, Output, T>
    where S: Producer<Output=Input>,
    T: Operation<Input, Output>,
{
    source: S,
    transform: T,
    stats: Stats,
}