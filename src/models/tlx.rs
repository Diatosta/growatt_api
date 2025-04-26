use crate::helpers;
use crate::relative_url::RelativeUrl;
use crate::session::Session;
use chrono::NaiveDate;
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
    pub energy_load_consumption_today: String,
    #[serde(alias = "elocalLoadTotal")]
    pub energy_load_consumption_total: String,
    #[serde(alias = "epvToday")]
    pub energy_photovoltaic_today: String,
    #[serde(alias = "epvTotal")]
    pub energy_photovoltaic_total: String,
    #[serde(alias = "eselfToday")]
    pub energy_self_today: String,
    #[serde(alias = "eselfTotal")]
    pub energy_self_total: String,
    #[serde(alias = "etoGridToday")]
    pub energy_to_grid_today: String,
    #[serde(alias = "etogridTotal")]
    pub energy_to_grid_total: String,
    #[serde(alias = "gridPowerToday")]
    pub energy_from_grid_today: String,
    #[serde(alias = "gridPowerTotal")]
    pub energy_from_grid_total: String,
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

#[derive(Clone, Debug, Deserialize)]
struct TlxBatteryChartDataRoot {
    pub result: i32,
    pub obj: TlxBatteryChartData,
}

#[derive(Clone, Debug, Deserialize)]
struct TlxBatteryChartDataSocChart {
    #[serde(alias = "bdc1Soc")]
    pub bdc1_soc: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxBatteryChartData {
    pub date: NaiveDate,
    #[serde(alias = "bcdStatus")]
    pub bcd_status: String,
    #[serde(alias = "cdsTitle")]
    pub cds_title: Vec<NaiveDate>,
    #[serde(alias = "wBatteryType")]
    pub battery_type: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TlxEnergyDayChartDataRoot {
    pub result: i32,
    pub obj: TlxEnergyDayChartData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyDayChartDataCharts {
    #[serde(alias = "bdc1ChargePower", with = "serde_arrays")]
    pub battery_charging_power: [Option<f32>; 288],
    #[serde(alias = "bdc1DischargePower", with = "serde_arrays")]
    pub battery_discharging_power: [Option<f32>; 288],
    #[serde(alias = "elocalLoad", with = "serde_arrays")]
    pub load_consumption: [Option<f32>; 288],
    #[serde(alias = "pacToGrid", with = "serde_arrays")]
    pub exported_to_grid: [Option<f32>; 288],
    #[serde(alias = "pacToUser", with = "serde_arrays")]
    pub imported_from_grid: [Option<f32>; 288],
    #[serde(alias = "pex", with = "serde_arrays")]
    // Unknown
    pub pex: [Option<f32>; 288],
    #[serde(alias = "ppv", with = "serde_arrays")]
    pub photovoltaic_output: [Option<f32>; 288],
    #[serde(alias = "pself", with = "serde_arrays")]
    // Unknown
    pub pself: [Option<f32>; 288],
    #[serde(alias = "sysOut", with = "serde_arrays")]
    // Unknown
    pub sys_out: [Option<f32>; 288],
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyDayChartData {
    pub charts: TlxEnergyDayChartDataCharts,
    #[serde(
        alias = "eCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced: f32,
    #[serde(
        alias = "eAcCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_exported: f32,
    #[serde(
        alias = "eChargeToday1",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_consumed1: f32,
    #[serde(
        alias = "elocalLoad",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_total: f32,
    #[serde(
        alias = "etouser",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_from_grid: f32,
    #[serde(
        alias = "eChargeToday2",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_self: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct TlxEnergyMonthChartDataRoot {
    pub result: i32,
    pub obj: TlxEnergyMonthChartData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyMonthChartDataCharts {
    #[serde(alias = "eCharge")]
    pub battery_charging_power: Vec<Option<f32>>,
    #[serde(alias = "eDischarge")]
    pub battery_discharging_power: Vec<Option<f32>>,
    #[serde(alias = "elocalLoad")]
    pub load_consumption: Vec<Option<f32>>,
    #[serde(alias = "pacToGrid")]
    pub exported_to_grid: Vec<Option<f32>>,
    #[serde(alias = "pacToUser")]
    pub imported_from_grid: Vec<Option<f32>>,
    #[serde(alias = "pex")]
    // Unknown
    pub pex: Vec<Option<f32>>,
    #[serde(alias = "ppv")]
    pub photovoltaic_output: Vec<Option<f32>>,
    #[serde(alias = "pself")]
    // Unknown
    pub pself: Vec<Option<f32>>,
    #[serde(alias = "sysOut")]
    // Unknown
    pub sys_out: Vec<Option<f32>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyMonthChartData {
    pub charts: TlxEnergyMonthChartDataCharts,
    #[serde(
        alias = "eCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced: f32,
    #[serde(
        alias = "eAcCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_exported: f32,
    #[serde(
        alias = "eChargeToday1",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_consumed1: f32,
    #[serde(
        alias = "elocalLoad",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_total: f32,
    #[serde(
        alias = "etouser",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_from_grid: f32,
    #[serde(
        alias = "eChargeToday2",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_self: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct TlxEnergyYearChartDataRoot {
    pub result: i32,
    pub obj: TlxEnergyYearChartData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyYearChartDataCharts {
    #[serde(alias = "eCharge", with = "serde_arrays")]
    pub battery_charging_power: [Option<f32>; 12],
    #[serde(alias = "eDischarge", with = "serde_arrays")]
    pub battery_discharging_power: [Option<f32>; 12],
    #[serde(alias = "elocalLoad", with = "serde_arrays")]
    pub load_consumption: [Option<f32>; 12],
    #[serde(alias = "pacToGrid", with = "serde_arrays")]
    pub exported_to_grid: [Option<f32>; 12],
    #[serde(alias = "pacToUser", with = "serde_arrays")]
    pub imported_from_grid: [Option<f32>; 12],
    #[serde(alias = "pex", with = "serde_arrays")]
    // Unknown
    pub pex: [Option<f32>; 12],
    #[serde(alias = "ppv", with = "serde_arrays")]
    pub photovoltaic_output: [Option<f32>; 12],
    #[serde(alias = "pself", with = "serde_arrays")]
    // Unknown
    pub pself: [Option<f32>; 12],
    #[serde(alias = "sysOut", with = "serde_arrays")]
    // Unknown
    pub sys_out: [Option<f32>; 12],
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyYearChartData {
    pub charts: TlxEnergyYearChartDataCharts,
    #[serde(
        alias = "eCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced: f32,
    #[serde(
        alias = "eAcCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_exported: f32,
    #[serde(
        alias = "eChargeToday1",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_consumed1: f32,
    #[serde(
        alias = "elocalLoad",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_total: f32,
    #[serde(
        alias = "etouser",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_from_grid: f32,
    #[serde(
        alias = "eChargeToday2",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_self: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct TlxEnergyTotalChartDataRoot {
    pub result: i32,
    pub obj: TlxEnergyTotalChartData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyTotalChartDataCharts {
    #[serde(alias = "eCharge")]
    pub battery_charging_power: Vec<Option<f32>>,
    #[serde(alias = "eDischarge")]
    pub battery_discharging_power: Vec<Option<f32>>,
    #[serde(alias = "elocalLoad")]
    pub load_consumption: Vec<Option<f32>>,
    #[serde(alias = "pacToGrid")]
    pub exported_to_grid: Vec<Option<f32>>,
    #[serde(alias = "pacToUser")]
    pub imported_from_grid: Vec<Option<f32>>,
    #[serde(alias = "pex")]
    // Unknown
    pub pex: Vec<Option<f32>>,
    #[serde(alias = "ppv")]
    pub photovoltaic_output: Vec<Option<f32>>,
    #[serde(alias = "pself")]
    // Unknown
    pub pself: Vec<Option<f32>>,
    #[serde(alias = "sysOut")]
    // Unknown
    pub sys_out: Vec<Option<f32>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlxEnergyTotalChartData {
    pub charts: TlxEnergyTotalChartDataCharts,
    #[serde(
        alias = "eCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced: f32,
    #[serde(
        alias = "eAcCharge",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_exported: f32,
    #[serde(
        alias = "eChargeToday1",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_produced_consumed1: f32,
    #[serde(
        alias = "elocalLoad",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_total: f32,
    #[serde(
        alias = "etouser",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_from_grid: f32,
    #[serde(
        alias = "eChargeToday2",
        deserialize_with = "helpers::serde_helper::deserialize_number_from_string"
    )]
    pub energy_consumed_self: f32,
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

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("tlxSn", tlx_serial_number);
        params.insert("plantId", plant_id);

        let response = session
            .post_message_return_response::<TlxTotalDataRoot>(url, Some(params), None)
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

    pub async fn energy_day_chart(
        session: &mut Session,
        plant_id: &str,
        tlx_serial_number: &str,
        date: NaiveDate,
    ) -> Result<TlxEnergyDayChartData, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::TlxEnergyDayChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let date_string = date.format("%Y-%m-%d").to_string();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("tlxSn", tlx_serial_number);
        params.insert("plantId", plant_id);
        params.insert("date", &date_string);

        let response = session
            .post_message_return_response::<TlxEnergyDayChartDataRoot>(url, Some(params), None)
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

    pub async fn energy_month_chart(
        session: &mut Session,
        plant_id: &str,
        tlx_serial_number: &str,
        date: NaiveDate,
    ) -> Result<TlxEnergyMonthChartData, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::TlxEnergyMonthChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let date_string = date.format("%Y-%m").to_string();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("tlxSn", tlx_serial_number);
        params.insert("plantId", plant_id);
        params.insert("date", &date_string);

        let response = session
            .post_message_return_response::<TlxEnergyMonthChartDataRoot>(url, Some(params), None)
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

    pub async fn energy_year_chart(
        session: &mut Session,
        plant_id: &str,
        tlx_serial_number: &str,
        date: NaiveDate,
    ) -> Result<TlxEnergyYearChartData, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::TlxEnergyYearChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let date_string = date.format("%Y").to_string();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("tlxSn", tlx_serial_number);
        params.insert("plantId", plant_id);
        params.insert("year", &date_string);

        let response = session
            .post_message_return_response::<TlxEnergyYearChartDataRoot>(url, Some(params), None)
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

    pub async fn energy_total_chart(
        session: &mut Session,
        plant_id: &str,
        tlx_serial_number: &str,
        date: NaiveDate,
    ) -> Result<TlxEnergyTotalChartData, StatusCode> {
        let url = session
            .api_base_url
            .join(RelativeUrl::TlxEnergyTotalChart.as_str())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let date_string = date.format("%Y").to_string();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("tlxSn", tlx_serial_number);
        params.insert("plantId", plant_id);
        params.insert("year", &date_string);

        let response = session
            .post_message_return_response::<TlxEnergyTotalChartDataRoot>(url, Some(params), None)
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
