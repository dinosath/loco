use std::env;
use axum::{async_trait, Router as AxumRouter};
use axum_login::AuthManagerLayerBuilder;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::tower_sessions::cookie::time::Duration;
use loco_rs::{
    app::{AppContext, Initializer},
    Error, Result,
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl, CsrfToken};
use oauth2::{
    reqwest::{async_http_client, AsyncHttpClientError},
    url::Url,
    AuthorizationCode, TokenResponse,
};
use sea_orm::{Database, DatabaseConnection, DbBackend, Schema};
use sea_orm::sea_query::{Table, TableCreateStatement};
use sea_orm_migration::MigrationTrait;
use loco_rs::prelude::cookie::SameSite;
pub struct AxumLoginInitializer;

#[async_trait]
impl Initializer for AxumLoginInitializer {
    fn name(&self) -> String {
        "axum-login".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let client_id = env::var("CLIENT_ID")
            .map(ClientId::new)
            .expect("CLIENT_ID should be provided.");
        let client_secret = env::var("CLIENT_SECRET")
            .map(ClientSecret::new)
            .expect("CLIENT_SECRET should be provided");

        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url));


        let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
        crate::initializers::axum_login::migration::Migration::up(&db, None).await.unwrap();

        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        let backend = Backend::new(db, client);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
        Ok(router.layer(auth_layer))
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    conn: DatabaseConnection,
    client: BasicClient,
}

impl Backend {
    pub fn new(conn: DatabaseConnection, client: BasicClient) -> Self {
        Self { conn, client }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client.authorize_url(CsrfToken::new_random).url()
    }
}