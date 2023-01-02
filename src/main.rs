use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    selected_path: String,
}

#[derive(Debug)]
enum AppMsg {
    Quit,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Widgets = AppWidgets;
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Argo"),
            set_default_width: 640,
            set_default_height: 480,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Quit",
                    // Emit quit signal
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Quit);
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Initial Directory: {}", model.selected_path),
                    set_margin_all: 5,
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { selected_path: "/home/jmd/".to_owned() };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => {
                // TODO: Close app on click
                //self.close();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("io.github.jmdaemon.argo");
    app.run::<AppModel>(0);
}
