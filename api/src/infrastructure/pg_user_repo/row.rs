use crate::domain::user::{
    ports::UserRepoError,
    value_objects::{email::Email, id::UserId, password_hash::PasswordHash},
    User,
};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    email: String,
    id:    Uuid,
}

impl TryFrom<UserRow> for User {
    type Error = UserRepoError;

    fn try_from(r: UserRow) -> Result<User, UserRepoError> {
        let email = Email::new(r.email)?;
        let id    = UserId::reconstitute(r.id);
        let user  = User::reconstitute(email, id, PasswordHash::new(String::new()));
        Ok(user)
    }
}
