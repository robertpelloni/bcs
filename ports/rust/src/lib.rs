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
