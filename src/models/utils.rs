use serde::{Deserialize, Deserializer};

pub fn deserialize_option_vec_f32<'de, D>(deserializer: D) -> Result<Option<Vec<f32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_vec: Option<Vec<Option<f32>>> = Option::deserialize(deserializer)?;
    Ok(opt_vec.map(|vec| vec.into_iter().map(|v| v.unwrap_or(0.0)).collect()))
}
