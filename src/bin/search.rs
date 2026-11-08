use relm4::gtk::prelude::*;
use relm4::prelude::*;

struct AppModel {
    text: String,
}

#[derive(Debug)]
enum AppInput {
    Update,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            text: "".to_string(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        #[root]
        gtk::ApplicationWindow {
            set_title: Some("PureSearch"),
            set_decorated: false,
            set_resizable: false,
            set_default_size: (850, 550),

            gtk::Box {
                add_css_class: "main-box",
                set_orientation: gtk::Orientation::Vertical,

                gtk::Entry {
                    set_valign: gtk::Align::Start,
                    set_hexpand: true,
                    set_placeholder_text: Some("Search..."),

                    add_css_class: "main-search",
                },

                append = &gtk::Separator,
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
:root {
    --bg: #ffffff;
    --fg: #111111;
}

/* not supported on macos
 * on gentoo (hyprland) relm4::set_global_css stays on light mode
 * when setting it manually with a css provider it changes when launching
 * but doesn't seem to work when changing the mode on noctalia settings dynamically
 * other apps detect the change */
@media (prefers-color-scheme: dark) {
    :root {
        --bg: #1e1e1e;
        --fg: #eeeeee;
    }
}

window {
    background-color: transparent;
}

separator {
    margin: 0 20px;
}

.main-box {
    color: var(--fg);
    background-color: var(--bg);
    border-radius: 30px;
}

.main-search {
    margin: 20px;
    border: none;
    outline: none;

    font-size: 24px;
    font-weight: 500;
}
"#;
