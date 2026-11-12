use gtk::glib::prelude::ToValue;
use gtk::{gdk, gio};
use relm4::factory::Position;
use relm4::factory::positions::GridPosition;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

struct AnimeImage {
    path: String,
}

#[relm4::factory]
impl FactoryComponent for AnimeImage {
    type Init = String;
    type Input = ();
    type Output = ();
    type ParentWidget = gtk::Grid;
    type CommandOutput = ();

    fn init_model(path: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self { path }
    }

    view! {
        &gtk::Picture {
            set_content_fit: gtk::ContentFit::Cover,
            set_size_request: (150, 150),
            set_filename: Some(&self.path),
            add_controller = gtk::DragSource {
                set_content:
                    Some(&gdk::ContentProvider::for_value(&gio::File::for_path(&self.path).to_value())),
                set_actions: gdk::DragAction::COPY,
            },
        },
    }
}

impl Position<GridPosition, DynamicIndex> for AnimeImage {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index() as i32;

        let row = index / 5;
        let column = index % 5;

        GridPosition {
            column,
            row,
            width: 1,
            height: 1,
        }
    }
}

struct AppModel {
    text: String,
    anime_images: FactoryVecDeque<AnimeImage>,
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
        let anime_images = FactoryVecDeque::builder()
            .launch(gtk::Grid::default())
            .detach();

        let mut model = Self {
            text: "start".to_string(),
            anime_images,
        };

        {
            let mut guard = model.anime_images.guard();

            for _ in 1..40 {
                guard.push_back("kurumi think.jpg".to_string());
            }
        }

        let anime_images_grid = model.anime_images.widget();

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

                gtk::Separator,

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    #[local_ref]
                    anime_images_grid -> gtk::Grid {
                        set_orientation: gtk::Orientation::Vertical,
                        set_row_spacing: 5,
                        set_column_spacing: 5,
                        add_css_class: "anime-images-grid",
                    }
                }
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

.anime-images-grid {
    margin: 0 20px 20px;
}
"#;
