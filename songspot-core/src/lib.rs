use relm4::prelude::*;
use adw::prelude::*;

// App state
#[derive(Default)]
struct App {
    is_listening: bool,
}

// App messages
#[derive(Debug)]
enum Msg {
    StartListening,
    StopListening,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("SongSpot"),
            set_default_size: (400, 300),

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &gtk::Label {
                        set_text: "SongSpot",
                        add_css_class: "title",
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 20,
                    set_margin_all: 40,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    gtk::Image {
                        set_icon_name: Some("audio-x-generic-symbolic"),
                        set_pixel_size: 64,
                        set_margin_bottom: 20,
                    },

                    gtk::Label {
                        set_text: "Welcome to SongSpot",
                        add_css_class: "title-1",
                        set_margin_bottom: 10,
                    },

                    gtk::Label {
                        set_text: "Your music companion",
                        add_css_class: "subtitle",
                        set_margin_bottom: 30,
                    },

                    gtk::Button {
                        #[watch]
                        set_label: if model.is_listening {
                            "Stop Listening"
                        } else {
                            "Listen"
                        },
                        add_css_class: "suggested-action",
                        connect_clicked[sender] => move |_| {
                            if model.is_listening {
                                sender.input(Msg::StopListening);
                            } else {
                                sender.input(Msg::StartListening);
                            }
                        }
                    },
                },
            },
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App { is_listening: false };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::StartListening => {
                self.is_listening = true;
                // Start listening logic here
            }
            Msg::StopListening => {
                self.is_listening = false;
                // Stop listening logic here
            }
        }
    }
}

pub fn run_app() {
    adw::init().expect("Failed to initialize libadwaita");
    
    let app = RelmApp::new("com.example.songspot");
    app.run::<App>(());
}
