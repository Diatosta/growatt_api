use crate::models::login_response::LoginResponse;
use crate::relative_url::RelativeUrl;
use reqwest::{Client, Error, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct Session {
    pub username: String,
    pub password: String,
    pub api_base_url: String,
    pub is_authenticated: bool,
    pub client: Client,
}

impl Session {
    pub fn new(username: String, password: String) -> Self {
        Session {
            username,
            password,
            api_base_url: "https://server.growatt.com".to_string(),
            is_authenticated: false,
            client: Client::new(),
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), StatusCode> {
        let validate_code = String::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("account", &self.username);
        params.insert("password", &self.password);
        params.insert("validateCode", validate_code.as_str());

        let url = format!("{}/{}", self.api_base_url, RelativeUrl::Login.as_str());
        let response: Result<LoginResponse, StatusCode> = self.build_post(url, params).await;

        match response {
            Ok(t) => {
                if t.result == 1 {
                    self.is_authenticated = true;
                    return Ok(());
                }

                println!("Authentication returned not 1: {:?}", t);
                Err(StatusCode::UNAUTHORIZED)
            }
            Err(e) => {
                println!("Authentication failed: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn build_get<T>(&self, url: String) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await;

        self.handle_response(response).await
    }

    pub async fn build_post<T>(
        &self,
        url: String,
        form: HashMap<&str, &str>,
    ) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new().post(url).form(&form).send().await;

        self.handle_response(response).await
    }

    async fn handle_response<T>(
        &self,
        response: Result<reqwest::Response, Error>,
    ) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        match &response {
            Ok(r) => {
                if !r.status().is_success() {
                    return Err(r.status());
                }
            }
            Err(e) => {
                return if e.is_status() {
                    Err(e.status().unwrap())
                } else {
                    Err(StatusCode::BAD_REQUEST)
                };
            }
        }

        let content = response.unwrap().json::<T>().await;

        match content {
            Ok(s) => Ok(s),
            Err(e) => {
                println!("{:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}
