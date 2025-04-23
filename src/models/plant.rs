use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Plant {
    pub id: String,
    #[serde(alias = "plantName")]
    pub plant_name: String,
    #[serde(alias = "timezone")]
    pub time_zone: String,
}
