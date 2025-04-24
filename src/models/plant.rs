use crate::models::inverter_plant_parameters::Power;
use crate::relative_url::RelativeUrl;
use crate::session::Session;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
struct DeviceDataChartResponseRoot {
    pub result: i32,
    pub obj: Vec<DeviceDataChartResponse>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceDataChartResponse {
    pub datas: DeviceDataChart,
    #[serde(alias = "sn")]
    pub serial_number: String,
    #[serde(alias = "type")]
    pub data_type: String,
    pub params: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceDataChart {
    pub pac: Option<Vec<Option<f32>>>,
    #[serde(alias = "VAC1")]
    pub vac1: Option<Vec<Option<f32>>>,
    pub energy: Option<Vec<Option<f32>>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Plant {
    pub id: String,
    #[serde(alias = "plantName")]
    pub plant_name: String,
    #[serde(alias = "timezone")]
    pub time_zone: String,
}

impl Plant {
    pub async fn all(session: &mut Session) -> Result<Vec<Plant>, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::PlantList.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        session.get_message_return_response(url).await
    }

    pub async fn detail_day_data_chart(
        session: &mut Session,
        plant_id: &str,
        date: DateTime<Utc>,
        serial_number: Option<&str>,
        param: Option<&str>,
        device_type_name: Option<&str>,
    ) -> Result<DeviceDataChart, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::InverterEnergyDataDayChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let json_data_type = match serial_number {
            Some(_) => device_type_name.unwrap_or(""),
            None => "plant",
        };
        let sn = serial_number.unwrap_or(plant_id);
        let param = param.unwrap_or(Power::Pac.as_str());
        let date = &date.format("%Y-%m-%d").to_string();
        let json_data = format!(
            "[{{\"type\":\"{}\",\"sn\":\"{}\",\"params\":\"{}\"}}]",
            json_data_type, sn, param
        );

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("plantId", plant_id);
        params.insert("date", date);
        params.insert("jsonData", json_data.as_str());

        let response = session
            .post_message_return_response::<DeviceDataChartResponseRoot>(url, Some(params))
            .await;

        match response {
            Ok(res) => {
                if res.result == 1 {
                    let data = res.obj.first().unwrap().datas.clone();

                    Ok(data)
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    pub async fn detail_month_data_chart(
        session: &mut Session,
        plant_id: &str,
        date: DateTime<Utc>,
        serial_number: Option<&str>,
        param: Option<&str>,
        device_type: Option<&str>,
        device_type_name: Option<&str>,
    ) -> Result<DeviceDataChart, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::InverterEnergyDataMonthChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let json_data_type = match serial_number {
            Some(_) => device_type_name.unwrap_or(""),
            None => "plant",
        };
        let sn = if device_type.is_none() && serial_number.is_some() {
            serial_number.unwrap().to_string()
        } else {
            plant_id.to_string()
        };
        let param = param.unwrap_or(Power::Energy.as_str());
        let date = &date.format("%Y-%m-%d").to_string();
        let json_data = format!(
            "[{{\"type\":\"{}\",\"sn\":\"{}\",\"params\":\"{}\"}}]",
            json_data_type, sn, param
        );

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("plantId", plant_id);
        params.insert("date", date);
        params.insert("jsonData", json_data.as_str());

        let response = session
            .post_message_return_response::<DeviceDataChartResponseRoot>(url, Some(params))
            .await;

        match response {
            Ok(res) => {
                if res.result == 1 {
                    let data = res.obj.first().unwrap().datas.clone();

                    Ok(data)
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    pub async fn detail_year_data_chart(
        session: &mut Session,
        plant_id: &str,
        date: DateTime<Utc>,
        serial_number: Option<&str>,
        param: Option<&str>,
        device_type: Option<&str>,
    ) -> Result<DeviceDataChart, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::InverterEnergyDataYearChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let json_data_type = match device_type {
            Some(_) => device_type.unwrap_or(""),
            None => match serial_number {
                None => "plant",
                Some(_) => "inv",
            },
        };
        let sn = if device_type.is_none() && serial_number.is_some() {
            serial_number.unwrap().to_string()
        } else {
            plant_id.to_string()
        };
        let param = param.unwrap_or(Power::Energy.as_str());
        let year = &date.format("%Y").to_string();
        let json_data = format!(
            "[{{\"type\":\"{}\",\"sn\":\"{}\",\"params\":\"{}\"}}]",
            json_data_type, sn, param
        );

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("plantId", plant_id);
        params.insert("year", year);
        params.insert("jsonData", json_data.as_str());

        let response = session
            .post_message_return_response::<DeviceDataChartResponseRoot>(url, Some(params))
            .await;

        match response {
            Ok(res) => {
                if res.result == 1 {
                    let data = res.obj.first().unwrap().datas.clone();

                    Ok(data)
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }
}
