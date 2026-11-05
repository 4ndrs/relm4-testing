use gtk::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

/// alert dialog component
pub struct Alert {
    pub settings: AlertSettings,
    pub is_active: bool,
}

/// messages that can be sent to the alert dialog component
#[derive(Debug)]
pub enum AlertInput {
    /// message sent by the parent to view the dialog
    Show,

    #[doc(hidden)]
    Response(gtk::ResponseType),
}

/// user action performed on the alert dialog
#[derive(Debug)]
pub enum AlertOutput {
    /// user clicked confirm button
    Confirm,

    /// user clicked cancel button
    Cancel,

    /// user clicked user-supplied option
    Option,
}

/// configuration for the alert dialog component
pub struct AlertSettings {
    /// large text
    pub text: String,
    /// optional secondary, smaller text
    pub secondary_text: Option<String>,
    /// modal dialogs freeze other windows as long they are visible
    pub is_modal: bool,
    /// sets color of the accept button to red if the theme supports it
    pub destructive_accept: bool,
    /// text for confirm button
    pub confirm_label: String,
    /// text for cancel button
    pub cancel_label: String,
    /// text for third option button
    /// if [`None`] the third button won't be created
    pub option_label: Option<String>,
}

#[relm4::component(pub)]
impl SimpleComponent for Alert {
    type Init = AlertSettings;
    type Input = AlertInput;
    type Output = AlertOutput;

    fn init(
        settings: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            settings,
            is_active: false,
        };

        let widgets = view_output!();

        if let Some(option_label) = &model.settings.option_label {
            widgets
                .dialog
                .add_button(option_label, gtk::ResponseType::Other(0));
        }

        if model.settings.destructive_accept {
            let accept_widget = widgets
                .dialog
                .widget_for_response(gtk::ResponseType::Accept)
                .expect("no button for accept response set");

            accept_widget.add_css_class("destructive-action");
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AlertInput::Show => {
                self.is_active = true;
            }
            AlertInput::Response(response_type) => {
                self.is_active = false;

                sender
                    .output(match response_type {
                        gtk::ResponseType::Accept => AlertOutput::Confirm,
                        gtk::ResponseType::Other(_) => AlertOutput::Option,
                        _ => AlertOutput::Cancel,
                    })
                    .unwrap()
            }
        }
    }

    view! {
        #[root]
        #[name = "dialog"]
        gtk::MessageDialog {
            set_message_type: gtk::MessageType::Question,

            #[watch]
            set_visible: model.is_active,
            connect_response[sender] => move |_, response| {
                sender.input(AlertInput::Response(response))
            },

            // apply configuration
            set_text: Some(&model.settings.text),
            set_secondary_text: model.settings.secondary_text.as_deref(),
            set_modal: model.settings.is_modal,
            add_button: (&model.settings.confirm_label, gtk::ResponseType::Accept),
            add_button: (&model.settings.cancel_label, gtk::ResponseType::Cancel)
        }
    }
}
