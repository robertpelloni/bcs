pub mod core {
    pub mod global {
        pub mod bcsstring;
        pub mod bcssignal;
    }
    pub mod tools {
        pub mod bcsgeometry;
        pub mod bcscommandlineparser;
    }
    pub mod kernel {
        pub mod bcs_event;
        pub mod bcstimer;
    }
    pub mod io {
        pub mod bcsdir;
        pub mod bcsfile;
        pub mod bcstextstream;
    }
}
pub mod gui {
    pub mod kernel {
        pub mod bcsapplication;
        pub mod bcsinputarbitrator;
    }
    pub mod widgets {
        pub mod bcswidget;
    }
}
pub mod network {
    pub mod socket {
        pub mod bcstcpserver;
        pub mod bcstcpsocket;
    }
}
