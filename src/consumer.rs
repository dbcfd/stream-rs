use anyhow::Result;
use async_trait::async_trait;
use crate::Demand;

#[async_trait]
pub trait Consumer {
    type Input;

    pub async fn poll(&mut self) -> Demand;
}