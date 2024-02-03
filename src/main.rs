use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use clap::{arg, Parser};
use config::Config;
use displayz::{query_displays, refresh, Resolution};
use log::{info, warn};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Change resolution to default value
    #[arg(short, long)]
    default: bool,

    /// Change resolution to custom value
    #[arg(short, long)]
    custom: bool,
}

fn main() {
    setup_logger().expect("Couldn't setup logger");
    info!("Starting sunshine-resolution.");
    let args = Args::parse();
    let config = get_config();
    if args.default {
        info!("Setting display resolution to default.");
        let default_resolution_str = config.get("default").expect("No default resolution is set");
        change_resolution(default_resolution_str);
    }
    else if args.custom {
        info!("Setting display resolution to custom.");
        let custom_resolution_str = config.get("custom").expect("No custom resolution is set");
        change_resolution(custom_resolution_str);
    } else {
        info!("Injecting sunshine-resolution callbacks to sunshine.conf");
        let sunshine_conf_path = config.get("sunshine_conf").expect("No sunshine.conf path is set");
        inject_to_sunshine_conf(sunshine_conf_path)
    }
    info!("Stopping sunshine-resolution.\n");
}


fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply().unwrap();
    Ok(())
}


fn get_config() -> HashMap<String, String> {
    let config_file = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Couldn't find config.toml");
    config_file.try_deserialize::<HashMap<String, String>>()
        .expect("Couldn't deserialize config content")
}

fn inject_to_sunshine_conf(sunshine_conf_path: &String) {
    let file_path = Path::new(sunshine_conf_path);
    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(&file_path)
        .expect("Can't open starlight-stop.txt");
    let current_dir = env::current_dir().unwrap();
    let result= format!(r#"global_prep_cmd = [
    {{
        "do": "powershell.exe cd '{dir}'; ./sunshine-resolution --custom",
        "undo": "powershell.exe cd '{dir}'; ./sunshine-resolution --default"
    }}]"#, dir = current_dir.display().to_string().replace("\\", "\\\\"));
    file.write(result.as_bytes()).expect("Can't write to starlight-stop.txt");
    file.flush().unwrap();
    info!("Injection is successfully completed!");
}

fn change_resolution(resolution: &String) {
    let new_resolution = parse_to_resolution(resolution.to_owned())
        .expect("Couldn't parse new resolution");
    info!("Desired resolution is {resolution}.");
    let current_resolution = get_current_resolution()
        .expect("Couldn't get current resolution");
    if new_resolution != current_resolution {
        set_resolution(new_resolution)
    } else { warn!("New resolution is the same!. Can't change anything.") }
}

fn parse_to_resolution(input: String) -> Option<Resolution> {
    let parts: Vec<&str> = input.split('x').collect();
    if let [width, height] = parts.as_slice() {
        let width: u32 = width.parse().unwrap();
        let height: u32 = height.parse().unwrap();
        Some(Resolution::new(width, height))
    } else { None }
}

pub fn get_current_resolution() -> Option<Resolution> {
    let displays = query_displays().expect("No primary display found!");
    if let Some(settings) = displays.primary().settings() {
        Some((*settings).borrow().resolution)
    } else { None }
}

pub fn set_resolution(new_resolution: Resolution) {
    let displays = query_displays().expect("No primary display found!");
    if let Some(settings) = displays.primary().settings() {
        (*settings).borrow_mut().resolution = new_resolution;
        displays.primary().apply().expect("Can't apply new settings to display");
        refresh().expect("Can't apply new settings to display");
    }
}