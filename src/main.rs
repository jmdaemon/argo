use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, Controller, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, Component};

pub struct DirectorySettings {
    pub full_path: String,
    pub filename: String,
}

pub struct Directory {
    settings: DirectorySettings,
}

#[derive(Debug)]
pub enum DirectoryMsg {
    /// Message sent by the parent to view the dialog
    Show,

    #[doc(hidden)]
    Response(gtk::ResponseType),
}

/// User action performed on the directory
#[derive(Debug)]
pub enum DirectoryResponse {
    /// User double clicked directory.
    Select,

    /// User deselected directory.
    Deselect,

    /// User shift clicked multiple directories.
    Multiselect,
}

#[relm4::component(pub)]
impl SimpleComponent for Directory {
    type Widgets = DirectoryWidgets;
    type Init = DirectorySettings;
    type Input = DirectoryMsg;
    type Output = DirectoryResponse;

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
        settings: DirectorySettings,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Directory {
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

    fn update(&mut self, input: DirectoryMsg, sender: ComponentSender<Self>) {
    }
}

struct AppModel {
    selected_path: String,
    filesview: Controller<Directory>,
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
                },
            }
        }
    }

    fn init(
        root: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel {
            selected_path: "/home/jmd/".to_owned(),
            filesview: Directory::builder()
                .launch(DirectorySettings {full_path: String::from("/home/jmd/"), filename: String::from("/home/jmd/")} )
                .detach()
                //.forward(sender.input_sender(), |_| {AppMsg::Quit} )
                //.forward(sender.input_sender(), convert_alert_response),

        };

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
