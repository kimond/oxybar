use std::thread;
use std::time::Duration;
use sys_info::loadavg;
use gtk::{Label, LabelExt, Box, Orientation, BoxExt};
use relm::Channel;
use relm::Update;
use relm::Relm;
use relm::Widget;

pub struct Model {
    _channel: Channel<String>,
    value: String,
}

#[derive(Msg)]
pub enum Msg {
    Value(String)
}

pub struct CpuModule {
    model: Model,
    block: Box,
    label: Label,
}

impl Update for CpuModule {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |val| {
            stream.emit(Msg::Value(val));
        });
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                match loadavg() {
                    Ok(load) => {
                        let load_value = load.one.to_string();
                        sender.send(load_value).expect("Counld't send data to channel");
                    }
                    Err(x) => {
                        eprintln!("Cannot load cpu usage: {}", x);
                        sender.send("error".to_string()).expect("Couldn't send data to channel");
                    }
                }
            }
        });
        Model {
            _channel: channel,
            value: "...".to_string(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Value(val) => {
                &self.label.set_text(&val);
            },
        }
    }
}

impl Widget for CpuModule {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.block.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let block = Box::new(Orientation::Horizontal, 0);
        let prefix = Label::new("CPU ");
        let label = Label::new("...");
        let suffix = Label::new("%");

        label.set_text(&model.value);

        block.pack_start(&prefix, true, true, 0);
        block.pack_start(&label, true, true, 0);
        block.pack_start(&suffix, true, true, 0);

        CpuModule {
            model,
            block,
            label,
        }
    }
}


