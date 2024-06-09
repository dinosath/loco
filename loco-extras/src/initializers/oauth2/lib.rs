use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenUrl
};
use oauth2::basic::BasicClient;
use crate::initializers::oauth2::config::OAuthClientConfig;

pub fn create_client_from_config(mut oauth_client_config: OAuthClientConfig) -> Option<BasicClient> {

    let client = BasicClient::new(
            ClientId::new(oauth_client_config.client_credentials.client_id),
            Some(ClientSecret::new(oauth_client_config.client_credentials.client_secret)),
            AuthUrl::new(oauth_client_config.url_config.auth_url).ok()?,
            Some(TokenUrl::new(oauth_client_config.url_config.token_url).ok()?)
        )
            .set_redirect_uri(RedirectUrl::new(oauth_client_config.url_config.redirect_url).ok()?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(oauth_client_config.url_config.scopes.iter().map(|x| Scope::new(x.clone())))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    Some(client)
}