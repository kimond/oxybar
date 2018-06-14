use chrono::Local;
use sys_info::{loadavg, mem_info};

pub enum ModuleType {
    LoadAvg,
    Memory,
    Date,
}


pub trait Module {
    fn get_value(&self) -> String;
}

pub fn module_from_type(t: ModuleType) -> Box<Module + Send> {
    match t {
        ModuleType::LoadAvg => Box::new(LoadAvg {}),
        ModuleType::Memory => Box::new(Memory {}),
        ModuleType::Date => Box::new(Date {}),
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

pub struct Memory {}

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

pub struct Date {}

impl Module for Date {
    fn get_value(&self) -> String {
        let time = Local::now();
        return format!("{}", time.format("%Y-%m-%d %H:%M"));
    }
}



