pub enum Current {
    MMPT1,
    MPPT2,
    MPPT3,
    STRING1,
    STRING2,
    STRING3,
    STRING4,
    STRING5,
    STRING6,
    STRING7,
    STRING8,
}

impl Current {
    pub fn as_str(&self) -> &'static str {
        match self {
            Current::MMPT1 => "MMPT1",
            Current::MPPT2 => "MPPT2",
            Current::MPPT3 => "MPPT3",
            Current::STRING1 => "STRING1",
            Current::STRING2 => "STRING2",
            Current::STRING3 => "STRING3",
            Current::STRING4 => "STRING4",
            Current::STRING5 => "STRING5",
            Current::STRING6 => "STRING6",
            Current::STRING7 => "STRING7",
            Current::STRING8 => "STRING8",
        }
    }
}

pub enum Power {
    MPPT_POWER,
    PAC,
    POWER1,
    POWER2,
    POWER3,
    R_PHASE_POWER,
    S_PHASE_POWER,
    T_PHASE_POWER,
}

impl Power {
    pub fn as_str(&self) -> &'static str {
        match self {
            Power::MPPT_POWER => "MPPT_POWER",
            Power::PAC => "PAC",
            Power::POWER1 => "POWER1",
            Power::POWER2 => "POWER2",
            Power::POWER3 => "POWER3",
            Power::R_PHASE_POWER => "R_PHASE_POWER",
            Power::S_PHASE_POWER => "S_PHASE_POWER",
            Power::T_PHASE_POWER => "T_PHASE_POWER",
        }
    }
}

pub enum Voltage {
    VAC1,
    MPPT1,
    MPPT2,
    MPPT3,
    STRING1,
    STRING2,
    STRING3,
    STRING4,
    STRING5,
    STRING6,
    STRING7,
    STRING8,
}

impl Voltage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Voltage::VAC1 => "VAC1",
            Voltage::MPPT1 => "MPPT1",
            Voltage::MPPT2 => "MPPT2",
            Voltage::MPPT3 => "MPPT3",
            Voltage::STRING1 => "STRING1",
            Voltage::STRING2 => "STRING2",
            Voltage::STRING3 => "STRING3",
            Voltage::STRING4 => "STRING4",
            Voltage::STRING5 => "STRING5",
            Voltage::STRING6 => "STRING6",
            Voltage::STRING7 => "STRING7",
            Voltage::STRING8 => "STRING8",
        }
    }
}
