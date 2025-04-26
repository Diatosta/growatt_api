use crate::models::device::Device;

pub fn get_device_type_status(device: &Device) -> String {
    match device.device_type_name.as_str() {
        "max" => get_max_device_status(&device.status),
        "storage" => get_storage_device_status(&device.status),
        "mix" => get_mix_device_status(&device.status),
        "pcs" => get_pcs_device_status(&device.status),
        "hps" => get_hps_device_status(&device.status),
        "spa" => get_spa_device_status(&device.status),
        "tlx" => get_tlx_device_status(&device.status),
        "pbd" => get_pbd_device_status(&device.status),
        "eybondInv" => get_eybond_device_status(&device.status),
        "igenInv" => get_igen_device_status(&device.status, device.device_type),
        "inv" => get_mix_device_status(&device.status),
        "pumper" => get_pumper_device_status(&device.status),
        _ => "N/A (possible new device type?".to_string(),
    }
}

fn get_max_device_status(device_status: &str) -> String {
    match device_status {
        "-1" => format!("Lost ({})", device_status),
        "1" => format!("Connection ({})", device_status),
        "2" | "3" => format!("Malfunction  ({})", device_status),
        _ => format!("Wait ({})", device_status),
    }
}

fn get_storage_device_status(device_status: &str) -> String {
    match device_status {
        "-1" => format!("Lost ({})", device_status),
        "0" => format!("Standby ({})", device_status),
        "1" => format!("PV&Grid Supporting Loads  ({})", device_status),
        "2" => format!("Battery Discharging ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Flash ({})", device_status),
        "5" => format!("MPPT charge ({})", device_status),
        "6" => format!("AC charge ({})", device_status),
        "7" => format!("PV&Grid Charging ({})", device_status),
        "8" => format!("PV&Grid Charging+Grid Bypass ({})", device_status),
        "9" => format!("PV Charging+Grid Bypass ({})", device_status),
        "10" => format!("Grid Charging+Grid Bypass ({})", device_status),
        "11" => format!("Grid Bypass ({})", device_status),
        "12" => format!("PV Charging+Loads Supporting ({})", device_status),
        "13" => format!("AC charge and Discharge ({})", device_status),
        "14" => format!("Combine charge and Discharge ({})", device_status),
        _ => format!("Unknown ({})", device_status),
    }
}

fn get_mix_device_status(device_status: &str) -> String {
    match device_status {
        "-1" => format!("Lost ({})", device_status),
        "0" => format!("Standby ({})", device_status),
        "1" => format!("Checking ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Burning ({})", device_status),
        "5" | "6" | "7" | "8" | "9" => format!("Normal ({})", device_status),
        _ => format!("Unknown ({})", device_status),
    }
}

fn get_pcs_device_status(device_status: &str) -> String {
    match device_status {
        "0" => format!("Wait ({})", device_status),
        "1" => format!("Checking ({})", device_status),
        "2" => format!("Normal ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Permanent Fault ({})", device_status),
        "5" => format!("Off Grid ({})", device_status),
        "6" => format!("Single MPPT ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}

fn get_hps_device_status(device_status: &str) -> String {
    match device_status {
        "0" => format!("Wait ({})", device_status),
        "1" => format!("Checking ({})", device_status),
        "2" => format!("On-Grid ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Permanent Fault ({})", device_status),
        "5" => format!("Off Grid ({})", device_status),
        "6" => format!("Single MPPT mode ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}

fn get_spa_device_status(device_status: &str) -> String {
    match device_status {
        "-1" => format!("Lost ({})", device_status),
        "0" => format!("Standby ({})", device_status),
        "1" => format!("Checking ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Burning ({})", device_status),
        "5" | "6" | "7" | "8" | "9" => format!("Normal ({})", device_status),
        _ => format!("Unknown ({})", device_status),
    }
}

fn get_tlx_device_status(device_status: &str) -> String {
    match device_status {
        "0" => format!("Standby ({})", device_status),
        "1" => format!("Normal ({})", device_status),
        "2" => format!("Off Grid ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Burning ({})", device_status),
        "5" | "6" | "7" | "8" => format!("Checking ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}

fn get_pbd_device_status(device_status: &str) -> String {
    match device_status {
        "0" => format!("Standby ({})", device_status),
        "1" => format!("Normal ({})", device_status),
        "2" => format!("Off Grid ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Burning ({})", device_status),
        "5" | "6" | "7" | "8" => format!("Checking ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}

fn get_eybond_device_status(device_status: &str) -> String {
    match device_status {
        "0" => format!("Connection ({})", device_status),
        "2" => format!("Malfunction ({})", device_status),
        "3" => format!("Wait ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}

fn get_igen_device_status(device_status: &str, device_type: i32) -> String {
    if (20..=30).contains(&device_type) {
        match device_status {
            "0" => format!("Wait ({})", device_status),
            "1" => format!("Connection ({})", device_status),
            "3" => format!("Malfunction ({})", device_status),
            _ => format!("Lost ({})", device_status),
        }
    } else {
        match device_status {
            "0" => format!("Wait ({})", device_status),
            "1" | "2" | "3" => format!("Connection ({})", device_status),
            "4" | "5" | "6" | "7" | "8" | "9" => format!("Malfunction ({})", device_status),
            _ => format!("Lost ({})", device_status),
        }
    }
}

fn get_pumper_device_status(device_status: &str) -> String {
    match device_status {
        "1" | "2" | "3" => format!("Normal ({})", device_status),
        "4" => format!("Malfunction ({})", device_status),
        "5" => format!("Wait ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}

fn get_generic_device_status(device_status: &str) -> String {
    match device_status {
        "0" => format!("Wait ({})", device_status),
        "1" => format!("Connection ({})", device_status),
        "2" => format!("Checking ({})", device_status),
        "3" => format!("Malfunction ({})", device_status),
        "4" => format!("Keep ({})", device_status),
        _ => format!("Lost ({})", device_status),
    }
}
