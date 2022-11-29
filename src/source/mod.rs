use async_trait::async_trait;
use crate::Producer;

#[async_trait]
trait Source: Producer<Output=Self::Output> {
    type Output;

    async fn has_items(&mut self) -> bool;
}

struct Stats {
    items_produced: usize,
    
}
struct MaterializedSource<O, T: Source<Output=O>> {
    source: T,
    stats: Stats,
}