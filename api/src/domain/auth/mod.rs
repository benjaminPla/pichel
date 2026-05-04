pub mod ports;

use crate::domain::user::value_objects::id::UserId;

pub struct Claims {
    exp: i64,
    iat: i64,
    sub: UserId,
}
