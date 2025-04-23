mod models;
mod relative_url;
mod session;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plant::Plant;
    use crate::relative_url::RelativeUrl;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn authenticate_successful() {
        let result = session::Session::new("username".to_string(), "password".to_string())
            .authenticate()
            .await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test() {
        let mut session = session::Session::new("username".to_string(), "password".to_string());

        session.authenticate().await.unwrap();

        let url = session
            .api_base_url
            .join(RelativeUrl::PlantList.as_str())
            .unwrap();

        let result: Result<Vec<Plant>, StatusCode> = session
            .get_message_return_response(url, StatusCode::OK)
            .await;

        println!("{:?}", result);

        assert_eq!(result.is_ok(), true);
    }
}
