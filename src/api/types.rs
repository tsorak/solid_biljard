pub use crate::api::auth::email_code;

pub mod login {
    pub enum IdentificationMethod {
        Email(String),
        Username(String),
    }
}
