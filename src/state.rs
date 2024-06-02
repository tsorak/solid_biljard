use tokio::sync::broadcast::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct State {
    pub motd: String,
    pub refresh_channel: RefreshChannel,
}

impl State {
    pub fn new(motd: &str) -> Self {
        Self {
            motd: motd.to_string(),
            refresh_channel: RefreshChannel::new(),
        }
    }
}

#[derive(Debug)]
pub struct RefreshChannel(Sender<()>, Receiver<()>);

impl RefreshChannel {
    pub fn new() -> Self {
        let (tx, rx) = channel::<()>(16);
        Self(tx, rx)
    }

    pub async fn recv(&mut self) -> Option<()> {
        self.1.recv().await.ok()
    }

    pub fn send_refresh_request(&self) -> anyhow::Result<()> {
        self.0.send(())?;

        Ok(())
    }
}

impl Clone for RefreshChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.resubscribe())
    }
}
