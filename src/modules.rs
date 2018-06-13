use sys_info::{loadavg, mem_info};

pub enum ModuleType {
    LoadAvg,
    Memory
}

pub trait Module {
   fn get_value(&self) -> String;
}

pub struct LoadAvg { }

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

pub struct Memory { }

impl Module for Memory {
    fn get_value(&self) -> String {
        match mem_info() {
            Ok(info) => {
                let value = (info.total - info.avail).to_string();
                return value;
            }
            Err(x) => {
                eprintln!("Cannot load module info: {}", x);
                return "error".to_string();
            }
        }
    }
}



