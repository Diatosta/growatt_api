pub fn get_sim_signal_text(sim_signal: i32, device_type_indicate: &str) -> String {
    if device_type_indicate == "11" || device_type_indicate == "16" {
        if (-50..=0).contains(&sim_signal) {
            format!("Excellent ({})", sim_signal)
        } else if (-75..=-51).contains(&sim_signal) {
            format!("Good ({})", sim_signal)
        } else if (-113..=-76).contains(&sim_signal) {
            format!("Poor ({})", sim_signal)
        } else {
            "No".to_string()
        }
    } else if (-70..=-51).contains(&sim_signal) {
        format!("Excellent ({})", sim_signal)
    } else if (-85..=-71).contains(&sim_signal) {
        format!("Good ({})", sim_signal)
    } else if (-113..=-86).contains(&sim_signal) {
        format!("Poor ({})", sim_signal)
    } else {
        "No".to_string()
    }
}
