use glib;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;
use sys_info::loadavg;
use gtk::{Box, Label, LabelExt};

pub struct CpuModule {
    pub label: Label
}

impl CpuModule {
    pub fn new() -> CpuModule {
        let label = Label::new("...");

        CpuModule { label }
    }

    pub fn spawn_polling(self) {
        let (tx, rx) = channel();
        GLOBAL.with(|global| {
            *global.borrow_mut() = Some((self.label.clone(), rx))
        });

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                match loadavg() {
                    Ok(load) => {
                        let load_value = load.one.to_string();
                        println!("{}", load_value);
                        tx.send(load_value).expect("Counld't send data to channel");
                        glib::idle_add(receive);
                    }
                    Err(x) => {
                        eprintln!("Cannot load cpu usage: {}", x);
                        tx.send("error".to_string()).expect("Couldn't send data to channel");
                        glib::idle_add(receive);
                    }
                }
            }
        });
    }
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref label, ref rx)) = *global.borrow() {
            if let Ok(text) = rx.try_recv() {
                label.set_text(&text);
            }
        }
    });
    glib::Continue(false)
}

// declare a new thread local storage key
thread_local!(
    static GLOBAL: RefCell<Option<(Label, Receiver<String>)>> = RefCell::new(None)
);
