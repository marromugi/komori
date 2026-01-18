use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick (for periodic updates).
    Tick,
    /// Key press event.
    Key(KeyEvent),
    /// Terminal resize event.
    Resize(u16, u16),
}

/// Handles terminal events asynchronously.
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    _tx: mpsc::UnboundedSender<Event>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate.
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick_delay => {
                        tx.send(Event::Tick).ok();
                    }
                    Some(Ok(event)) = crossterm_event => {
                        match event {
                            CrosstermEvent::Key(key) => {
                                tx.send(Event::Key(key)).ok();
                            }
                            CrosstermEvent::Resize(w, h) => {
                                tx.send(Event::Resize(w, h)).ok();
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self { rx, _tx }
    }

    /// Receive the next event.
    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
