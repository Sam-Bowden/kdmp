use eframe::{egui::{self, TextEdit, Color32}, epi};
use crate::command::Command;

pub struct App {
    command: Command,
}

impl Default for App {
    fn default() -> Self {
        Self {
            command: Command::new(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "KDMP"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>
    ) {
        ctx.set_visuals(egui::Visuals::dark());
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //Create a command text field.
            let mut command_text_edit = TextEdit::singleline(&mut self.command.text);
            command_text_edit = command_text_edit.lock_focus(true);
            let command_text_edit = ui.add(command_text_edit);

            //Create scrollable options list.
            let mut options_list = egui::ScrollArea::new([true; 2]);
            options_list = options_list.auto_shrink([false; 2]);

            //Update options if the command has changed.
            if command_text_edit.changed() { self.command.update_options(); }

            //Attempt to execute command if enter is pressed
            if command_text_edit.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                match self.command.execute() {
                    Ok(()) => frame.quit(),
                    Err(e) => println!("{}", e),
                }
            }

            //Exit if escape pressed.
            if ui.input().key_pressed(egui::Key::Escape) {
                frame.quit();
            }

            //Ensure command text field keeps focus.
            command_text_edit.request_focus();

            //Add options to options list in ui.
            options_list.show(ui, |ui| {
                let mut list = self.command.options.iter();

                //Highlight first option which will be chosen
                if let Some(text) = list.next() {
                    ui.colored_label(Color32::WHITE, text);
                }

                for text in list {
                    ui.label(text);
                }
            });
        });
    }
}