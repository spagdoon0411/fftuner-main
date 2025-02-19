// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use cpal::platform::{Device, HostId};
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Listener};

// Packages devices with hosts
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
struct FftunerDevice {
    selector_id: String,
    name: String,
    #[serde(serialize_with = "serialize_host_id")]
    host_id: HostId,
    #[serde(skip)]
    device: Device,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeviceConfiguration {
    devices: Vec<FftunerDevice>,
    selected: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct Selection {
    selector_id: String,
}

impl FftunerDevice {
    fn new(host_id: HostId, device: Device, name: String) -> Self {
        FftunerDevice {
            selector_id: create_device_id(host_id, name.as_str()),
            name,
            host_id,
            device,
        }
    }
}

fn serialize_host_id<S>(host_id: &HostId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(host_id.name())
}

// TODO: use some deeper sense of equality beyond hosts and names. cpal doesn't
// expose some underlying device ID; this might involve overriding
impl PartialEq for FftunerDevice {
    fn eq(&self, other: &Self) -> bool {
        (self.host_id == other.host_id) && (self.name == other.name)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Hash for FftunerDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.host_id.hash(state);
        self.name.hash(state);
    }
}

// TODO: we should be using a "deeper" ID but cpal doesn't expose this
fn create_device_id(host_id: HostId, name: &str) -> String {
    format!("{:?} {}", host_id, name)
}

impl Eq for FftunerDevice {}

#[tauri::command]
fn print_input_devices() {
    let devices = get_input_devices();
    for device in devices {
        log::info!(
            "Device: {} on host {:?}",
            device.name,
            device.host_id.name()
        );
    }
}

// TODO: should you trigger a refresh event in every situation
// where you access devices? (Then the frontend simply responds
// through an event callback?)
//
// This prevents having to commit crimes with statics and mutex locks.
// Generally interior mutability bad?

/// Creates a Tauri event announcing device updates.

fn jsonify_list<T>(things: T) -> String
where
    T: IntoIterator,
    T::Item: Serialize,
{
    let things_json = things
        .into_iter()
        .map(|t| serde_json::to_string(&t).unwrap())
        .collect::<Vec<String>>()
        .join(",");

    format!("[{things_json}]")
}

fn query_devices() -> String {
    let devices = get_input_devices();
}

fn announce_dev_config() {
    match APP_HANDLE.lock().unwrap().get() {
        Some(app) => app.emit("refresh_devices", "").unwrap(),
        None => log::warn!("announce_device_refresh called before app handle was set"),
    }
}

#[tauri::command]
fn request_device_select(device_id: String) {
    let devices = DEVICES
        .lock()
        .unwrap()
        .get_or_init(|| get_input_devices())
        .clone();

    let selected_device = devices
        .iter()
        .find(|device| device.selector_id == device_id);

    match selected_device {
        Some(device) => {
            SELECTED_DEVICE.lock().unwrap().replace(device.clone());
        }
        None => {
            SELECTED_DEVICE.lock().unwrap().take();
        }
    }

    announce_dev_config();
}

fn drop_selected_device() {
    SELECTED_DEVICE.lock().unwrap().take();
    announce_dev_config();
}

fn get_input_devices() -> HashSet<FftunerDevice> {
    // TODO: does a polling mechanism need to run this?
    // Aggregate all hosts
    let hosts = cpal::available_hosts();
    let mut devices: HashSet<FftunerDevice> = HashSet::new();
    let mut num_unknown = 0;

    for host_id in hosts.iter() {
        let host = cpal::host_from_id(*host_id).unwrap();
        let cpal_devices = match host.input_devices() {
            Ok(devices) => devices,
            Err(err) => {
                log::warn!(
                    "Error getting input devices for host {:?}: {:?}",
                    host_id.name(),
                    err
                );
                continue;
            }
        };

        for device in cpal_devices {
            let name = match device.name() {
                Ok(name) => name,
                Err(_err) => {
                    num_unknown += 1;
                    log::warn!("Error getting device name");
                    format!("Unknown device {num_unknown}").to_string()
                }
            };

            devices.insert(FftunerDevice::new(*host_id, device, name));
        }
    }

    devices
}

static APP_HANDLE: Mutex<OnceCell<AppHandle>> = Mutex::new(OnceCell::new());
static SELECTED_DEVICE: Mutex<Option<FftunerDevice>> = Mutex::new(None);
static DEVICES: Mutex<OnceCell<HashSet<FftunerDevice>>> = Mutex::new(OnceCell::new());

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE
                .lock()
                .unwrap()
                .set(app.handle().clone())
                .unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![print_input_devices, query_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
