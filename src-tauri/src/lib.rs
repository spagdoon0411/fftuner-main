// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use cpal::platform::{Device, Host};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::HostId;
use log;
use serde::Serialize;
use serde_json;

static mut SELECTED_DEVICE: Option<&FftunerDevice> = None;
static mut DEVICES: Vec<FftunerDevice> = Vec::new();

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FftunerDevice {
    name: String,
    host_name: String,
    selector_id: String,

    #[serde(skip)]
    cpal_device: Device,
}

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

    unsafe {
        DEVICES = devices;
    }

    devs_ser
}

#[tauri::command]
fn set_device(selector_id: String) {
    let device = unsafe {
        DEVICES
            .iter()
            .find(|d| d.selector_id == selector_id)
            .unwrap()
    };

    unsafe {
        SELECTED_DEVICE = Some(device);
        log::info!(
            "Selected device with id: {}",
            SELECTED_DEVICE.as_ref().unwrap().selector_id
        );
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_devices, set_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
