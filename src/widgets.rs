use std::thread;
use std::time::Duration;
use gtk;
use gtk::{Label, LabelExt, Orientation, BoxExt};
use relm::Channel;
use relm::Update;
use relm::Relm;
use relm::Widget;

use modules::{LoadAvg, Module, ModuleType, Memory, module_from_type, Config};

pub struct Model {
    _channel: Channel<String>,
    prefix: String,
    value: String,
    suffix: String,
}


#[derive(Msg)]
pub enum Msg {
    Value(String)
}

pub struct Text {
    model: Model,
    block: gtk::Box,
    label: Label,
}

impl Update for Text {
    type Model = Model;
    type ModelParam = Config;
    type Msg = Msg;

    fn model(relm: &Relm<Self>, params: Self::ModelParam) -> Model {
        let module = module_from_type(params.mod_type);
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |val| {
            stream.emit(Msg::Value(val));
        });
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                sender.send(module.get_value()).expect("Couldn't send value to channel");
            }
        });
        Model {
            _channel: channel,
            prefix: params.prefix,
            value: "...".to_string(),
            suffix: params.suffix,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Value(val) => {
                &self.label.set_text(&val);
            }
        }
    }
}

impl Widget for Text {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.block.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let block = gtk::Box::new(Orientation::Horizontal, 0);
        let prefix = Label::new(model.prefix.as_str());
        let label = Label::new("...");
        let suffix = Label::new(model.suffix.as_str());

        block.pack_start(&prefix, true, true, 0);
        block.pack_start(&label, true, true, 0);
        block.pack_start(&suffix, true, true, 0);

        Text {
            model,
            block,
            label,
        }
    }
}


