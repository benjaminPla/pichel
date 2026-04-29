use uuid::Uuid;

use crate::domain::user::{
    aggregate_root::User,
    repository::UserRepoError,
    value_objects::{email::Email, id::UserId, password_hash::PasswordHash},
};

#[derive(sqlx::FromRow)]
pub struct UserRow {
    email:         String,
    id:            Uuid,
    password_hash: String,
}

impl TryFrom<UserRow> for User {
    type Error = UserRepoError;

    fn try_from(r: UserRow) -> Result<User, UserRepoError> {
        let email         = Email::new(r.email).map_err(|e| UserRepoError::Mapping(e.to_string()))?;
        let id            = UserId::reconstitute(r.id);
        let password_hash = PasswordHash::new(r.password_hash);
        let user          = User::reconstitute(email, id, password_hash);
        Ok(user)
    }
}
