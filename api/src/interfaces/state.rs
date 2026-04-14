use std::sync::Arc;

use crate::{
    application::{
        auth::{LoginUseCase, RegisterUseCase},
        product::{CreateProductUseCase, ListProductsUseCase},
    },
    domain::auth::service::TokenService,
};

#[derive(Clone)]
pub struct AppState {
    /// Kept in state so the auth middleware can validate tokens.
    pub token_service: Arc<dyn TokenService>,
    pub register_uc: Arc<RegisterUseCase>,
    pub login_uc: Arc<LoginUseCase>,
    pub create_product_uc: Arc<CreateProductUseCase>,
    pub list_products_uc: Arc<ListProductsUseCase>,
}
