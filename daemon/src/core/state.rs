use arc_swap::ArcSwap;
use std::sync::Arc;
use tokio::sync::watch;
use crate::core::traits::StateStore;

pub struct ArcStateStore<T: Clone + Send + Sync + 'static> {
    inner: ArcSwap<T>,
    watch_tx: watch::Sender<T>,
}

impl<T: Clone + Send + Sync + 'static> ArcStateStore<T> {
    pub fn new(initial: T) -> (Self, watch::Receiver<T>) {
        let (watch_tx, watch_rx) = watch::channel(initial.clone());
        let store = Self {
            inner: ArcSwap::from_pointee(initial),
            watch_tx,
        };
        (store, watch_rx)
    }
}

impl<T: Clone + Send + Sync + 'static> StateStore<T> for ArcStateStore<T> {
    fn get(&self) -> T {
        self.inner.load().as_ref().clone()
    }

    fn set(&self, value: T) {
        self.inner.store(Arc::new(value.clone()));
        let _ = self.watch_tx.send(value);
    }

    fn watch(&self) -> watch::Receiver<T> {
        self.watch_tx.subscribe()
    }
}
