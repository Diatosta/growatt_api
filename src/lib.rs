mod models;
mod relative_url;
mod session;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::plant::Plant;
    use crate::models::weather::Weather;

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

        let plants = Plant::all(&mut session).await.unwrap();

        println!("{:#?}", plants);

        let first_plant = plants.first().unwrap().clone();

        let weather = Weather::by_plant(&mut session, &first_plant.id)
            .await
            .unwrap();

        println!("{:#?}", weather);

        let devices = models::device::Device::by_plant(&mut session, &first_plant.id, "1")
            .await
            .unwrap();

        println!("{:#?}", devices);
    }
}
