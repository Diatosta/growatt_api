mod models;
mod plants_list;
mod query_builder;
mod relative_url;
mod session;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn authenticate_successful() {
        let result = session::Session::new("username".to_string(), "password".to_string())
            .authenticate()
            .await;
        assert_eq!(result.is_ok(), true);
    }
}
