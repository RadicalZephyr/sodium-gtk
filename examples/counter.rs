use sodium_gtk::gtk::prelude::*;
use sodium_gtk::gtk::{Application, ApplicationWindow};

fn main () {
    let app = Application::builder()
        .application_id("io.github.sodium-gtk.Counter")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Counter")
            .build();

            let button = Button::with_label("Click me!");
            button.connect_clicked(|_| {
                eprintln!("Clicked!");
            });
            window.add(&button);

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}