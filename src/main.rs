use std::path::PathBuf;

use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk,
    Controller,
    ComponentParts,
    ComponentSender,
    RelmApp,
    RelmWidgetExt,
    SimpleComponent,
    Component,
    factory::{FactorySender, FactoryVecDeque, FactoryView}
};

/*
 * 1. Read a directory of files into a string
 */

pub struct FileState {
    pub full_path: String,
}

pub struct FileCard {
    settings: FileState,
}

// FileCardFactory

#[derive(Debug)]
struct FileCardFactoryWidgets {
    label: gtk::Label,
}

impl FactoryPrototype for FileCard {
    type Factory = FactoryVecDeque<Self>;
    type Widgets = FileCardFactoryWidgets;
    type Root = gtk::Label;
    type View = gtk::Box;
    type Msg = AppMsg;

    fn generate(&self, index: &usize, sender: Sender<AppMsg>) -> FileCardFactoryWidgets {
        let button = gtk::Button::with_label(&self.value.to_string());
        //let label = gtk::Label::new(Some(self.full_path.to_str().unwrap()))
        let index = *index;
        button.connect_clicked(move |_| {
            sender.send(AppMsg::Clicked(index)).unwrap();
        });

        FileCardFactoryWidgets { label: button }
    }

    fn position(&self, _index: &usize) {}

    fn update(&self, _index: &usize, widgets: &FileCardFactoryWidgets) {
        widgets.label.set_label(&self.full_path.to_string());
    }

    fn get_root(widgets: &FileCardFactoryWidgets) -> &gtk::Label {
        &widgets.label
    }
}

/*
#[relm4::component(pub)]
impl SimpleComponent for FileCard {
    type Widgets = DirectoryWidgets;
    type Init = FileState;
    type Input = FileMsg;
    type Output = FileResponse;

    view! {
        #[wrap(Some)]
        gtk::Box {
            gtk::Box {
                gtk::Label {
                    set_label: "Filename"
                }
            }
        }
    }

    fn init(
        settings: FileState,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = FileCard {
            settings,
        };

        // Make Treeview
        //let view = gtk::TreeView::new();

        //let file_column = gtk::TreeViewColumn::new();

        //let icon_cell = gtk::CellRendererPixbuf::new();
        //file_column.pack_start(&icon_cell, false);
        //file_column.add_attribute(&icon_cell, "gicon", ICON_COLUMN.into());

        //let filename_cell = gtk::CellRendererText::new();
        //filename_cell
            //.set_property("ellipsize", &pango::EllipsizeMode::End)
            //.unwrap();
        //file_column.pack_start(&filename_cell, true);
        //file_column.add_attribute(&filename_cell, "text", NAME_COLUMN.into());

        //let dir_expand_cell = gtk::CellRendererPixbuf::new();
        //file_column.pack_end(&dir_expand_cell, false);
        //file_column.add_attribute(&dir_expand_cell, "gicon", IS_DIR_COLUMN.into());

        //view.append_column(&file_column);

        //let store = list_store_for(&model.dir).unwrap();
        //view.set_model(Some(&store));

        //root.add(&view);

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: FileMsg, sender: ComponentSender<Self>) {
    }
}
*/

struct AppModel {
    // For now only select one file at a time
    selected_file: Option<usize>,
    //filesview: Controller<FileCard>,
    files: FactoryVecDeque<FileCard>,
}

#[derive(Debug)]
enum AppMsg {
    Add,
    Remove,
    Clicked(usize),
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

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,
                    factory!(model.files),
                }

                //gtk::Label {
                    //#[watch]
                    //set_label: &format!("Initial Directory: {}", model.selected_path),
                    //set_margin_all: 5,
                //},
            }
        }
    }

    //fn init(
        //root: Self::Init,
        //window: &Self::Root,
        //sender: ComponentSender<Self>,
    //) -> relm4::ComponentParts<Self> {

        //let model = AppModel { selected_file: None, files: Files }

        ////let model = AppModel {
            ////selected_path: "/home/jmd/".to_owned(),
            ////filesview: FileCard::builder()
                ////.launch(FileState {full_path: String::from("/home/jmd/"), filename: String::from("/home/jmd/")} )
                ////.detach()
                //////.forward(sender.input_sender(), |_| {AppMsg::Quit} )
                //////.forward(sender.input_sender(), convert_alert_response),

        ////};

        //let widgets = view_output!();
        //ComponentParts { model, widgets }
    //}

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Add => {
                self.files.push(FileCard {
                    settings: FileState{
                        full_path: PathBuf::from("aaaa"),
                    },
                });

            }
            AppMsg::Remove => {
                self.files.pop();
            }
            AppMsg::Clicked(index) => {
                if let Some(file_index) = self.files.get_mut(index) {
                    println!("Clicked on {}", file_index);
                }
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
    app.run::<AppModel>(0);
}
