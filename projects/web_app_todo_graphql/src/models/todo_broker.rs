use futures_util::StreamExt;
use once_cell::sync::Lazy;
use slab::Slab;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Mutex,
};

static SUBSCRIBERS: Lazy<Mutex<HashMap<TypeId, Box<dyn Any + Send>>>> = Lazy::new(Default::default);

struct Senders<T>(Slab<futures_channel::mpsc::UnboundedSender<T>>);

struct BrokerStream<T: Sync + Send + Clone + 'static>(
    usize,
    futures_channel::mpsc::UnboundedReceiver<T>,
);

fn with_senders<T, F, R>(f: F) -> R
where
    T: Sync + Send + Clone + 'static,
    F: FnOnce(&mut Senders<T>) -> R,
{
    log::debug!("call with_senders");
    let mut map = SUBSCRIBERS.lock().unwrap();
    let senders = map
        .entry(TypeId::of::<Senders<T>>())
        .or_insert_with(|| Box::new(Senders::<T>(Default::default())));
    f(senders.downcast_mut::<Senders<T>>().unwrap())
}

impl<T: Sync + Send + Clone + 'static> Drop for BrokerStream<T> {
    fn drop(&mut self) {
        log::debug!("BrokerStream::drop()");
        with_senders::<T, _, _>(|senders| senders.0.remove(self.0));
    }
}

impl<T: Sync + Send + Clone + 'static> futures_util::Stream for BrokerStream<T> {
    type Item = T;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        log::debug!("BrokerStream::poll_next()");
        self.1.poll_next_unpin(cx)
    }
}

pub struct TodoBroker<T>(std::marker::PhantomData<T>);

impl<T: Sync + Send + Clone + 'static> TodoBroker<T> {
    pub fn publish(msg: T) {
        log::info!("TodoBroker::publish()");
        with_senders::<T, _, _>(|senders| {
            for (_, sender) in senders.0.iter_mut() {
                sender.start_send(msg.clone()).ok();
            }
        });
    }

    pub fn subscribe() -> impl futures_util::Stream<Item = T> {
        log::info!("TodoBroker::subscribe()");
        with_senders::<T, _, _>(|senders| {
            let (tx, rx) = futures_channel::mpsc::unbounded();
            let id = senders.0.insert(tx);
            BrokerStream(id, rx)
        })
    }
}
