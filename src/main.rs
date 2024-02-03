use std::collections::HashMap;
use std::env;
use config::Config;
use displayz::{query_displays, refresh, Resolution};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Couldn't find config.toml");

    let config = config_file.try_deserialize::<HashMap<String, String>>()
        .expect("Couldn't deserialize config content");

    if &args.len() < &2 {
        println!("Resolution argument is not passed");
        return
    }

    let arg = &args[1];
    let new_resolution = if arg == "custom" {
        let custom_resolution_str = config.get("custom").expect("No custom resolution is set");
        parse_to_resolution(custom_resolution_str.to_owned())
    } else {
        let default_resolution_str = config.get("default").expect("No default resolution is set");
        parse_to_resolution(default_resolution_str.to_owned())
    };
    let current_resolution = get_current_resolution();
    if new_resolution != current_resolution {
        set_resolution(new_resolution)
    } else {
        println!("New resolution is the same!")
    }
}

fn parse_to_resolution(input: String) -> Resolution {
    let parts: Vec<&str> = input.split('x').collect();

    if let [width, height] = parts.as_slice() {
        let width: u32 = width.parse().unwrap();
        let height: u32 = height.parse().unwrap();
        Resolution::new(width, height)
    } else {
        println!("Invalid input format");
        Resolution::new(0,0)
    }
}

pub fn get_current_resolution() -> Resolution {
    let displays = query_displays().expect("No primary display found!");
    return match displays.primary().settings() {
        Some(settings) => {
            (*settings).borrow().resolution
        }
        None => Resolution::new(0, 0)
    }
}

pub fn set_resolution(new_resolution: Resolution) {
    let displays = query_displays().expect("No primary display found!");
    match displays.primary().settings() {
        Some(settings) => {
            (*settings).borrow_mut().resolution = new_resolution;
        }
        None => {
            println!("Primary display has no settings!");
        }
    }
    displays.primary().apply().expect("Can't apply new settings to display");
    refresh().expect("Can't apply new settings to display");
}