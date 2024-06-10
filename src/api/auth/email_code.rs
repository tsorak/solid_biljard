use axum::{extract::State, response::Response, Json};
use serde::Deserialize;

use crate::ext::res;

type Email = String;
type Code = String;

#[derive(Debug, Deserialize)]
pub struct NewCodeReq {
    email: Email,
}

pub async fn new_code(state: State<crate::State>, body: Json<NewCodeReq>) -> Response {
    let email = body.email.clone();

    match state.email_code_session.new_code(email).await {
        Ok(_) => res::Json::new("Session created")
            .with_uid("session_created")
            .status(201),
        // failed to send request over channel
        Err(_) => res::Json::new("Server error").status(500),
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidateCodeReq {
    email: Email,
    code: Code,
}

pub enum ValidateCodeError {
    // No session matches the email in the request
    SessionMissing,
    // Server fault
    ServerError,
}

pub async fn validate_code(state: State<crate::State>, body: Json<ValidateCodeReq>) -> Response {
    let ValidateCodeReq { email, code } = body.0;

    match state.email_code_session.validate_code(email, code).await {
        Ok(is_correct) => {
            if is_correct {
                let mut res = res::Json::new("Correct code")
                    .with_uid("correct_code")
                    .status(200);

                let _headers = res.headers_mut();
                // TODO: do some jwt stuff

                res
            } else {
                res::Json::new("Incorrect code")
                    .with_uid("incorrect_code")
                    .status(400)
            }
        }
        Err(ValidateCodeError::SessionMissing) => {
            res::Json::new("No session active for the specified email")
                .with_uid("session_missing")
                .status(400)
        }
        Err(ValidateCodeError::ServerError) => res::Json::new("Server error").status(500),
    }
}

pub mod session {
    use tokio::{
        sync::{
            mpsc::{channel, Receiver, Sender},
            oneshot,
        },
        // task::AbortHandle,
    };

    use super::{Code, Email, ValidateCodeError};

    enum Request {
        NewCode(Email),
        ValidateCode(
            (
                Email,
                Code,
                oneshot::Sender<anyhow::Result<bool, ValidateCodeError>>,
            ),
        ),
    }

    #[derive(Debug, Clone)]
    pub struct CodeSession {
        tx: Sender<Request>,
    }

    impl CodeSession {
        pub fn new() -> Self {
            let (tx, rx) = channel::<Request>(16);

            let _handle = tokio::spawn(async move { session_event_loop(rx).await }).abort_handle();

            Self { tx }
        }

        pub async fn new_code(&self, email: Email) -> anyhow::Result<()> {
            Ok(self.tx.send(Request::NewCode(email)).await?)
        }

        pub async fn validate_code(
            &self,
            email: Email,
            code: Code,
        ) -> anyhow::Result<bool, ValidateCodeError> {
            let (resolver, rx) = oneshot::channel::<anyhow::Result<bool, ValidateCodeError>>();

            self.tx
                .send(Request::ValidateCode((email, code, resolver)))
                .await
                .map_err(|_| ValidateCodeError::ServerError)?;

            rx.await.map_err(|_| ValidateCodeError::ServerError)?
        }
    }

    async fn session_event_loop(mut rx: Receiver<Request>) {
        let mut state = code_store::CodeStore::new();

        loop {
            if let Some(m) = rx.recv().await {
                match m {
                    Request::NewCode(email) => {
                        // TODO: generate random 4-digit code
                        let code = "0000".to_string();

                        state.insert(email, code);
                    }
                    Request::ValidateCode((email, code, response_channel)) => {
                        let correct_code = match state.get(&email) {
                            Some(v) => v,
                            None => {
                                let _ =
                                    response_channel.send(Err(ValidateCodeError::SessionMissing));
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

        use super::{Code, Email};

        pub struct CodeStore(HashMap<Email, Code>);

        impl CodeStore {
            pub fn new() -> Self {
                Self(HashMap::new())
            }

            pub fn insert(&mut self, email: Email, code: Code) {
                self.0.insert(email, code);
            }

            pub fn get(&mut self, email: &Email) -> Option<Code> {
                self.0.get(email).map(|v| v.to_owned())
            }

            pub fn remove(&mut self, email: Email) {
                self.0.remove(&email);
            }
        }
    }
}
