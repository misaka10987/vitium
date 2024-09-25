use std::{
    collections::VecDeque,
    time::SystemTime,
};

use tokio::sync::{oneshot, Mutex, RwLock};
use vitium_api::net::Chat;

pub struct ChatSto {
    cap: usize,
    list: RwLock<VecDeque<Chat>>,
    watch: Mutex<Vec<oneshot::Sender<Vec<Chat>>>>,
}

impl ChatSto {
    pub const fn new(cap: usize) -> Self {
        Self {
            cap,
            list: RwLock::const_new(VecDeque::new()),
            watch: Mutex::const_new(Vec::new()),
        }
    }

    pub async fn pull(&self, after: SystemTime) -> oneshot::Receiver<Vec<Chat>> {
        let (s, r) = oneshot::channel();
        let list = self.list.read().await;
        let res = list
            .iter()
            .filter(|x| x.send_time >= after)
            .cloned()
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
        let chat = chat.received();
        let t = chat.recv_time;
        for i in self.watch.lock().await.drain(..) {
            let _ = i.send(vec![chat.clone()]);
        }
        let mut list = self.list.write().await;
        list.push_front(chat);
        list.truncate(self.cap);
        t
    }
}
