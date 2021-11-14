use actix_web::{error::BlockingError, web};
use diesel::prelude::*;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, SlimUser, User};
use crate::utils::verify;

#[derive(Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

pub async fn fomo(
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<String, ServiceError> {
    let res = web::block(move || query(auth_data.into_inner(), pool)).await;

    match res {
        Ok(user) => Ok(format!("all set {}", user.email)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
