use std::sync::Arc;

use crate::{
    application::user::errors::UserAppError,
    domain::user::{aggregate_root::User, ports::repository::UserRepo},
};

// ── Query ────────────────────────────────────────────────────────────────

pub struct UserGetAllQuery {
    pub page:     i64,
    pub per_page: i64,
}

// ── Handler ──────────────────────────────────────────────────────────────

pub struct UserGetAllHandler {
    user_repo: Arc<dyn UserRepo>,
}

impl UserGetAllHandler {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self,
        query: UserGetAllQuery,
    ) -> Result<(Vec<User>, i64), UserAppError> {
        let users = self.user_repo.get_all(query.page, query.per_page).await?;
        Ok(users)
    }
}
