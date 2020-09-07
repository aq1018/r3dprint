pub mod parser;

use std::collections::HashMap;

type ByteSize = u32;
type ExtruderNumber = u8;
type Temperature = f32;
type Position = f32;
type LineNumber = u32;

#[derive(Debug)]
pub enum Message {
    Ok,
    Error,
    Busy,
    Wait,
    Action(Action),
    Resend(Resend),
    SDPrintStatus(SDPrintStatus),
    SDFileOpened(SDFileOpened),
    TemperatureReport(TemperatureReport),
    PositionReport(PositionReport),
    FirmwareReport(FirmwareReport),
}

#[derive(Debug)]
pub enum Action {
    Cancel,
    Pause,
    Paused,
    Resume,
    Resumed,
    Disconnect,
    Other(String),
}

#[derive(Debug)]
pub struct SDPrintStatus {
    current: ByteSize,
    total: ByteSize,
}

#[derive(Debug)]
pub struct SDFileOpened {
    name: String,
    size: ByteSize,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ExtruderID {
    Current,
    Numbered(ExtruderNumber),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum HeatSource {
    Extruder(ExtruderID),
    Bed,
    Sensor,
    Ambient,
}

#[derive(Debug)]
pub struct Measurement {
    actual: Temperature,
    target: Option<Temperature>,
}

#[derive(Debug)]
pub struct TemperatureReport {
    data: HashMap<HeatSource, Measurement>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    Extruder(ExtruderID),
}

#[derive(Debug)]
pub struct PositionReport {
    data: HashMap<Axis, Position>,
}

#[derive(Debug)]
pub struct FirmwareReport {
    uuid: String,
    firmware_name: String,
    source_code_url: String,
    protocol_version: String,
    machine_type: String,
    extruder_count: ExtruderNumber,
    capabilities: Capability,
}

bitflags! {
    struct Capability: u32 {
        const AUTOREPORT_TEMP = 0b00000001;
        const AUTOREPORT_SD_STATUS = 0b00000010;
        const EMERGENCY_PARSER = 0b00000100;
    }
}

#[derive(Debug)]
pub struct Resend {
    line_number: LineNumber,
}
