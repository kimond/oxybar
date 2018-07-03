use std::thread;
use gtk;
use gtk::{Label, Orientation, BoxExt, WidgetExt, StyleContextExt, ContainerExt};
use relm::Channel;
use relm::Update;
use relm::Relm;
use relm::Widget;
use xcb;
use xcb_util::ewmh;
use std::sync::Arc;

pub struct Model {
    _channel: Channel<()>,
    ewmh_conn: Arc<ewmh::Connection>,
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
    fn render_items(&mut self) {
        let children = self.ws_box.get_children();
        for child in children {
            child.destroy();
        }
        for item in &self.model.items {
            let label = Label::new(item.name.as_str());
            if item.active {
                label.get_style_context().map(|c| c.add_class("active"));
            }
            self.ws_box.pack_start(&label, true, true, 0);
        }
        self.ws_box.show_all();
    }
}

impl Update for Workspace {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let (conn, screen_idx) = xcb::Connection::connect(None)
            .expect("Failed to connect to X server");
        let ewmh_conn = ewmh::Connection::connect(conn)
            .map_err(|(e, _)| e)
            .expect("Failed to wrap xcb conn in ewmh conn");
        let ewmh_conn = Arc::new(ewmh_conn);
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |_| {
            stream.emit(Msg::Update);
        });
        let sub_ewmh_conn = ewmh_conn.clone();
        thread::spawn(move || {
            let setup = sub_ewmh_conn.get_setup();
            let screen = setup.roots().nth(screen_idx as usize).unwrap();
            let values =
                [(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE)];
            {
                let _ = xcb::change_window_attributes(&sub_ewmh_conn, screen.root(), &values)
                    .request_check();
            }
            sub_ewmh_conn.flush();
            loop {
                let event = sub_ewmh_conn.wait_for_event();
                match event {
                    Some(_) => {
                        sender.send(()).expect("Couldn't send value to sender")
                    }
                    None => ()
                }
            }
        });
        Model {
            _channel: channel,
            ewmh_conn: ewmh_conn.clone(),
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

