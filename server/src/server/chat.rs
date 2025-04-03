use std::{collections::BTreeMap, time::SystemTime};

use tokio::sync::{oneshot, Mutex, RwLock};
use tracing::info;
use vitium_api::net::Chat;

/// Storage for [`Chat`] messages.
pub struct ChatStore {
    /// Message capacity.
    cap: usize,
    /// List of messages, with the latest at the first.
    list: RwLock<BTreeMap<SystemTime, Chat>>,
    /// Subscribers to update.
    watch: Mutex<Vec<oneshot::Sender<Vec<(SystemTime, Chat)>>>>,
}

impl ChatStore {
    /// Create a chat storage with specified capacity.
    pub const fn new(cap: usize) -> Self {
        Self {
            cap,
            list: RwLock::const_new(BTreeMap::new()),
            watch: Mutex::const_new(Vec::new()),
        }
    }

    pub async fn pull(&self, after: SystemTime) -> oneshot::Receiver<Vec<(SystemTime, Chat)>> {
        let (s, r) = oneshot::channel();
        let list = self.list.read().await;
        let res = list
            .range(after..)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        if res.len() > 0 {
            let _ = s.send(res);
        } else {
            let mut watch = self.watch.lock().await;
            watch.push(s);
        }
        r
    }

    pub async fn push(&self, chat: Chat) -> SystemTime {
        info!("<{}> {}", chat.sender, chat.msg);
        let time = SystemTime::now();
        for i in self.watch.lock().await.drain(..) {
            let _ = i.send(vec![(time, chat.clone())]);
        }
        let mut list = self.list.write().await;
        list.insert(time, chat);
        // list.push_front(chat);
        // list.truncate(self.cap);
        time
    }

    pub async fn broadcast(&self, msg: String) {
        let chat = Chat {
            sender: "".into(),
            msg,
        };
        self.push(chat).await;
    }
}
