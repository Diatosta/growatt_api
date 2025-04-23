use crate::relative_url::RelativeUrl;
use crate::session::Session;
use reqwest::StatusCode;
use serde::Deserialize;

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

        session
            .get_message_return_response(url, StatusCode::OK)
            .await
    }
}
