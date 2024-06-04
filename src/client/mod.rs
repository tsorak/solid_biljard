use crate::state::State;

mod build;
mod watch;

pub struct Client {
    watcher: Option<watch::ClientWatcher>,
    builder: build::Builder,
}

impl Client {
    pub fn new(watch_dir: &str, state: State) -> Self {
        let builder = build::Builder::new(state.clone());
        let watcher = watch::ClientWatcher::new(watch_dir, state.clone());

        Self {
            watcher: Some(watcher),
            builder,
        }
    }

    pub async fn init(&mut self) -> &Self {
        self.builder.init().await;
        self
    }

    pub fn take_watcher(&mut self) -> Option<watch::ClientWatcher> {
        self.watcher.take()
    }
}
