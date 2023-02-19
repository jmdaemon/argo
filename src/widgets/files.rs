use std::path::PathBuf;

use gtk::prelude::{
    BoxExt,
    OrientableExt
};

use relm4::{
    component,
    gtk,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    factory::{FactorySender, FactoryVecDeque},
    prelude::{DynamicIndex, FactoryComponent},
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

#[derive(Debug)]
pub struct FilesView {
    pub directory: Option<PathBuf>,
    // For now only select one file at a time
    selected_file: Option<usize>,
    files: FactoryVecDeque<FileCard>,
}

#[derive(Debug)]
pub enum FilesViewMsg {
    AddFileCard,
    RemoveFileCard,
}

#[derive(Debug)]
pub enum FilesViewOutput {
    Icon,
    Detail,
}

#[component(pub)]
impl SimpleComponent for FilesView {
    type Init = Option<PathBuf>;
    type Input = FilesViewMsg;
    type Output = FilesViewOutput;

    view! {
        #[root]
        gtk::ScrolledWindow {
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
        // NOTE: This will be done later by the library code that we'll call later on
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
            directory,
            selected_file: None,
            files: filecards,
        };

        let files_box = model.files.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    // Add or remove file cards
    // TODO: In the future we may have to remove by name, so we'll probably have to develop
    // A proper function that removes these file cards by name
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
