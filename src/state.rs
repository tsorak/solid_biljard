use tokio::sync::broadcast::{channel, Receiver, Sender};

use crate::{api::email_code, db::DB};

#[derive(Debug, Clone)]
pub struct State {
    pub motd: String,
    pub client_channel: ClientChannel,
    pub db: DB,
    pub email_code_session: email_code::session::CodeSession,
}

impl State {
    pub async fn new(motd: &str) -> Self {
        // use sqlite unless told not to
        if cfg!(feature = "postgres") || !cfg!(feature = "sqlite") {
            Self::new_with_db(DB::new_postgres().await, motd)
        } else {
            Self::new_with_db(DB::new_sqlite().await, motd)
        }
    }

    fn new_with_db(db: DB, motd: &str) -> Self {
        Self {
            motd: motd.to_string(),
            client_channel: ClientChannel::new(),
            db,
            email_code_session: email_code::session::CodeSession::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClientChannelMessage {
    Refresh,
    Build,
}

#[derive(Debug)]
pub struct ClientChannel(Sender<ClientChannelMessage>, Receiver<ClientChannelMessage>);

impl ClientChannel {
    pub fn new() -> Self {
        let (tx, rx) = channel::<ClientChannelMessage>(16);
        Self(tx, rx)
    }

    pub async fn recv(&mut self) -> Option<ClientChannelMessage> {
        self.1.recv().await.ok()
    }

    pub fn send_rebuild(&self) -> &Self {
        let _ = self.0.send(ClientChannelMessage::Build);
        self
    }

    pub fn send_refresh(&self) -> &Self {
        let _ = self.0.send(ClientChannelMessage::Refresh);
        self
    }
}

impl Clone for ClientChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.resubscribe())
    }
}
