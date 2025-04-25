use crate::relative_url::RelativeUrl;
use crate::session::Session;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WeatherBasic {
    pub admin_area: String,
    #[serde(alias = "cnty")]
    pub country: String,
    #[serde(alias = "lat")]
    pub latitude: Option<String>,
    #[serde(alias = "lon")]
    pub longitude: Option<String>,
    pub location: String,
    pub parent_city: String,
    #[serde(alias = "sr")]
    pub sunrise: String,
    #[serde(alias = "ss")]
    pub sunset: String,
    #[serde(alias = "tz")]
    pub timezone: Option<String>,
    #[serde(alias = "toDay")]
    pub today: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeatherNow {
    pub cloud: String,
    #[serde(alias = "cond_code")]
    pub condition_code: String,
    #[serde(alias = "cond_txt")]
    pub condition_text: String,
    pub fl: String,
    #[serde(alias = "hum")]
    pub humidity: String,
    #[serde(alias = "pcpn")]
    pub precipitation: String,
    #[serde(alias = "pres")]
    pub pressure: String,
    #[serde(alias = "tmp")]
    pub temperature: String,
    #[serde(alias = "wind_deg")]
    pub wind_degrees: String,
    #[serde(alias = "wind_dir")]
    pub wind_direction: String,
    #[serde(alias = "wind_sc")]
    pub wind_scale: String,
    #[serde(alias = "wind_spd")]
    pub wind_speed: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeatherUpdate {
    #[serde(alias = "loc")]
    pub location: String,
    pub utc: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeatherHeData {
    pub basic: WeatherBasic,
    pub now: WeatherNow,
    pub status: String,
    pub update: WeatherUpdate,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeatherData {
    #[serde(alias = "HeWeather6")]
    pub weather_list: Vec<WeatherHeData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Weather {
    pub city: String,
    pub data: WeatherData,
    #[serde(alias = "dataStr")]
    pub data_string: String,
    pub radiant: String,
    pub week: Option<String>,
    #[serde(alias = "tempType")]
    pub temp_type: i32,
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherResponse {
    pub result: i32,
    pub obj: Weather,
}

impl Weather {
    pub async fn by_plant(session: &mut Session, plant_id: &str) -> Result<Weather, StatusCode> {
        let mut url = session
            .api_base_url
            .join(RelativeUrl::WeatherByPlantId.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        url.query_pairs_mut().append_pair("plantId", plant_id);

        let response = session
            .post_message_return_response::<WeatherResponse>(url, None, None)
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
