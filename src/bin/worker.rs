use gtk::prelude::*;
use relm4::Worker;
use relm4::WorkerController;
use relm4::gtk;
use relm4::prelude::*;

struct AsyncHandler;

#[derive(Debug)]
enum AsyncHandlerInput {
    DelayedIncrement,
    DelayedDecrement,
}

impl Worker for AsyncHandler {
    type Init = ();
    type Input = AsyncHandlerInput;
    type Output = AppInput;

    fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        // simulating heavy cpu-bound task
        std::thread::sleep(std::time::Duration::from_secs(2));

        // send the result of the calculation back
        match message {
            AsyncHandlerInput::DelayedIncrement => sender.output(AppInput::Increment),
            AsyncHandlerInput::DelayedDecrement => sender.output(AppInput::Decrement),
        }
        .unwrap()
    }
}

struct AppModel {
    counter: u8,
    worker: WorkerController<AsyncHandler>,
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
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
        let counter = 0;

        let worker = AsyncHandler::builder()
            .detach_worker(())
            .forward(sender.input_sender(), std::convert::identity);

        let model = AppModel { counter, worker };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Increment => self.counter = self.counter.wrapping_add(1),
            AppInput::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }

    view! {
        gtk::Window {
            set_title: Some("Worker Testing"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button::with_label("Increment") {
                    connect_clicked[sender = model.worker.sender().clone()] => move |_| {
                        sender.send(AsyncHandlerInput::DelayedIncrement).unwrap()
                    }
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked[sender = model.worker.sender().clone()] => move |_| {
                        sender.send(AsyncHandlerInput::DelayedDecrement).unwrap()
                    }
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
    let app = RelmApp::new("relm4.testing.worker");

    app.run::<AppModel>(());
}
