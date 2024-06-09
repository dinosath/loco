use axum::{async_trait, Extension, Router as AxumRouter};
use tracing::{debug, error, info, warn};
use loco_rs::prelude::*;
use crate::initializers::oauth2::config::{OAuth2Config,apply_default_urls};
use oauth2::basic::BasicClient;
use tracing::log::trace;
use crate::initializers::oauth2::lib::create_client_from_config;

pub struct OAuth2Initializer;

#[async_trait]
impl Initializer for OAuth2Initializer {
    fn name(&self) -> String {
        "oauth2".to_string()
    }
    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        // Get all the settings from the config
        info!("oauth2 configuration is missing or invalid");
        let oauth2_config = ctx
            .config
            .settings
            .clone()
            .and_then(|t| t.get("oauth2").cloned())
            .and_then(|t| serde_json::from_value(t).ok());

        let oauth2_config: OAuth2Config = match oauth2_config {
            Some(config) => config,
            None => {
                warn!("oauth2 configuration is missing or invalid");
                return Ok(router);
            }
        };

        if !oauth2_config.enabled {
            debug!("oauth2 is enabled");
            let clients: Vec<BasicClient> = oauth2_config.clients.into_iter()
                .map(apply_default_urls)
                .flat_map(create_client_from_config)
                .collect();
            Ok(router.layer(Extension(clients)))
        } else {
            Ok(router)
        }
    }
}