use relm4::gtk::prelude::*;
use relm4::prelude::*;

struct AppModel {
    text: String,
}

#[derive(Debug)]
enum AppInput {
    Update(String),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            text: "start".to_string(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Update(new) => {
                dbg!(&new);

                self.text = new;
            }
        }
    }

    view! {
        #[root]
        gtk::ApplicationWindow {
            set_title: Some("PureSearch"),
            set_decorated: false,
            set_resizable: false,
            set_default_size: (850, 550),
            add_css_class: "main-box",

            gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

                gtk::Entry {
                    set_valign: gtk::Align::Start,
                    set_hexpand: true,
                    set_placeholder_text: Some("Search..."),
                    add_css_class: "main-search",

                    // too many updates
                    connect_changed[sender] => move |entry| sender.input(AppInput::Update(entry.text().to_string())),
                },

                append = &gtk::Separator,
                append = &gtk::Picture {
                    set_filename: Some("kurumi think.jpg"),
                },
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.PureSearch");

    relm4::set_global_css(GLOBAL_CSS);

    app.run::<AppModel>(());
}

const GLOBAL_CSS: &str = r#"
window {
    border-radius: 30px;
}

separator {
    margin: 0 20px;
}

.main-search {
    margin: 20px;
    border: none;
    outline: none;
    background-color: transparent;

    font-size: 24px;
    font-weight: 500;
}
"#;
