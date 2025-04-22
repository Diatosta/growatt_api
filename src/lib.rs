mod models;
mod plants_list;
mod query_builder;
mod relative_url;
mod session;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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
