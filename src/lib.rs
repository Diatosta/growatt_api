mod models;
mod relative_url;
mod session;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plant::Plant;

    static USERNAME: &str = "USERNAME";
    static PASSWORD: &str = "PASSWORD";

    #[tokio::test]
    async fn authenticate_successful() {
        let result = session::Session::new(USERNAME.to_string(), PASSWORD.to_string())
            .authenticate()
            .await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test() {
        let mut session = session::Session::new(USERNAME.to_string(), PASSWORD.to_string());

        let result = Plant::all(&mut session).await;

        assert_eq!(result.is_ok(), true);
    }
}
