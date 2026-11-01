use gtk::prelude::*;
use relm4::SimpleComponent;
use relm4::gtk;
use relm4::prelude::*;

struct HeaderModel;

#[derive(Debug)]
enum HeaderOutput {
    View,
    Edit,
    Export,
}

#[relm4::component]
impl SimpleComponent for HeaderModel {
    type Init = ();
    type Input = AppMsg;
    type Output = HeaderOutput;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        #[root]
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled[sender]  => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::View).unwrap()
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::Edit).unwrap()
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::Export).unwrap()
                        }
                    }
                },
            },
        }
    }
}

struct DialogModel {
    hidden: bool,
}

#[derive(Debug)]
enum DialogInput {
    Show,
    Accept,
    Cancel,
}

#[derive(Debug)]
enum DialogOutput {
    Close,
}

#[relm4::component]
impl SimpleComponent for DialogModel {
    type Init = bool;
    type Input = DialogInput;
    type Output = DialogOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { hidden: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DialogInput::Show => self.hidden = false,
            DialogInput::Accept => {
                self.hidden = true;
                sender.output(DialogOutput::Close).unwrap()
            }
            DialogInput::Cancel => self.hidden = true,
        }
    }

    view! {
        gtk::MessageDialog {
            set_modal: true,

            #[watch]
            set_visible: !model.hidden,
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: ("Close", gtk::ResponseType::Accept),
            add_button: ("Cancel", gtk::ResponseType::Cancel),
            connect_response[sender] => move |_, response| {
                sender.input(if response == gtk::ResponseType::Accept {
                    DialogInput::Accept
                } else {
                    DialogInput::Cancel
                })
            }
        }
    }
}

struct AppModel {
    mode: AppMode,
    header: Controller<HeaderModel>,
    dialog: Controller<DialogModel>,
}

#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
enum AppMsg {
    Close,
    CloseRequest,
    SetMode(AppMode),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = AppMode;
    type Input = AppMsg;
    type Output = AppMsg;

    fn init(
        mode: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header: Controller<HeaderModel> = HeaderModel::builder().launch(()).forward(
            sender.input_sender(),
            |output| match output {
                HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
            },
        );

        let dialog: Controller<DialogModel> = DialogModel::builder()
            .transient_for(&root)
            .launch(true)
            .forward(sender.input_sender(), |output| match output {
                DialogOutput::Close => AppMsg::Close,
            });

        let model = Self {
            mode,
            header,
            dialog,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::SetMode(mode) => self.mode = mode,
            AppMsg::Close => relm4::main_application().quit(),
            AppMsg::CloseRequest => self.dialog.sender().send(DialogInput::Show).unwrap(),
        }
    }

    view! {
        gtk::Window {
            set_default_size: (500, 250),
            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                #[watch]
                set_label: &format!("Placeholder for {:?}", model.mode),
            },
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                gtk::glib::Propagation::Stop
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.components");

    app.run::<AppModel>(AppMode::View);
}
