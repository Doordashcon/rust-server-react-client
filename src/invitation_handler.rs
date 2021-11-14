use actix_web::{error::BlockingError, web};
use diesel::prelude::*;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool};

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String
}

pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>, 
    pool: web::Data<Pool>
) -> Result<String, ServiceError> {
    // run diesel blocking code
    let res = web::block(move || query(invitation_data.into_inner().email, pool)).await;

    match res {
        Ok(info) => Ok(format!("Here's your Id:{0} registered on:{2} email:{1}", info.id, info.email, info.expires_at)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError)
        },
    }
}

// Diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, ServiceError> {
    use crate::schema::invitations::dsl::invitations;
    use crate::schema::users::dsl::users;

    let conn: &PgConnection = &pool.get().unwrap();

    let count: i64 = users.count().get_result(conn).unwrap();

    dbg!("I count {:?} rows",count);

    if count < 10 {
        let new_invitation: Invitation = eml.into();
        let inserted_invitation = diesel::insert_into(invitations)
            .values(&new_invitation)
            .get_result(conn)?;

        Ok(inserted_invitation)
    } else {
        Err(ServiceError::Unauthorized)
    }
}
