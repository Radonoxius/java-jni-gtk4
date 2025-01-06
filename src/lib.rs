use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
//use crate::core::JAPPLICATION;

mod core;
mod ffi;
mod io;

#[allow(static_mut_refs)]
pub(crate) fn get_app(name: String) {
    let app = Application::builder()
        .application_id(&format!("ix.radon.guiframe.{}", &name))
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(960)
            .default_height(540)
            .title(&name)
            .build();

        window.present();
    });
    app.run();
}
