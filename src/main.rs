use std::path::PathBuf;

use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt
};

use relm4::{gtk,
    ComponentParts,
    ComponentSender,
    RelmApp,
    RelmWidgetExt,
    SimpleComponent,
    factory::{FactorySender, FactoryVecDeque},
    prelude::{DynamicIndex, FactoryComponent}
};

// FileCard Component
// Constructs FileCards that contain a name, and an icon (TODO)
// FileCards can be shown in a variety of views:
//      - Icon FlowBox: Displays files with their icons
//      - Vertical Box Detail Mode: Displays files with their file metadata in columns
#[derive(Debug)]
struct FileCard {
    pub full_path: PathBuf,
}

#[derive(Debug)]
enum FileCardMsg { }

#[derive(Debug)]
enum FileCardOutput { }

#[relm4::factory]
impl FactoryComponent for FileCard {
    type Init = PathBuf;
    type Input = FileCardMsg;
    type Output = FileCardOutput;
    type CommandOutput = ();
    type ParentInput = AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: self.full_path.to_str().unwrap(),
                set_width_chars: 32,
            },
        }
    }
    fn init_model(full_path: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { full_path }
    }
}

// Main App

#[derive(Debug)]
struct App {
    // For now only select one file at a time
    selected_file: Option<usize>,
    files: FactoryVecDeque<FileCard>,
}

#[derive(Debug)]
enum AppMsg {
    AddFileCard,
    RemoveFileCard,
    Quit,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Argo"),
            set_default_width: 640,
            set_default_height: 480,

            // Main App View
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                // Quit/Exit Button
                gtk::Button {
                    set_label: "Quit",
                    // Emit quit signal
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Quit);
                    }
                },

                // Add/Remove file cards
                gtk::Button {
                    set_label: "Add file card",
                    connect_clicked => AppMsg::AddFileCard,
                },

                gtk::Button {
                    set_label: "Remove file card",
                    connect_clicked => AppMsg::RemoveFileCard,
                },

                // File Cards
                #[local_ref]
                files_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }

    /// Initialize the main application user interface.
    fn init(
        filecard: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let filecards = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = App {
            selected_file: None,
            files: filecards
        };

        let files_box = model.files.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::AddFileCard => {
                self.files.guard().push_back(PathBuf::from("bbbb"));
            }
            AppMsg::RemoveFileCard => {
                self.files.guard().pop_back();
            }
            AppMsg::Quit => {
                // TODO: Close app on click
                //self.close();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("io.github.jmdaemon.argo");
    app.run::<App>(0);
}
