pub mod ports;

use crate::domain::user::value_objects::id::UserId;

#[derive(Clone)]
pub struct Claims {
    exp: i64,
    iat: i64,
    sub: UserId,
}
