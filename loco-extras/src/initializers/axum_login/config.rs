use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct OAuth2Config {
    pub enabled: bool,
    pub clients: Vec<OAuthClientConfig>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthClientConfig {
    pub provider: String,
    pub client_credentials: ClientCredentials,
    pub url_config: UrlConfig,
    pub cookie_config: CookieConfig,
    pub timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct UrlConfig {
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub profile_url: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CookieConfig {
    protected_url: String,
}

pub fn apply_default_urls(mut client: OAuthClientConfig) -> OAuthClientConfig {
    let default_url_config = match client.provider.as_str() {
        "google" => UrlConfig {
            auth_url: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v3/token".to_string(),
            redirect_url: "http://localhost:3000/api/oauth2/google/callback".to_string(),
            profile_url: "https://openidconnect.googleapis.com/v1/userinfo".to_string(),
            scopes: vec![
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ],
        },
        _ => UrlConfig {
            auth_url: "".to_string(),
            token_url: "".to_string(),
            redirect_url: "".to_string(),
            profile_url: "".to_string(),
            scopes: vec![],
        },
    };

    if client.url_config.auth_url.is_empty() {
        client.url_config.auth_url = default_url_config.auth_url;
    }
    if client.url_config.token_url.is_empty() {
        client.url_config.token_url = default_url_config.token_url;
    }
    if client.url_config.redirect_url.is_empty() {
        client.url_config.redirect_url = default_url_config.redirect_url;
    }
    if client.url_config.profile_url.is_empty() {
        client.url_config.profile_url = default_url_config.profile_url;
    }
    if client.url_config.scopes.is_empty() {
        client.url_config.scopes = default_url_config.scopes;
    }

    client
}