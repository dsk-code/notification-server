pub mod error;
pub mod message;
pub mod router;

use message::{LineMessageKind, LineSendMessage, LineSender, ScheduledMessage};

use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

#[derive(Debug)]
pub struct State {
    pub line: Arc<LineSender>,
    pub schedule_queue: Arc<Mutex<Vec<ScheduledMessage>>>,
}

impl State {
    pub async fn polling_task(self: Arc<Self>) {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            let now = Utc::now();
            let mut queue = self.schedule_queue.lock().await;
            let (to_send, remaining): (Vec<_>, Vec<_>) =
                queue.drain(..).partition(|msg| msg.send_at <= now);
            queue.extend(remaining);
            drop(queue);

            for msg in to_send {
                let message = LineSendMessage {
                    message: msg.message.clone(),
                };
                if let Err(e) = self.line.send(LineMessageKind::Version1(message)).await {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
