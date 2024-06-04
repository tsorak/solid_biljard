use tokio::sync::broadcast::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct State {
    pub motd: String,
    pub refresh_channel: ClientChannel,
}

impl State {
    pub fn new(motd: &str) -> Self {
        Self {
            motd: motd.to_string(),
            refresh_channel: ClientChannel::new(),
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

    pub fn send_refresh_request(&self) -> anyhow::Result<()> {
        self.0.send(ClientChannelMessage::Refresh)?;

        Ok(())
    }
}

impl Clone for ClientChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.resubscribe())
    }
}
