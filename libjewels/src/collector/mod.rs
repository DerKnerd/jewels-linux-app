use serde::Serialize;
use std::option::Option;

mod detector;
mod sender;

pub use detector::collect_device_info;
pub use sender::send_device_data;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Drive {
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub size: f64,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Cpu {
    pub manufacturer: String,
    pub model: String,
    pub speed: f64,
    pub cores: i32,
    pub threads: i32,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Bios {
    pub manufacturer: String,
    pub version: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Mainboard {
    pub manufacturer: String,
    pub version: String,
    pub model: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Kernel {
    pub version: String,
    pub architecture: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct OperatingSystem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Device {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    pub model: String,
    pub manufacturer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<OperatingSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Cpu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<Bios>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainboard: Option<Mainboard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<Kernel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<Drive>>,
}
