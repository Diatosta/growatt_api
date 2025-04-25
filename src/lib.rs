mod models;
mod relative_url;
mod session;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::device::Device;
    use crate::models::inverter_plant_parameters::Voltage;
    use crate::models::plant::Plant;
    use crate::models::tlx::Tlx;
    use crate::models::weather::Weather;
    use crate::models::{signal_helper, status_helper};
    use chrono::{Datelike, Duration, Timelike, Utc};

    static USERNAME: &str = "USERNAME";
    static PASSWORD: &str = "PASSWORD";

    #[tokio::test]
    async fn authenticate_successful() {
        let result = session::Session::new(USERNAME.to_string(), PASSWORD.to_string())
            .authenticate()
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_all_data() {
        // Fail if called with default credentials
        assert!(
            USERNAME != "USERNAME" && PASSWORD != "PASSWORD",
            "Please set USERNAME and PASSWORD to your credentials"
        );

        let mut session = session::Session::new(USERNAME.to_string(), PASSWORD.to_string());

        let plants = Plant::all(&mut session).await.unwrap();

        let first_plant = plants.first().unwrap().clone();

        let weather = Weather::by_plant(&mut session, &first_plant.id)
            .await
            .unwrap();

        println!("----- Weather -------");
        println!("{:<40}{}", "- City:", weather.city);
        println!(
            "{:<40}{}",
            "- Sunrise:",
            weather
                .data
                .weather_list
                .first()
                .map_or("", |weather_data| weather_data.basic.sunrise.as_str())
        );
        println!(
            "{:<40}{}",
            "- Sunset:",
            weather
                .data
                .weather_list
                .first()
                .map_or("", |weather_data| weather_data.basic.sunset.as_str())
        );
        println!(
            "{:<40}{}",
            "- Latitude:",
            weather
                .data
                .weather_list
                .first()
                .and_then(|weather_data| weather_data.basic.latitude.as_deref())
                .unwrap_or("")
        );
        println!(
            "{:<40}{}",
            "- Longitude:",
            weather
                .data
                .weather_list
                .first()
                .and_then(|weather_data| weather_data.basic.longitude.as_deref())
                .unwrap_or("")
        );
        println!(
            "{:<40}{} C",
            "- Temperature:",
            weather
                .data
                .weather_list
                .first()
                .map_or("", |weather_data| weather_data.now.temperature.as_str())
        );
        println!(
            "{:<40}{}",
            "- Cloud Volume:",
            weather
                .data
                .weather_list
                .first()
                .map_or("", |weather_data| weather_data.now.cloud.as_str())
        );
        println!(
            "{:<40}{}",
            "- Condition:",
            weather
                .data
                .weather_list
                .first()
                .map_or("", |weather_data| weather_data.now.condition_text.as_str())
        );
        println!("---------------------");
        println!();

        let devices = Device::by_plant(&mut session, &first_plant.id, "1")
            .await
            .unwrap();

        let first_device = devices.datas.first().unwrap().clone();

        let all_plant_data_for_given_day = Plant::detail_day_data_chart(
            &mut session,
            &first_plant.id,
            Utc::now(),
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let all_plant_voltage_data_for_given_day = Plant::detail_day_data_chart(
            &mut session,
            &first_plant.id,
            Utc::now(),
            Some(&first_device.serial_number),
            Some(Voltage::Vac1.as_str()),
            Some(&first_device.device_type_name),
        )
        .await
        .unwrap();

        let all_plant_power_data_for_given_month = Plant::detail_month_data_chart(
            &mut session,
            &first_plant.id,
            Utc::now(),
            Some(&first_device.serial_number),
            None,
            None,
            Some(&first_device.device_type_name),
        )
        .await
        .unwrap();

        let all_plant_data_for_given_year = Plant::detail_year_data_chart(
            &mut session,
            &first_plant.id,
            Utc::now(),
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let all_plant_power_data_per_year =
            Plant::detail_total_data_chart(&mut session, &first_plant.id, None, None, None)
                .await
                .unwrap();

        let now = Utc::now();
        let data_step_per_number_of_minutes = 5;

        // Calculate the index for the data 20 minutes ago
        let current_minutes = now.hour() * 60 + now.minute();

        let index_20_minutes_ago: usize = if current_minutes <= 20 {
            0
        } else {
            ((current_minutes - 20) / data_step_per_number_of_minutes) as usize
        };

        println!("----- My Solar Panel Information -------");

        // Today at HH:mm (20 minutes ago)
        println!(
            "{:<40}{} W",
            format!(
                "- Today at {}:",
                (now - Duration::minutes(-20)).format("%H:%M")
            ),
            all_plant_data_for_given_day
                .pac
                .and_then(|data| data.get(index_20_minutes_ago).copied())
                .unwrap_or(0.0)
        );

        println!(
            "{:<40}{} V",
            format!(
                "- Today at {}:",
                (now - Duration::minutes(-20)).format("%H:%M")
            ),
            all_plant_voltage_data_for_given_day
                .vac1
                .and_then(|data| data.get(index_20_minutes_ago).copied())
                .unwrap_or(0.0)
        );

        println!(
            "{:<40}{} kWh",
            format!("- {}:", now.format("%Y-%m-01")),
            all_plant_power_data_for_given_month
                .energy
                .and_then(|data| data.first().copied())
                .unwrap_or(0.0)
        );

        println!(
            "{:<40}{} kWh",
            format!("- {}:", now.format("%Y-%m")),
            all_plant_data_for_given_year
                .energy
                .and_then(|data| data.get((now.month() - 1) as usize).copied())
                .unwrap_or(0.0)
        );

        println!(
            "{:<40}{} kWh",
            format!("- {}:", now.format("%Y")),
            all_plant_power_data_per_year
                .energy
                .and_then(|data| data.last().copied())
                .unwrap_or(0.0)
        );

        println!("----------------------------------------");
        println!();

        let data_logger_device = Device::data_logger_device_info(
            &mut session,
            &first_plant.id,
            &first_device.data_logger_serial_number,
        )
        .await
        .unwrap();

        let data_logger_devices = Device::data_logger_devices(&mut session, &first_plant.id, "1")
            .await
            .unwrap();

        let first_data_logger_device = data_logger_devices.datas.first().unwrap().clone();

        let tlx_total_data =
            Tlx::total_data(&mut session, &first_plant.id, &first_device.serial_number)
                .await
                .unwrap();

        let device_status = status_helper::get_device_type_status(&first_device);

        println!("----- My Photovoltaic Devices -------");

        println!(
            "{:<40}{}",
            "- Device Serial Number:", first_device.serial_number
        );
        println!("{:<40}{}", "- Device User Name:", first_device.account_name);
        println!(
            "{:<40}{}",
            "- Device Today (kWh):", first_device.energy_today
        );
        println!(
            "{:<40}{}",
            format!("- Today ({}):", tlx_total_data.unit),
            tlx_total_data.photovoltaic_revenue_today
        );
        println!("{:<40}{}", "- Status:", device_status);
        println!("{:<40}{}", "- Plant Name:", first_device.plant_name);
        println!("{:<40}{}", "- This Month (kWh):", first_device.energy_month);
        println!(
            "{:<40}{}",
            "- Server Update Time:", first_device.time_server
        );
        println!(
            "{:<40}{}",
            "- Data Logger:", first_device.data_logger_serial_number
        );
        println!(
            "{:<40}{}",
            "----- Signal:",
            signal_helper::get_sim_signal_text(
                first_data_logger_device.sim_signal,
                &first_data_logger_device.device_type_indicate
            )
        );
        println!(
            "{:<40}{}",
            "----- Collector Model:", first_device.data_logger_type_test
        );
        println!(
            "{:<40}{}",
            "----- Firmware Version:", data_logger_device.firmware_version
        );
        println!(
            "{:<40}{}",
            "----- Ip & Port:", data_logger_device.ip_and_port
        );
        println!(
            "{:<40}{} Minute",
            "----- Data Update Interval:", data_logger_device.interval
        );
        println!(
            "{:<40}{}",
            "----- Wireless type:", first_data_logger_device.wireless_type
        );
        println!(
            "{:<40}{}",
            "- Total Energy (kWh):", first_device.energy_total
        );
        println!("{:<40}{}", "- Rated Power (W):", first_device.nominal_power);
        println!("{:<40}{}", "- Current Power (W):", first_device.pac);

        println!("-------------------");
        println!();
    }
}
