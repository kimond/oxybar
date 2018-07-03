use gtk;
use gtk::prelude::*;
use relm::{ContainerWidget, Component};
use widgets::{Text, TextConfig, Workspace};
use modules::ModuleType;
use config::Config;

pub struct Bar {
    pub container: gtk::Box,
    _workspace: Option<Component<Workspace>>,
    _blocks: Vec<Component<Text>>,
}

impl Bar {
    pub fn new(config: &Config) -> Bar {
        let left_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        left_widgets.set_halign(gtk::Align::Start);
        let right_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        right_widgets.set_halign(gtk::Align::End);

        let mut workspace_module = None;
        let mut blocks = Vec::new();
        for module_name in &config.left_modules {
            if let Some(module) = &config.modules.get(module_name.as_str()) {
                match module.mod_type {
                    ModuleType::Workspace => {
                        workspace_module = Some(left_widgets.add_widget::<Workspace>(()));
                    },
                    _ => ()
                }
            }
        }

        for module_name in &config.right_modules {
            if let Some(ref module) = &config.modules.get(module_name.as_str()) {
                let module_config = TextConfig {
                    mod_type: module.mod_type,
                    prefix: module.prefix.as_ref().map_or("".to_string(), | v| v.clone()),
                    suffix: module.suffix.as_ref().map_or("".to_string(), |v| v.clone()),
                };
                let module_component = right_widgets.add_widget::<Text>(module_config);
                blocks.push(module_component);
            }
        }

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        container.pack_start(&left_widgets, true, true, 0);
        container.pack_end(&right_widgets, true, true, 0);

        Bar { container, _workspace: workspace_module, _blocks: blocks }
    }
}
