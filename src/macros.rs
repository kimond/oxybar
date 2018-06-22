macro_rules! set_prop {
    ($conn:expr, $window:expr, $name:expr, @atom $value:expr) => {
        {
            match xcb::intern_atom($conn, true, $value).get_reply() {
                Ok(atom) => set_prop!($conn, $window, $name, &[atom.atom()], "ATOM", 32),
                _ => panic!("Unable to set window property"),
            }
        }
    };
    ($conn:expr, $window:expr, $name:expr, $data:expr) => {
        {
            set_prop!($conn, $window, $name, $data, "CARDINAL", 32)
        }
    };
    ($conn:expr, $window:expr, $name:expr, $data:expr, $type:expr, $size:expr) => {
        {
            let type_atom = xcb::intern_atom($conn, true, $type).get_reply();
            let property = xcb::intern_atom($conn, true, $name).get_reply();
            match (type_atom, property) {
                (Ok(type_atom), Ok(property)) => {
                    let property = property.atom();
                    let type_atom = type_atom.atom();
                    let mode = xcb::PROP_MODE_REPLACE as u8;
                    xcb::change_property($conn, mode, $window, property, type_atom, $size, $data);
                },
                (Err(_), _) | (_, Err(_)) => panic!("Unable to set window property"),
            }
        }
    };
}