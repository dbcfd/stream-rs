mod collect;
mod sum;

pub mod sinks {
    use super::*;

    pub use collect::Collect;
    pub use sum::Sum;
}

use anyhow::Result;
use async_trait::async_trait;
use crate::{Demand, Producer, Consumer};

#[async_trait]
pub trait Sink: Consumer {
    type Input;

    async fn consume(&mut self, items: Vec<Self::Input>) -> Result<Vec<Self::Input>>;
}