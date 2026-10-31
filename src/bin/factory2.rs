use gtk::prelude::*;
use relm4::factory::FactoryComponent;
use relm4::factory::Position;
use relm4::factory::positions::GridPosition;
use relm4::gtk;
use relm4::prelude::*;

struct LabelButton {
    value: u8,
}

#[relm4::factory]
impl FactoryComponent for LabelButton {
    type Init = u8;
    type Input = ();
    type Output = ();
    type ParentWidget = gtk::Grid;
    type CommandOutput = ();

    fn init_model(value: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self { value }
    }

    view! {
        #[name(label_button)]
        gtk::Button {
            set_label: &self.value.to_string(),
        },
    }
}

impl Position<GridPosition, DynamicIndex> for LabelButton {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index() as i32;

        let row = index / 5;
        let column = index % 5;

        GridPosition {
            row,
            column,
            width: 1,
            height: 1,
        }
    }
}

struct App {
    created_widgets: u8,
    label_buttons: FactoryVecDeque<LabelButton>,
}

#[derive(Debug)]
enum AppMsg {
    Add,
    Remove,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let label_buttons = FactoryVecDeque::builder()
            .launch(gtk::Grid::default())
            .detach();

        let model = App {
            created_widgets: counter,
            label_buttons,
        };

        let label_button_grid = model.label_buttons.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Add => {
                self.label_buttons.guard().push_back(self.created_widgets);
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            AppMsg::Remove => {
                self.label_buttons.guard().pop_back();
            }
        }
    }

    view! {
        gtk::Window {
            set_title: Some("Grid Factory"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Add",
                    connect_clicked => AppMsg::Add,
                },

                gtk::Button {
                    set_label: "Remove",
                    connect_clicked => AppMsg::Remove,
                },

                #[local_ref]
                label_button_grid -> gtk::Grid {
                    set_orientation: gtk::Orientation::Vertical,
                    set_row_spacing: 5,
                    set_column_spacing: 15,
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.testing.factory2");

    app.run::<App>(0);
}
