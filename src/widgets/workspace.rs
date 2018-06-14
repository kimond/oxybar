use std::thread;
use std::time::Duration;
use gtk;
use gtk::{Label, LabelExt, Orientation, BoxExt, WidgetExt, StyleContextExt};
use relm::Channel;
use relm::Update;
use relm::Relm;
use relm::Widget;

use modules::{ModuleType};

pub struct WorkspaceConfig {
    pub mod_type: ModuleType,
    pub prefix: String,
    pub suffix: String,
}

pub struct Model {
    _channel: Channel<String>,
    value: String,
}


#[derive(Msg)]
pub enum Msg {
    Value(String)
}

pub struct Workspace {
    model: Model,
    block: gtk::Box,
    label: Label,
}

impl Update for Workspace {
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
//                sender.send(module.get_value()).expect("Couldn't send value to channel");
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
                self.model.value = val;
            }
        }
    }
}

impl Widget for Workspace {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.block.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let block = gtk::Box::new(Orientation::Horizontal, 0);
        block.get_style_context().map(|c| c.add_class("oxybar-block"));
        let label = Label::new(model.value.as_str());

        block.pack_start(&label, true, true, 0);

        Workspace {
            model,
            block,
            label,
        }
    }
}

