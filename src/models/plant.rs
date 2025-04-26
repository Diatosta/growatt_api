use crate::relative_url::RelativeUrl;
use crate::session::Session;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Plant {
    pub id: String,
    #[serde(alias = "plantName")]
    pub plant_name: String,
    #[serde(alias = "timezone")]
    pub time_zone: String,
}

#[derive(Clone, Debug, Deserialize)]
struct PlantDataRoot {
    pub result: i32,
    pub obj: PlantData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlantData {
    #[serde(rename = "accountName")]
    pub account_name: String,
    pub city: String,
    pub co2: String,
    pub coal: String,
    pub country: String,
    #[serde(rename = "creatDate")]
    pub create_date: String,
    #[serde(rename = "eTotal")]
    pub energy_total: String,
    #[serde(rename = "fixedPowerPrice")]
    pub fixed_power_price: String,
    #[serde(rename = "flatPeriodPrice")]
    pub flat_period_price: String,
    #[serde(rename = "formulaCo2")]
    pub formula_co2: String,
    #[serde(rename = "formulaCoal")]
    pub formula_coal: String,
    #[serde(rename = "formulaMoney")]
    pub formula_money: String,
    #[serde(rename = "formulaTree")]
    pub formula_tree: String,
    pub id: String,
    #[serde(rename = "isShare")]
    pub is_share: String,
    #[serde(rename = "lng")]
    pub longitude: String,
    #[serde(rename = "lat")]
    pub latitude: String,
    #[serde(rename = "locationImg")]
    pub location_img: Option<String>,
    #[serde(rename = "moneyUnit")]
    pub money_unit: String,
    #[serde(rename = "moneyUnitText")]
    pub money_unit_text: String,
    #[serde(rename = "nominalPower")]
    pub nominal_power: String,
    #[serde(rename = "peakPeriodPrice")]
    pub peak_period_price: String,
    #[serde(rename = "plantImg")]
    pub plant_img: Option<String>,
    #[serde(rename = "plantName")]
    pub plant_name: String,
    #[serde(rename = "plantType")]
    pub plant_type: String,
    #[serde(rename = "timezone")]
    pub time_zone: String,
    #[serde(rename = "tree")]
    pub tree: String,
    #[serde(rename = "valleyPeriodPrice")]
    pub valley_period_price: String,
}
impl Plant {
    pub async fn all(session: &mut Session) -> Result<Vec<Plant>, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::PlantList.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        session.get_message_return_response(url).await
    }

    pub async fn plant_data(
        session: &mut Session,
        plant_id: &str,
    ) -> Result<PlantData, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::PlantData.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut query: HashMap<&str, &str> = HashMap::new();
        query.insert("plantId", plant_id);

        let response = session
            .post_message_return_response::<PlantDataRoot>(url, None, Some(query))
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
