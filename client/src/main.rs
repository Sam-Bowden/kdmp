use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Entry};
use gtk4::glib::{self, Continue, MainContext, PRIORITY_DEFAULT, clone};
use std::os::unix::net::UnixStream;
use std::thread;
use std::io::Write;

fn main() {
    let app = Application::builder()
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    entry.connect_activate(move |_| {
        let sender = sender.clone();
        thread::spawn(move || {
            let msg = match send_request() {
                Ok(_) => "ok",
                Err(e) => e,
            };

            sender.send(msg).expect("Error sending result through channel")
        });
    });
    
    receiver.attach(None, clone!(@weak app => @default-return Continue(false), move |msg| {
        if msg == "ok" {
            app.quit();
        } else {
            println!("Error: {}", msg);
        }

        glib::Continue(true)
    }),);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("KDMP")
        .child(&entry)
        .default_height(100)
        .default_width(300)
        .resizable(false)
        .build();

    window.present();
}

fn send_request() -> Result<(), &'static str> {
    let mut stream = match UnixStream::connect("/tmp/kdmp.sock") {
        Ok(s) => s,
        Err(_) => return Err("Failed to connect to daemon"),
    };

    match stream.write_all("test".as_bytes()) {
        Ok(()) => Ok(()),
        Err(_) => Err("Failed to communicate to daemon"),
    }
}
