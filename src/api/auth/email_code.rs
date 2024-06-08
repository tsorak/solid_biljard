use axum::response::Response;

pub async fn new_code() -> Response {}

pub async fn validate_code() -> Response {}

pub mod session {
    use anyhow::Error;
    use tokio::{
        sync::{
            mpsc::{channel, Receiver, Sender},
            oneshot,
        },
        task::AbortHandle,
    };

    type Email = String;

    enum Request {
        NewCode(Email),
        ValidateCode((Email, u16, oneshot::Sender<anyhow::Result<bool>>)),
    }

    pub struct CodeSession {
        tx: Sender<Request>,
        handle: AbortHandle,
    }

    impl CodeSession {
        pub fn new() -> Self {
            let (tx, rx) = channel::<Request>(16);

            let handle = tokio::spawn(async move { session_event_loop(rx).await }).abort_handle();

            Self { tx, handle }
        }

        pub async fn new_code(&self, email: Email) -> anyhow::Result<()> {
            Ok(self.tx.send(Request::NewCode(email)).await?)
        }

        pub async fn validate_code(&self, email: Email, code: u16) -> anyhow::Result<bool> {
            let (resolver, rx) = oneshot::channel::<anyhow::Result<bool>>();

            let _sent = self
                .tx
                .send(Request::ValidateCode((email, code, resolver)))
                .await?;

            let result = rx.await?;
            result
        }
    }

    impl Drop for CodeSession {
        fn drop(&mut self) {
            self.handle.abort();
        }
    }

    async fn session_event_loop(mut rx: Receiver<Request>) {
        let mut state = code_store::CodeStore::new();

        loop {
            if let Some(m) = rx.recv().await {
                match m {
                    Request::NewCode(email) => {
                        let code = 0000;

                        state.insert(email, code);
                    }
                    Request::ValidateCode((email, code, response_channel)) => {
                        let correct_code = match state.get(&email) {
                            Some(v) => v,
                            None => {
                                let _ = response_channel
                                    .send(Err(Error::msg("No active session for that email")));
                                continue;
                            }
                        };

                        if code == correct_code {
                            let _ = response_channel.send(Ok(true));
                            state.remove(email);
                        } else {
                            let _ = response_channel.send(Ok(false));
                        }
                    }
                }
            }
        }
    }

    mod code_store {
        use std::collections::HashMap;

        use super::Email;

        pub struct CodeStore(HashMap<Email, u16>);

        impl CodeStore {
            pub fn new() -> Self {
                Self(HashMap::new())
            }

            pub fn insert(&mut self, email: Email, code: u16) {
                self.0.insert(email, code);
            }

            pub fn get(&mut self, email: &Email) -> Option<u16> {
                self.0.get(email).map(|code| *code)
            }

            pub fn remove(&mut self, email: Email) {
                self.0.remove(&email);
            }
        }
    }
}
