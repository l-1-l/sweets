use std::sync::Arc;

use async_trait::async_trait;
use regex::Regex;
use tokio::sync::oneshot::{self, Receiver};
use tokio::sync::Mutex;

use crate::error::Error;
use crate::event::{Event, EventHandler, EventPublisher, EventSubscriber, PublicationResult};
use crate::result::Result;

#[derive(Default)]
pub struct InMemEventBus {
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler>>>>,
}

impl InMemEventBus {
    pub fn new() -> Self {
        InMemEventBus {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl EventPublisher for InMemEventBus {
    async fn publish(&self, event: Event) -> Result<Receiver<PublicationResult>> {
        self.publish_all(vec![event]).await
    }

    async fn publish_all(&self, events: Vec<Event>) -> Result<Receiver<PublicationResult>> {
        let handlers = Arc::clone(&self.handlers);
        let (tx, rx) = oneshot::channel();
        let mut publication_result = PublicationResult::default();

        tokio::spawn(async move {
            for event in events.into_iter() {
                let mut handlers = handlers.lock().await;
                for handler in handlers.iter_mut() {
                    match Regex::new(handler.topic()) {
                        Ok(re) => {
                            if re.is_match(event.topic()) {
                                // Execute handler
                                if let Err(err) = handler.handle(&event).await {
                                    let err = Error::internal("event_publisher", "handler_error")
                                        .wrap(err);
                                    println!("{:?}", err);

                                    publication_result.err_handlers += 1;
                                } else {
                                    publication_result.ok_handlers += 1;
                                }
                            }
                        }
                        Err(err) => {
                            let err = Error::internal("event_publisher", "invalid_topic_regex")
                                .wrap_raw(err);
                            println!("{:?}", err);
                        }
                    }
                }

                publication_result.published_events += 1;
            }

            if tx.send(publication_result).is_err() {}
        });

        Ok(rx)
    }
}

#[async_trait]
impl EventSubscriber for InMemEventBus {
    async fn subscribe(&self, handler: Box<dyn EventHandler>) -> Result<bool> {
        let mut handlers = self.handlers.lock().await;
        handlers.push(handler);
        Ok(true)
    }
}
