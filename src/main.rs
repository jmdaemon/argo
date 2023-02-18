use std::path::PathBuf;
use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt
};

use relm4::{gtk,
    Controller,
    ComponentParts,
    ComponentSender,
    RelmApp,
    RelmWidgetExt,
    SimpleComponent,
    factory::{FactorySender, FactoryVecDeque},
    prelude::{DynamicIndex, FactoryComponent}, Component, ComponentController
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
    type ParentInput = FilesViewMsg;
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
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

// DirectoryView / FilesView
// Manages the FileCard Views and populates them with the correct filenames

#[derive(Debug)]
struct FilesView {
    pub directory: Option<PathBuf>,
    // For now only select one file at a time
    selected_file: Option<usize>,
    files: FactoryVecDeque<FileCard>,
}

#[derive(Debug)]
enum FilesViewMsg {
    AddFileCard,
    RemoveFileCard,
}

#[derive(Debug)]
enum FilesViewOutput {
    Icon,
    Detail,
}

#[relm4::component]
impl SimpleComponent for FilesView {
    type Init = Option<PathBuf>;
    type Input = FilesViewMsg;
    type Output = FilesViewOutput;

    view! {
        #[name = "filesview"]
        gtk::Box {
            // File Cards
            #[local_ref]
            files_box -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
            }
        }
    }

    fn init(
        directory: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let mut filecards = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());

        // TODO: Pretend we listed out all the paths in the directory
        let paths = vec![
            PathBuf::from("bbbb"),
            PathBuf::from("cccc"),
            PathBuf::from("dddd"),
        ];

        // Add all the found entries as filecard widgets
        for path in paths {
            filecards.guard().push_back(path);
        }

        // Set the model and the open directory
        let model = FilesView {
            //directory: Some(PathBuf::from("/home/jmd")),
            directory,
            selected_file: None,
            files: filecards,
        };

        let files_box = model.files.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    // Add or remove file cards
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            FilesViewMsg::AddFileCard => {
                self.files.guard().push_back(PathBuf::from("bbbb"));
            }
            FilesViewMsg::RemoveFileCard => {
                self.files.guard().pop_back();
            }
        }
    }
}

// Main App

#[derive(Debug)]
enum AppMode {
    Icon,
    Detail,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
    AddFileCard,
    RemoveFileCard,
    Quit,
}

#[derive(Debug)]
struct App {
    mode: AppMode,
    filesview: Controller<FilesView>,

    //filesview: Controller<FilesView>,
    //filesview: FilesView,
    //filesview: gtk::Box,
    //cont_filesview: Controller<FilesView>,

    // For now only select one file at a time
    //selected_file: Option<usize>,
    //files: FactoryVecDeque<FileCard>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
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


                //#[wrap(Some)]
                model.filesview.widget(),
                //#[local_ref]
                //filesview -> gtk::Box {
                //}
                //filesview -> FilesViewWidgets {
                //filesview -> Controller<FilesView> {
                //cont_filesview -> Controller<FilesView> {
                //},
                // FilesView Component
                //#[local_ref]
                //filesview -> FilesView {
                //},


                // File Cards
                //#[local_ref]
                //filesview -> gtk::Box {
                //},

                //gtk::Box {
                    //set_orientation: gtk::Orientation::Vertical,
                    //set_spacing: 5,

                    //#[local_ref]
                    //filesview -> FilesView {
                    //},
                //}
                //files_box -> gtk::Box {
                    //set_orientation: gtk::Orientation::Vertical,
                    //set_spacing: 5,
                //}
            }
        }
    }

    /// Initialize the main application user interface.
    fn init(
        filecard: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let mode = AppMode::Icon;
        
        //let fv_builder = FilesView::builder().attach_to(root);

        //let filesview = fv_builder.widget();

        //let cont_filesview = FilesView::builder()
        let default_dir = PathBuf::from("/home/jmd");
        let filesview = FilesView::builder()
            .launch(Some(default_dir))
            .forward(sender.input_sender(),
            |msg| match msg {
                FilesViewOutput::Icon => AppMsg::SetMode(AppMode::Icon),
                FilesViewOutput::Detail => AppMsg::SetMode(AppMode::Detail),
        });
        
        //let model = App { mode, filesview, cont_filesview };
        //let model = App { mode, cont_filesview };
        //let filesview = cont_filesview.
        let model = App { mode, filesview };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    //FilesViewMsg::Quit => {
        //// TODO: Close app on click
        ////self.close();
    //}

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::AddFileCard => {
                //self.files.guard().push_back(PathBuf::from("bbbb"));
            }
            AppMsg::RemoveFileCard => {
                //self.files.guard().pop_back();
            }
            AppMsg::Quit => {
                // TODO: Close app on click
                //self.close();
            }
        }
    }



    //fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        //match message {
            //AppMsg::AddFileCard => {
                //self.files.guard().push_back(PathBuf::from("bbbb"));
            //}
            //AppMsg::RemoveFileCard => {
                //self.files.guard().pop_back();
            //}
            //AppMsg::Quit => {
                //// TODO: Close app on click
                ////self.close();
            //}
        //}
    //}
}

fn main() {
    let app = RelmApp::new("io.github.jmdaemon.argo");
    app.run::<App>(());
}
