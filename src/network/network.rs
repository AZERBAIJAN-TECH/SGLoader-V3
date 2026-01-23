use reqwest::Client;

use crate::AUTH_URL;

//LOGIN (TODO)
pub fn check_login() {
    let url = format!("{AUTH_URL}/api/auth/authenticate");

    let client = Client::new();

    let resp = client.post(url);
}