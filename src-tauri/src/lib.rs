// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use cpal::platform::{Device, Host};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::HostId;
use log;
use once_cell::sync::OnceCell;
use serde::Serialize;
use serde_json;
use std::sync::Mutex;

// Locking the whole device config state minimizes cheese-moving
struct DeviceConfig {
    devices: Vec<FftunerDevice>,
    selected_device: Option<usize>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FftunerDevice {
    name: String,
    host_name: String,
    selector_id: String,

    #[serde(skip)]
    cpal_device: Device,
}

impl DeviceConfig {
    fn new() -> Self {
        Self {
            devices: Vec::new(),
            selected_device: None,
        }
    }

    fn set_devices(&mut self, devices: Vec<FftunerDevice>) {
        self.devices = devices;

        // Maintain consistency of any selected indices
        if let Some(selected_index) = self.selected_device {
            let new_selected_index = self
                .devices
                .iter()
                .position(|device| device.selector_id == self.devices[selected_index].selector_id);

            match new_selected_index {
                Some(index) => self.selected_device = Some(index),
                // TODO: create device drop event in this case
                None => self.selected_device = None,
            }
        }
    }

    fn get_selected_device(&self) -> Option<FftunerDevice> {
        match self.selected_device {
            Some(index) => Some(self.devices[index].clone()),
            None => None,
        }
    }

    fn set_selected_index(&mut self, index: usize) {
        if index >= self.devices.len() {
            self.selected_device = None;
        } else {
            self.selected_device = Some(index);
        }
    }
}

static DEVICE_CONFIG: OnceCell<Mutex<DeviceConfig>> = OnceCell::new();

impl FftunerDevice {
    fn new(host_id: HostId, device: Device, name: &str) -> Self {
        let device_name = device.name().unwrap_or("Unknown device".to_string());

        Self {
            host_name: host_id.name().to_string(),
            selector_id: Self::create_id(host_id, name, device_name.to_string()),
            cpal_device: device,
            name: name.to_string(),
        }
    }

    fn create_id(host_id: HostId, name: &str, cpal_name: String) -> String {
        format!("{}:{}:{}", host_id.name(), name, cpal_name)
    }
}

#[tauri::command]
fn get_devices() -> String {
    let host = cpal::default_host();
    let host_id = host.id();

    let devices = host.devices().unwrap();
    let devices = devices
        .filter_map(|device| {
            let name = device.name().ok()?;
            Some(FftunerDevice::new(host_id, device, &name))
        })
        .collect::<Vec<_>>();

    let devs_ser = serde_json::to_string(&devices).unwrap();

    log::info!("Found {} devices on the backend", devices.len());
    // log::info!("Serialized devices: {}", devs_ser);

    let mut device_config = DEVICE_CONFIG.get().unwrap().lock().unwrap();
    device_config.set_devices(devices);

    devs_ser
}

#[tauri::command]
fn set_device(index: usize) {
    let mut device_config = DEVICE_CONFIG.get().unwrap().lock().unwrap();
    device_config.set_selected_index(index);
    log::info!(
        "Selected device: {:?}",
        device_config.get_selected_device().unwrap().name
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    DEVICE_CONFIG.get_or_init(|| Mutex::new(DeviceConfig::new()));
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_devices, set_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
