#[derive(Clone)]
pub struct OAuth2ClientStore {
    clients: BTreeMap<String, OAuth2ClientGrantEnum>,
    pub key: Key,
}