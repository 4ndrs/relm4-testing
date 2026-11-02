use gtk::prelude::*;
use relm4::loading_widgets::LoadingWidgets;
use relm4::prelude::*;
use relm4::{gtk, view};
use tokio;

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

#[relm4::component(async)]
impl AsyncComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let counter = 0;

        let model = AppModel { counter };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        match message {
            AppInput::Increment => self.counter = self.counter.wrapping_add(1),
            AppInput::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local]
            root {
                set_title: Some("Async Component Testing"),
                set_default_size: (300, 100),

                // this will be removed automatically by
                // LoadingWidgets when the full view has loaded
                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_halign: gtk::Align::Center,
                }
            }
        }

        Some(LoadingWidgets::new(root, spinner))
    }

    view! {
        gtk::Window {
            set_title: Some("Async Component Testing"),
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
    let app = RelmApp::new("relm4.testing.async");

    app.run_async::<AppModel>(());
}
