use crate::state::State;

mod build;
mod watch;

pub struct Client {
    watcher: Option<watch::ClientWatcher>,
    builder: build::Builder,
}

impl Client {
    pub async fn init(watch_dir: &str, state: State) -> Self {
        let mut builder = build::Builder::new(state.clone()).await;
        builder.init();

        let watcher = watch::ClientWatcher::new(watch_dir, state.clone());

        Self {
            watcher: Some(watcher),
            builder,
        }
    }

    pub fn take_watcher(&mut self) -> Option<watch::ClientWatcher> {
        self.watcher.take()
    }
}
