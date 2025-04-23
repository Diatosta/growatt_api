use crate::models::login_response::LoginResponse;
use crate::relative_url::RelativeUrl;
use reqwest::{Client, Error, StatusCode, Url};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Session {
    pub username: String,
    pub password: String,
    pub api_base_url: Url,
    pub is_authenticated: bool,
    pub cookie_container: Arc<reqwest_cookie_store::CookieStoreMutex>,
    pub client: Client,
}

impl Session {
    pub fn new(username: String, password: String) -> Self {
        let cookie_container = Arc::new(reqwest_cookie_store::CookieStoreMutex::new(
            reqwest_cookie_store::CookieStore::new(None),
        ));

        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_container))
            .build()
            .expect("Failed to create HTTP client");

        let api_base_url = Url::parse("https://server.growatt.com").expect("Invalid URL");

        Session {
            username,
            password,
            api_base_url,
            is_authenticated: false,
            cookie_container,
            client,
        }
    }

    async fn build_get<T>(&self, url: Url) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await;

        self.handle_response(response).await
    }

    async fn build_post<T>(&self, url: Url, form: HashMap<&str, &str>) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        let response = self.client.post(url).form(&form).send().await;

        self.handle_response(response).await
    }

    async fn handle_response<T>(
        &self,
        response: Result<reqwest::Response, Error>,
    ) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        let content = match response {
            Ok(r) => {
                if r.status().is_success() {
                    r.json::<T>().await
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            Err(e) => {
                return if e.is_status() {
                    Err(e.status().unwrap())
                } else {
                    Err(StatusCode::BAD_REQUEST)
                };
            }
        };

        match content {
            Ok(s) => Ok(s),
            Err(e) => {
                println!("{:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), StatusCode> {
        let validate_code = String::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("account", &self.username);
        params.insert("password", &self.password);
        params.insert("validateCode", validate_code.as_str());

        let url = match self.api_base_url.join(RelativeUrl::Login.as_str()) {
            Ok(url) => url,
            Err(e) => {
                println!("Error joining URL: {:?}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        };

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

    pub async fn get_message_return_response<T>(
        &mut self,
        url: Url,
        expected_status_code: StatusCode,
    ) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        // Get if any cookies are expired
        let non_expired_cookie_count = {
            let cookies = self.cookie_container.lock().unwrap();
            cookies.iter_unexpired().count()
        };

        if !self.is_authenticated || non_expired_cookie_count <= 0 {
            self.authenticate().await?;
        }

        self.build_get(url).await
    }
}
