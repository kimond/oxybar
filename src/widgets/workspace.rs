use std::thread;
use std::time::Duration;
use gtk;
use gtk::{Label, LabelExt, Orientation, BoxExt, WidgetExt, StyleContextExt};
use relm::Channel;
use relm::Update;
use relm::Relm;
use relm::Widget;
use xcb;
use xcb_util::ewmh;

use modules::ModuleType;

pub struct WorkspaceConfig {
    pub mod_type: ModuleType,
    pub prefix: String,
    pub suffix: String,
}

pub struct Model {
    ewmh_conn: ewmh::Connection,
    screen_idx: i32,
    items: Vec<WorkspaceItem>,
}


#[derive(Msg)]
pub enum Msg {
    Update
}

struct WorkspaceItem {
    name: String,
    active: bool,
}

pub struct Workspace {
    model: Model,
    ws_box: gtk::Box,
}

impl Workspace {
    fn get_workspace_items(&self, ewmh_conn: &ewmh::Connection, screen_idx: i32) -> Vec<WorkspaceItem> {
        let current = ewmh::get_current_desktop(&ewmh_conn, screen_idx)
            .get_reply()
            .unwrap_or(0) as usize;
        let names_reply = ewmh::get_desktop_names(&ewmh_conn, screen_idx).get_reply();
        let names = match names_reply {
            Ok(ref r) => r.strings(),
            Err(_) => Vec::new(),
        };

        names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                WorkspaceItem {
                    name: name.to_string(),
                    active: i == current,
                }
            })
            .collect()
    }
    fn render_items(&self) {
        for item in &self.model.items {
            let label = Label::new(item.name.as_str());
            if item.active {
                label.get_style_context().map(|c| c.add_class("active"));
            }
            self.ws_box.pack_start(&label, true, true, 0);
        }
    }
}

impl Update for Workspace {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let (conn, screen_idx) = xcb::Connection::connect(None).expect("Failed to connect to X server");
        let ewmh_conn = ewmh::Connection::connect(conn)
            .map_err(|(e, _)| e)
            .expect("Failed to wrap xcb conn in ewmh conn");

        Model {
            ewmh_conn,
            screen_idx,
            items: Vec::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Update => {
                self.model.items = self.get_workspace_items(&self.model.ewmh_conn, self.model.screen_idx);
                self.render_items();
            }
        }
    }

    fn subscriptions(&mut self, _relm: &Relm<Self>) {
        let xcb_stream = xcb::XcbEventStream::new(self.model.ewmh_conn.clone(), ());

    }
}

impl Widget for Workspace {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.ws_box.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let ws_box = gtk::Box::new(Orientation::Horizontal, 0);
        ws_box.get_style_context().map(|c| c.add_class("oxybar-workspace"));

        let mut ws = Workspace {
            model,
            ws_box,
        };

        ws.update(Msg::Update);

        ws
    }
}

