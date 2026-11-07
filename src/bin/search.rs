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
                add_css_class: "main-box"
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

@media (prefers-color-scheme: dark) {
    :root {
        --bg: #1e1e1e;
        --fg: #eeeeee;
    }
}

window {
    background-color: transparent;
}

.main-box {
    color: var(--fg);
    background-color: var(--bg);
    border-radius: 30px;
}
"#;
