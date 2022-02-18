mod daemon_ipc;
mod config;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Entry};
use gtk4::glib::{self, Continue, MainContext, PRIORITY_DEFAULT, clone};
use std::thread;
use std::path::PathBuf;

fn main() {
    let app = Application::builder()
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) { 
    let config = config::Config::load();

    let entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    entry.set_placeholder_text(Some("Enter song"));

    entry.connect_activate(clone!(@weak entry => move |_| {
        let mut music_path = PathBuf::new();
        music_path.push(&config.music_directory);
        music_path.push(entry.buffer().text());
        music_path.set_extension("mp3");
        entry.set_text(""); 

        let sender = sender.clone();

        thread::spawn(move || {
            let request = daemon_ipc::Request::new(music_path);
            let msg = match daemon_ipc::Request::send_request(&request) {
                Ok(_) => "ok",
                Err(e) => e,
            };
            sender.send(msg).expect("Error sending result through channel")
        });
    }));
    
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
