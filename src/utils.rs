use crate::errors::ServiceError;
use argon2::{self, Config};

const SECRET_KEY: &'static [u8] = b"0123";

const SALT: &'static [u8] = b"supersecuresalt";

// Warning this is only for development

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY,
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError // should output "Internal Server Error"
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY, &[])
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}
