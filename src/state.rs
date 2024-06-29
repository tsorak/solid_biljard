use tokio::sync::broadcast::{channel, Receiver, Sender};

use crate::{api::types::email_code, db::DB, password_hash::PasswordHasher};

#[derive(Debug, Clone)]
pub struct State {
    pub client_channel: ClientChannel,
    pub db: DB,
    pub email_code_session: email_code::session::CodeSession,
    pub password_hasher: PasswordHasher,
}

impl State {
    pub async fn new() -> Self {
        // use sqlite unless told not to
        if cfg!(feature = "postgres") || !cfg!(feature = "sqlite") {
            let postgres_db = DB::new_postgres().await;
            Self::new_with_db(postgres_db).await
        } else {
            let sqlite_db = DB::new_sqlite().await;
            Self::new_with_db(sqlite_db).await
        }
    }

    async fn new_with_db(db: DB) -> Self {
        Self {
            client_channel: ClientChannel::new(),
            db,
            email_code_session: email_code::session::CodeSession::new(),
            password_hasher: PasswordHasher::init("key.pem").await,
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
