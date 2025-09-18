use relm4::prelude::*;
use adw::prelude::*;
use crate::audio::AudioRecorder;
use gio::prelude::*;

// Include the compiled resource
static RESOURCES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource"));

pub struct App {
    is_listening: bool,
    audio_recorder: AudioRecorder,
}

#[derive(Debug)]
pub enum Msg {
    ToggleListening,
}

#[relm4::component(pub)]
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
                        set_resource: Some("/me/ayushshukla/songspot/anime_girl.jpg"),
                        set_pixel_size: 128,
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
                        set_label: if model.is_listening { "Stop Listening" } else { "Listen" },
                        add_css_class: "suggested-action",
                        connect_clicked[sender] => move |_| {
                            sender.input(Msg::ToggleListening);
                        },
                    },

                    gtk::Label {
                        #[watch]
                        set_text: if model.is_listening { "Listening..." } else { "Idle" },
                        add_css_class: "subtitle",
                        set_margin_top: 10,
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
        let audio_recorder = AudioRecorder::new();
        let model = App {
            is_listening: false,
            audio_recorder,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::ToggleListening => {
                self.is_listening = !self.is_listening;
                if self.is_listening {
                    if let Err(e) = self.audio_recorder.start() {
                        eprintln!("Failed to start recording: {}", e);
                        self.is_listening = false;
                    }
                } else {
                    if let Err(e) = self.audio_recorder.stop_and_save("recording.wav") {
                        eprintln!("Failed to save recording: {}", e);
                    }
                }
            }
        }
    }
}

pub fn run_app() {
    // Register the GResource
    let resource = gio::Resource::from_data(&glib::Bytes::from_static(RESOURCES))
        .expect("Could not load gresource file");
    gio::resources_register(&resource);
    
    adw::init().expect("Failed to initialize libadwaita");
    let app = RelmApp::new("com.example.songspot");
    app.run::<App>(());
}
