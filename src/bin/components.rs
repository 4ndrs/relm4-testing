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

struct AppModel {
    header: Controller<HeaderModel>,
}

#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = AppMsg;

    fn init(
        _init: Self::Init,
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

        let model = Self { header };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        gtk::Window {
            set_default_size: (500, 250),
            set_titlebar: Some(model.header.widget())
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.components");

    app.run::<AppModel>(());
}
