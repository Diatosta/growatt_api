use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Offset, TimeZone};
use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn deserialize_option_vec_f32<'de, D>(deserializer: D) -> Result<Option<Vec<f32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_vec: Option<Vec<Option<f32>>> = Option::deserialize(deserializer)?;
    Ok(opt_vec.map(|vec| vec.into_iter().map(|v| v.unwrap_or(0.0)).collect()))
}

pub fn deserialize_option_vec_i32<'de, D>(deserializer: D) -> Result<Option<Vec<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_vec: Option<Vec<Option<i32>>> = Option::deserialize(deserializer)?;
    Ok(opt_vec.map(|vec| vec.into_iter().map(|v| v.unwrap_or(0)).collect()))
}

pub fn deserialize_datetime_to_china_timezone<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let format = "%Y-%m-%d %H:%M:%S";
    let china_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // +8 hours

    let datetime_string = String::deserialize(deserializer)?;
    let datetime = NaiveDateTime::parse_from_str(&datetime_string, format)
        .map_err(serde::de::Error::custom)?;
    let datetime = china_offset
        .from_local_datetime(&datetime)
        .single()
        .ok_or_else(|| serde::de::Error::custom("Invalid local datetime for China timezone"))?;
    Ok(datetime)
}

pub fn deserialize_datetime_to_local_timezone<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let format = "%Y-%m-%d %H:%M:%S";
    let local_offset = Local::now().offset().fix();

    let datetime_string = String::deserialize(deserializer)?;
    let datetime = NaiveDateTime::parse_from_str(&datetime_string, format)
        .map_err(serde::de::Error::custom)?;
    let datetime = local_offset
        .from_local_datetime(&datetime)
        .single()
        .ok_or_else(|| serde::de::Error::custom("Invalid local datetime for local timezone"))?;
    Ok(datetime)
}

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}
