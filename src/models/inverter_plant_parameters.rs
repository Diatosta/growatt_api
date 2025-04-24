pub enum Current {
    Mppt1,
    Mppt2,
    Mppt3,
    String1,
    String2,
    String3,
    String4,
    String5,
    String6,
    String7,
    String8,
}

impl Current {
    pub fn as_str(&self) -> &'static str {
        match self {
            Current::Mppt1 => "IPV1",
            Current::Mppt2 => "IPV2",
            Current::Mppt3 => "IPV3",
            Current::String1 => "cString1",
            Current::String2 => "cString2",
            Current::String3 => "cString3",
            Current::String4 => "cString4",
            Current::String5 => "cString5",
            Current::String6 => "cString6",
            Current::String7 => "cString7",
            Current::String8 => "cString8",
        }
    }
}

pub enum Power {
    MpptPower,
    Pac,
    Power1,
    Power2,
    Power3,
    RPhasePower,
    SPhasePower,
    TPhasePower,
    Energy,
}

impl Power {
    pub fn as_str(&self) -> &'static str {
        match self {
            Power::MpptPower => "ppv",
            Power::Pac => "pac",
            Power::Power1 => "ppv1",
            Power::Power2 => "ppv2",
            Power::Power3 => "ppv3",
            Power::RPhasePower => "pacr",
            Power::SPhasePower => "pacs",
            Power::TPhasePower => "pact",
            Power::Energy => "energy",
        }
    }
}

pub enum Voltage {
    Vac1,
    Mppt1,
    Mppt2,
    Mppt3,
    String1,
    String2,
    String3,
    String4,
    String5,
    String6,
    String7,
    String8,
}

impl Voltage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Voltage::Vac1 => "VAC1",
            Voltage::Mppt1 => "VPV1",
            Voltage::Mppt2 => "VPV2",
            Voltage::Mppt3 => "VPV3",
            Voltage::String1 => "vString1",
            Voltage::String2 => "vString2",
            Voltage::String3 => "vString3",
            Voltage::String4 => "vString4",
            Voltage::String5 => "vString5",
            Voltage::String6 => "vString6",
            Voltage::String7 => "vString7",
            Voltage::String8 => "vString8",
        }
    }
}
