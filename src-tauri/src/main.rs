use serde::Deserialize;
use std::{time::Duration, sync::{atomic::AtomicBool, Arc, Mutex}, collections::HashMap};
use tauri::{Manager, Window};
use nt::{NetworkTables, EntryValue};
use tokio::runtime::Runtime;
use nt::EntryValue::{Boolean as ntBool, Double as ntDouble, String as ntString};
use tokio::time::timeout;
use tokio::sync::oneshot;

const TABLE_PREFIX: &str = "/data/";
lazy_static::lazy_static! {
    pub static ref IP: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    // pub static ref TABLE_PREFIX: String = String::from("/data/");
}

#[derive(Clone, serde::Serialize, Debug)]
struct PassedData {
    flywheel_rpm: f64,
    flywheel_temp: f64,

    left_pos: f64,
    right_pos: f64,

    rotation_2d: f64,

    // motor voltages
    left_front_voltage: f64,
    left_back_voltage: f64,

    right_front_voltage: f64,
    right_back_voltage: f64,

    gyro_turn_rate: f64,

    x_speed: f64,
    z_rotation: f64,

    compressor_current: f64,
    compressor_enabled: bool,

    forward_solenoid: bool,
    reverse_solenoid: bool,

    intake_alive: bool,
    intake_power: f64,
    intake_state: bool,

    // time in ms
    // TODO
    unix_time: String,
}

macro_rules! getbool {
    ($map:expr, $key: expr) => {
        if let Some(v) = $map.get(&(TABLE_PREFIX.to_owned() + $key)) {
            match v {
                ntBool(v) => *v,
                _ => false,
            }
        } else {
            false
        }
    };
}

macro_rules! getdouble {
    ($map:expr, $key: expr) => {
        if let Some(v) = $map.get(&(TABLE_PREFIX.to_owned() + $key)) {
            match v {
                ntDouble(v) => *v,
                _ => 0.0,
            }
        } else {
            0.0
        }
    };
}

macro_rules! getstring {
    ($map:expr, $key: expr) => {
        if let Some(v) = $map.get(&(TABLE_PREFIX.to_owned() + $key)) {
            match v {
                ntString(v) => v.to_string(),
                _ => String::default(),
            }
        } else {
            String::default()
        }
    };
}

impl PassedData {
    pub fn get_from_table(client: &Result<NetworkTables<nt::Client>, nt::error::Error>) -> Result<Self, ()> {
        if let Ok(v) = client {
            let v: HashMap<String, EntryValue> = v.entries().iter().map(|x| (x.1.name.clone(), x.1.value.clone())).collect();
            return Ok(Self {
                flywheel_rpm: getdouble!(v, "flywheel_rpm"),
                flywheel_temp: getdouble!(v, "flywheel_temp"),
                left_pos: getdouble!(v, "left_pos"),
                right_pos: getdouble!(v, "right_pos"),
                rotation_2d: getdouble!(v, "rotation_2d"),
                left_front_voltage: getdouble!(v, "left_front_voltage"),
                left_back_voltage: getdouble!(v, "left_back_voltage"),
                right_front_voltage: getdouble!(v, "right_front_voltage"),
                right_back_voltage: getdouble!(v, "right_back_voltage"),
                gyro_turn_rate: getdouble!(v, "gyro_turn_rate"),
                x_speed: getdouble!(v, "x_speed"),
                z_rotation: getdouble!(v, "z_rotation"),
                compressor_current: getdouble!(v, "compressor_current"),
                compressor_enabled: getbool!(v, "compressor_enabled"),
                intake_alive: getbool!(v, "intake_alive"),
                intake_power: getdouble!(v, "intake_power"),
                intake_state: getbool!(v, "intake_state"),
                forward_solenoid: getbool!(v, "forward_solenoid"),
                reverse_solenoid: getbool!(v, "reverse_solenoid"),
                unix_time: getstring!(v, "unix_time"),
            })
        }
        Err(())
    }
}

#[derive(Clone, serde::Serialize, Deserialize, Debug)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_process(window: Window, ip: String) {
    std::thread::spawn(move || {
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
                let t = timeout(Duration::from_millis(500), NetworkTables::connect(&(IP.lock().unwrap()), "seanboard")).await;
                if let Ok(t) = t {
                    let t = PassedData::get_from_table(&t);
                    std::thread::sleep(Duration::from_millis(200));
                    if let Ok(v) = t {
                        if window
                            .emit(
                                "new_data",
                                v
                            ).is_err() {
                                break;
                            }
                    }
                }
            }
        });
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();

            // listen to the `event-name` (emitted on the `main` window)
            let id = main_window.listen("set_ip", |event| {
                println!("got window event-name with payload {:?}", event.payload());
                // check ip here
                if let Some(v) = event.payload() {
                    if let Ok(v) = serde_json::from_str::<Payload>(v) {
                        *IP.lock().unwrap() = v.message.to_string();
                        println!("{:?}", v);
                    }
                }
            });
            // unlisten to the event using the `id` returned on the `listen` function
            // an `once` API is also exposed on the `Window` struct
            // main_window.unlisten(id);

            // emit the `event-name` event to the `main` window
            main_window.emit("event-", Payload { message: "Tauri is awesome!".into() }).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
