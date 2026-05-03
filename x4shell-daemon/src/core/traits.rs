use async_trait::async_trait;
use tokio::sync::broadcast;
use crate::core::events::Event;

pub type EventChannel = broadcast::Sender<Event>;

#[async_trait]
pub trait Service: Send + Sync + 'static {
    async fn start(&mut self, events: EventChannel) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait StateStore<T: Clone + Send + Sync>: Send + Sync {
    fn get(&self) -> T;
    fn set(&self, value: T);
    fn watch(&self) -> tokio::sync::watch::Receiver<T>;
}
