use gtk::prelude::*;
use rand::seq::IteratorRandom;
use relm4::prelude::*;
use relm4::{SimpleComponent, gtk};

const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic", // doesn't exist? 🤔
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon_name() -> &'static str {
    ICON_LIST
        .iter()
        .choose(&mut rand::rng())
        .expect("could not choose a random icon")
}

// returns a random icon different from the excluded one (avoids repeats)
fn gen_unique_icon(exclude: &'static str) -> &'static str {
    let mut rnd = random_icon_name();

    while rnd == exclude {
        rnd = random_icon_name()
    }

    dbg!(rnd);

    rnd
}

#[tracker::track]
struct AppModel {
    first_icon: &'static str,
    second_icon: &'static str,
    identical: bool,
}

#[derive(Debug)]
enum AppMsg {
    UpdateFirst,
    UpdateSecond,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let first_icon = random_icon_name();
        let second_icon = random_icon_name();

        let identical = first_icon == second_icon;

        let model = AppModel {
            identical,
            first_icon,
            second_icon,
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.reset();

        match message {
            AppMsg::UpdateFirst => {
                self.set_first_icon(gen_unique_icon(self.first_icon));
            }
            AppMsg::UpdateSecond => {
                self.set_second_icon(gen_unique_icon(self.second_icon));
            }
        }

        self.set_identical(self.first_icon == self.second_icon);
    }

    view! {
        #[root]
        gtk::ApplicationWindow {
            #[track = "model.changed(AppModel::identical())"]
            set_class_active: ("identical", model.identical),

            gtk::Box {
                set_spacing: 10,
                set_margin_all: 10,
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Box {
                    set_spacing: 10,
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Image {
                        set_pixel_size: 50,

                        #[track = "model.changed(AppModel::first_icon())"]
                        set_icon_name: Some(model.first_icon),
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::UpdateFirst)
                        }
                    }
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image {
                        set_pixel_size: 50,

                        #[track = "model.changed(AppModel::second_icon())"]
                        set_icon_name: Some(model.second_icon),
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::UpdateSecond)
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.tracker");

    relm4::set_global_css(".identical { background: #00ad5c; }");

    app.run::<AppModel>(());
}
