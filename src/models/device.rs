use crate::relative_url::RelativeUrl;
use crate::session::Session;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
struct DeviceResponseRoot {
    pub result: i32,
    pub obj: DeviceResponse,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceResponse {
    #[serde(alias = "currPage")]
    pub curr_page: i32,
    pub pages: i32,
    #[serde(alias = "pageSize")]
    pub page_size: i32,
    pub count: i32,
    pub ind: i32,
    pub datas: Vec<Device>,
    #[serde(alias = "notPager")]
    pub not_pager: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Device {
    #[serde(alias = "accountName")]
    pub account_name: String,
    pub alias: String,
    #[serde(alias = "bdcStatus")]
    pub bdc_status: String,
    #[serde(alias = "datalogSn")]
    pub data_logger_serial_number: String,
    #[serde(alias = "datalogTypeTest")]
    pub data_logger_type_test: String,
    #[serde(alias = "deviceModel")]
    pub device_model: String,
    #[serde(alias = "deviceType")]
    pub device_type: String,
    #[serde(alias = "deviceTypeName")]
    pub device_type_name: String,
    #[serde(alias = "eMonth")]
    pub energy_month: String,
    #[serde(alias = "eToday")]
    pub energy_today: String,
    #[serde(alias = "eTotal")]
    pub energy_total: String,
    #[serde(alias = "lastUpdateTime")]
    pub last_update_time: String,
    pub location: String,
    #[serde(alias = "nominalPower")]
    pub nominal_power: String,
    pub pac: String,
    #[serde(alias = "plantId")]
    pub plant_id: String,
    #[serde(alias = "plantName")]
    pub plant_name: String,
    #[serde(alias = "sn")]
    pub serial_number: String,
    pub status: String,
    #[serde(alias = "timeServer")]
    pub time_server: String,
    #[serde(alias = "timezone")]
    pub time_zone: String,
}

impl Device {
    pub async fn by_plant(
        session: &mut Session,
        plant_id: &str,
        current_page: &str,
    ) -> Result<DeviceResponse, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::DevicesByPlantList.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("currPage", current_page);
        params.insert("plantId", plant_id);

        let response = session
            .post_message_return_response::<DeviceResponseRoot>(url, Some(params))
            .await;

        match response {
            Ok(res) => {
                if res.result == 1 {
                    Ok(res.obj)
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }
}
