use crate::get_app;

pub static mut JAPPLICATION: Application = Application {
    name: String::new(),
};

pub struct Application {
    pub name: String,
}

impl Application {
    pub(crate) fn start(&self) {
        get_app(self.name.to_string());
    }
}
