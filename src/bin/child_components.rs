use gtk::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use relm4_testing::alert::{Alert, AlertInput, AlertOutput, AlertSettings};

struct App {
    counter: u8,
    alert_toggle: bool,
    dialog: Controller<Alert>,
    second_dialog: Controller<Alert>,
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
    CloseRequest,
    Save,
    Close,
    Ignore,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppInput;
    type Output = ();
    type Init = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            counter: 0,
            alert_toggle: false,
            dialog: Alert::builder()
                .transient_for(&root)
                .launch(AlertSettings {
                    text: String::from("do you want to quit without saving? (first alert)"),
                    secondary_text: Some(String::from("your counter hans't reached 42 yet")),
                    confirm_label: String::from("close without saving"),
                    cancel_label: String::from("cancel"),
                    option_label: Some(String::from("save")),
                    is_modal: true,
                    destructive_accept: true,
                })
                .forward(sender.input_sender(), convert_alert_output),
            second_dialog: Alert::builder()
                .transient_for(&root)
                .launch(AlertSettings {
                    text: String::from("do you want to quit without saving? (second alert)"),
                    secondary_text: Some(String::from("your counter hasn't reached 42 yet")),
                    confirm_label: String::from("close without saving"),
                    cancel_label: String::from("cancel"),
                    option_label: Some(String::from("Save")),
                    is_modal: true,
                    destructive_accept: true,
                })
                .forward(sender.input_sender(), convert_alert_output),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Save => println!("* open save dialog here *"),
            AppInput::Close => relm4::main_application().quit(),
            AppInput::Ignore => (),
            AppInput::Increment => self.counter = self.counter.wrapping_add(1),
            AppInput::Decrement => self.counter = self.counter.wrapping_sub(1),
            AppInput::CloseRequest => {
                if self.counter == 42 {
                    relm4::main_application().quit()
                } else {
                    self.alert_toggle = !self.alert_toggle;

                    if self.alert_toggle {
                        self.dialog.emit(AlertInput::Show);
                    } else {
                        self.second_dialog.emit(AlertInput::Show)
                    }
                }
            }
        }
    }

    view! {
        main_window = gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            connect_close_request[sender] => move |_| {
                sender.input(AppInput::CloseRequest);
                gtk::glib::Propagation::Stop
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppInput::Increment)
                    }
                },

                append = &gtk::Button {
                    set_label: "Decrement",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppInput::Decrement);
                    }
                },

                append = &gtk::Label {
                    set_margin_all: 5,
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter)
                },

                append = &gtk::Button {
                    set_label: "Close",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppInput::CloseRequest);
                    }
                }
            }
        }

    }
}

fn convert_alert_output(output: AlertOutput) -> AppInput {
    match output {
        AlertOutput::Option => AppInput::Save,
        AlertOutput::Cancel => AppInput::Ignore,
        AlertOutput::Confirm => AppInput::Close,
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.ChildComponents");

    app.run::<App>(());
}
