use std::thread;
use std::time::Duration;
use sys_info::loadavg;
use gtk::{Label, LabelExt};
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
    pub label: Label
}

impl Update for CpuModule {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |val| {
            println!("in stream");
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
        println!("in update");
        match event {
            Msg::Value(val) => {
                println!("{}", &val);
                &self.label.set_text(&val);
            },
        }
    }
}

impl Widget for CpuModule {
    type Root = Label;

    fn root(&self) -> Self::Root {
        self.label.clone()
    }


    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let label = Label::new("...");

        label.set_text(&model.value);

        let mut module = CpuModule {
            model,
            label
        };

        module
    }
}


