use std::time::Duration;

use gtk::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum AppCommand {
    IncrementFinished,
    DecrementFinished,
}

#[relm4::component]
impl Component for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = AppCommand;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counter = 0;

        let model = AppModel { counter };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            // can use sender.spawn_oneshot_command for sync
            AppInput::Increment => sender.oneshot_command(async {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                AppCommand::IncrementFinished
            }),
            AppInput::Decrement => sender.oneshot_command(async {
                tokio::time::sleep(Duration::from_secs(1)).await;

                AppCommand::DecrementFinished
            }),
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppCommand::IncrementFinished => self.counter = self.counter.wrapping_add(1),
            AppCommand::DecrementFinished => self.counter = self.counter.wrapping_sub(1),
        }
    }

    view! {
        gtk::Window {
            set_title: Some("Commands Testing"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button::with_label("Increment") {
                    connect_clicked => AppInput::Increment,

                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked => AppInput::Decrement,
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.commands");

    app.run::<AppModel>(());
}
