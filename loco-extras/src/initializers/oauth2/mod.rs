use std::collections::BTreeMap;
use std::env;
use axum::{Extension, Router as AxumRouter};
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
use crate::initializers::axum_login::users::Backend;
use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};
use std::sync::{Arc, Mutex};
use loco_oauth2::config::OAuth2Config;
use loco_oauth2::OAuth2ClientStore;

pub struct OAuth2StoreInitializer;

#[async_trait]
impl Initializer for OAuth2StoreInitializer {
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
        Ok(router.layer(Extension(oauth2_store)))
    }
}