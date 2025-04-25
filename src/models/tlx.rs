use crate::relative_url::RelativeUrl;
use crate::session::Session;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Tlx {}

#[derive(Clone, Debug, Deserialize)]
struct TlxTotalDataRoot {
    pub result: i32,
    pub obj: TlxTotalData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxTotalData {
    #[serde(alias = "edischargeToday")]
    pub energy_discharge_today: String,
    #[serde(alias = "edischargeTotal")]
    pub energy_discharge_total: String,
    #[serde(alias = "elocalLoadToday")]
    pub energy_local_load_today: String,
    #[serde(alias = "elocalLoadTotal")]
    pub energy_local_load_total: String,
    #[serde(alias = "epvToday")]
    pub energy_pv_today: String,
    #[serde(alias = "epvTotal")]
    pub energy_pv_total: String,
    #[serde(alias = "eselfToday")]
    pub energy_self_today: String,
    #[serde(alias = "eselfTotal")]
    pub energy_self_total: String,
    #[serde(alias = "etoGridToday")]
    pub energy_to_grid_today: String,
    #[serde(alias = "etogridTotal")]
    pub energy_to_grid_total: String,
    #[serde(alias = "gridPowerToday")]
    pub grid_power_today: String,
    #[serde(alias = "gridPowerTotal")]
    pub grid_power_total: String,
    #[serde(alias = "isParallel")]
    pub is_parallel: String,
    #[serde(alias = "outEnergyToday")]
    pub out_energy_today: String,
    #[serde(alias = "outEnergyTotal")]
    pub out_energy_total: String,
    #[serde(alias = "photovoltaicRevenueToday")]
    pub photovoltaic_revenue_today: String,
    #[serde(alias = "photovoltaicRevenueTotal")]
    pub photovoltaic_revenue_total: String,
    pub unit: String,
}

impl Tlx {
    pub async fn total_data(
        session: &mut Session,
        plant_id: &str,
        tlx_serial_number: &str,
    ) -> Result<TlxTotalData, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::TlxTotalData.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut query: HashMap<&str, &str> = HashMap::new();
        query.insert("plantId", plant_id);

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("tlxSn", tlx_serial_number);

        let response = session
            .post_message_return_response::<TlxTotalDataRoot>(url, Some(params), Some(query))
            .await;

        match response {
            Ok(res) => {
                if res.result == 1 {
                    let data = res.obj.clone();

                    Ok(data)
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }
}
