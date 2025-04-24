mod models;
mod relative_url;
mod session;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::inverter_plant_parameters::Voltage;
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

        let first_device = devices.datas.first().unwrap().clone();

        let all_plant_data_for_given_day = Plant::detail_day_data_chart(
            &mut session,
            &first_plant.id,
            chrono::Utc::now(),
            None,
            None,
            None,
        )
        .await
        .unwrap();

        println!("{:#?}", all_plant_data_for_given_day);

        let all_plant_voltage_data_for_given_day = Plant::detail_day_data_chart(
            &mut session,
            &first_plant.id,
            chrono::Utc::now(),
            Some(&first_device.serial_number),
            Some(Voltage::Vac1.as_str()),
            Some(&first_device.device_type_name),
        )
        .await
        .unwrap();

        println!("{:#?}", all_plant_voltage_data_for_given_day);

        let all_plant_power_data_for_given_month = Plant::detail_month_data_chart(
            &mut session,
            &first_plant.id,
            chrono::Utc::now(),
            Some(&first_device.serial_number),
            None,
            None,
            Some(&first_device.device_type_name),
        )
        .await
        .unwrap();

        println!("{:#?}", all_plant_power_data_for_given_month);

        let all_plant_data_for_given_year = Plant::detail_year_data_chart(
            &mut session,
            &first_plant.id,
            chrono::Utc::now(),
            None,
            None,
            None,
        )
        .await
        .unwrap();

        println!("{:#?}", all_plant_data_for_given_year);

        let all_plant_power_data_per_year =
            Plant::detail_total_data_chart(&mut session, &first_plant.id, None, None, None)
                .await
                .unwrap();

        println!("{:#?}", all_plant_power_data_per_year);
    }
}
