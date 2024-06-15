use std::collections::BTreeMap;
use std::env;
use axum::{Router as AxumRouter};
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum_login::{AuthUser, AuthnBackend, UserId, AuthManagerLayerBuilder};
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::tower_sessions::cookie::time::Duration;
use loco_rs::{
    app::{AppContext, Initializer},
    Error, Result,
};
use ::reqwest::blocking::Client;
use oauth2::{basic::{BasicClient, BasicRequestTokenError}, reqwest::{async_http_client, AsyncHttpClientError}, url::Url, AuthorizationCode, CsrfToken, TokenResponse, reqwest, AuthUrl, TokenUrl, ClientId, ClientSecret, http};
use sea_orm::{Database, DatabaseConnection, DbBackend, Schema};
use sea_orm::sea_query::{Table, TableCreateStatement};
use sea_orm_migration::MigrationTrait;
use sea_orm_migration::schema::{pk_auto, string, table_auto};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn, error};
use loco_rs::prelude::cookie::SameSite;
use crate::initializers::axum_login::config::{OAuth2Config, apply_default_urls, OAuthClientConfig};
use crate::initializers::axum_login::users::Backend;
use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};
use std::sync::{Arc, Mutex};

pub struct AxumLoginInitializer;

#[async_trait]
impl Initializer for AxumLoginInitializer {
    fn name(&self) -> String {
        "axum-login".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        // Get all the settings from the config
        let settings = ctx
            .config
            .settings
            .clone()
            .ok_or_else(|| Error::Message("settings config not configured".to_string()))?;
        // Get the oauth2 config in json format
        let oauth2_config_value = settings
            .get("oauth2")
            .ok_or(Error::Message("oauth2 config not found".to_string()))?
            .clone();
        // Convert the oauth2 config json to OAuth2Config
        let oauth2_config: OAuth2Config = oauth2_config_value.try_into().map_err(|e| {
            tracing::error!(error = ?e, "could not convert oauth2 config");
            Error::Message("could not convert oauth2 config".to_string())
        })?;
        // Create the OAuth2ClientStore
        let oauth2_store = OAuth2ClientStore::new(oauth2_config).map_err(|e| {
            tracing::error!(error = ?e, "could not create oauth2 store");
            Error::Message("could not create oauth2 store".to_string())
        })?;

        if !oauth2_config.enabled {
            debug!("oauth2 is enabled");
            let client_configs: Vec<OAuthClientConfig> = oauth2_config.clients.into_iter()
                .map(apply_default_urls).collect();

            let client_config = client_configs.get(0).unwrap();
            let auth_url = match AuthUrl::new(client_config.url_config.auth_url.clone()) {
                Ok(url) => url,
                Err(e) => return Err(Error::Message(e.to_string())),
            };

            let token_url = match TokenUrl::new(client_config.url_config.token_url.clone()) {
                Ok(url) => url,
                Err(e) => return Err(Error::Message(e.to_string())),
            };

            let client = BasicClient::new(ClientId::new(client_config.client_credentials.client_id.clone()), Some(ClientSecret::new(client_config.client_credentials.client_secret.clone())), auth_url, Some(token_url));

            // let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
            let pool = match SqlitePool::connect(":memory:").await {
                Ok(pool) => pool,
                Err(e) => return Err(Error::Message(e.to_string())),
            };
            let db_conn: DatabaseConnection = Database::connect(":memory:").await.map_err(|e| Error::Message(e.to_string()))?;

            crate::initializers::axum_login::migration::setup_schema(&db_conn).await;


            let session_store = MemoryStore::default();
            let session_layer = SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
                .with_expiry(Expiry::OnInactivity(Duration::days(1)));

            let backend = Backend::new(pool, client);
            let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
            Ok(router.layer(auth_layer))
        } else {
            debug!("oauth2 is not enabled");

            Ok(router)
        }
    }
}
#[derive(Clone)]
pub struct OAuth2ClientStore {
    clients: BTreeMap<String, OAuth2ClientGrantEnum>
}

// let settings = match ctx.config.settings.clone(){
// Some(value)=> value,
// None => {
// error!("Settings are missing from the configuration");
// return Ok(router);
// }
// };
//
// let oauth2_config_option = match settings.get("oauth2"){
// Some(value)=> value,
// None => {
// error!("Block oauth2 is missing from the settings");
// return Ok(router);
// }
// };
// let config_option = serde_json::from_value(oauth2_config_option.clone()).ok();
//
// let oauth2_config: OAuth2Config = match config_option {
// Some(config) => config_option,
// None => {
// error!("oauth2 configuration is missing or invalid");
// return Ok(router);
// }
// };
