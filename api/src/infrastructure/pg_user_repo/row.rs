use crate::domain::user::{
    ports::UserRepoError,
    value_objects::{email::Email, id::UserId, password_hash::PasswordHash},
    User,
};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    email:         String,
    id:            Uuid,
}

impl TryFrom<UserRow> for User {
    type Error = UserRepoError;

    fn try_from(r: UserRow) -> Result<User, UserRepoError> {
        let email         = Email::new(r.email)?;
        let id            = UserId::reconstitute(r.id);
        let user          = User::reconstitute(email, id);
        Ok(user)
    }
}

#[derive(sqlx::FromRow)]
pub struct UserRowWithPasswordHash {
    email:         String,
    id:            Uuid,
    password_hash: String,
}

impl TryFrom<UserRowWithPasswordHash> for User {
    type Error = UserRepoError;

    fn try_from(r: UserRowWithPasswordHash) -> Result<User, UserRepoError> {
        let email         = Email::new(r.email)?;
        let id            = UserId::reconstitute(r.id);
        let password_hash = PasswordHash::new(r.password_hash);
        let user          = User::reconstitute_with_password_hash(email, id, password_hash);
        Ok(user)
    }
}
