use chrono::Local;
use sys_info::{loadavg, mem_info};
use sys_info::disk_info;


#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ModuleType {
    Workspace,
    LoadAvg,
    Memory,
    Date,
    Disk,
}


pub trait Module {
    fn get_value(&self) -> String;
}

pub fn module_from_type(t: ModuleType) -> Box<Module + Send> {
    match t {
        ModuleType::Workspace => Box::new(LoadAvg{}),
        ModuleType::LoadAvg => Box::new(LoadAvg {}),
        ModuleType::Memory => Box::new(Memory {}),
        ModuleType::Date => Box::new(Date {}),
        ModuleType::Disk => Box::new(Disk {}),
    }
}

pub struct LoadAvg {}

impl Module for LoadAvg {
    fn get_value(&self) -> String {
        match loadavg() {
            Ok(load) => {
                let load_value = load.one.to_string();
                return load_value;
            }
            Err(x) => {
                eprintln!("Cannot load module info: {}", x);
                return "error".to_string();
            }
        }
    }
}

pub struct Memory;

impl Module for Memory {
    fn get_value(&self) -> String {
        match mem_info() {
            Ok(info) => {
                let value = ((info.total - info.avail) as f64 / 1000.0) / 1000.0;
                let value = format!("{:.2}", value);
                return value;
            }
            Err(x) => {
                eprintln!("Cannot load module info: {}", x);
                return "error".to_string();
            }
        }
    }
}

pub struct Date;

impl Module for Date {
    fn get_value(&self) -> String {
        let time = Local::now();
        return format!("{}", time.format("%Y-%m-%d %H:%M"));
    }
}


pub struct Disk;

impl Module for Disk {
    fn get_value(&self) -> String {
        match disk_info() {
            Ok(info) => {
                let value = ((info.total - info.free) as f64 / 1000.0) / 1000.0;
                let value = format!("{:.0}", value);
                return value;
            }
            Err(x) => {
                eprintln!("Cannot load module info: {}", x);
                return "error".to_string();
            }
        }
    }
}

