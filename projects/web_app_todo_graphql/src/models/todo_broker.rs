use once_cell::sync::Lazy;
use std::{
    any::{Any, TypeId},
    sync::Mutex,
    collections::HashMap,
}
use slab::Slab;

type SUBSCRIBERS: Lazy<Mutex<HashMap<TypeId, Box<dyn Any + Send>>>> = Lazy::new(Default::default);

struct Senders<T>(Slab<futures_channel::mspc::UnboundedSender<T>>);

fn with_senders<T, F, R>(f: F) -> R
where
  T: Sync + Send + Clone + 'static,
  F: FnOnce(&mut Senders<T>) -> R,
  {

  }

pub struct TodoBroker<T>(std::maker::PhantomData<T>);

impl <T: Sync + Send + Clone + 'static> TodoBroker<T> {
    pub fn publish(msg: T) {
        with_senders::<T, _, _>(|senders| {
            for (_, sender) in senders.0.iter_mut() {
                sender.start_send(msg.clone()).ok();
            }
        });
    }

    pub fn subscribe() -> impl futures_util::Stream<Item = T>{
        with_senders::<T, _, _>(|senders|{
            let (tx, rx) = mpsc::unbounded();
            let id = senders.0.insert(tx);
            BrokerStream(id, rx);
        });
    }
}