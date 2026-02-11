use reqwest::{Client, StatusCode};
use serde::{Serialize, Deserialize};
use thiserror::Error;

use uuid::Uuid;

use crate::AUTH_URL;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    Username: String,
    Password: String,
    TfaCode: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    token: String,
    username: String,
    userId: Uuid,
    expireTime: String
}

pub async fn attempt_login(username: String, password: String) -> Result<bool, Error> {
    let url = format!("{}/api/auth/authenticate", AUTH_URL.trim_end_matches('/'));

    let client = Client::new();

    let request = AuthRequest {
        Username: username, 
        Password: password, 
        TfaCode: None
    };

    let response = client.post(&url)
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    match status {
        StatusCode::OK => {
            let _resp_json: AuthResponse = serde_json::from_str(&response_text)?;
            //TODO: save data
            Ok(true)
        }

        StatusCode::UNAUTHORIZED => {
            println!("Login failed"); //TODO: replace this with ui
            Ok(false)
        }

        _ => {
            println!("Login failed"); //TODO: replace this with ui
            Ok(false)
        }
    }
}
