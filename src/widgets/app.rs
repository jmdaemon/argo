use crate::widgets::{
    files::{FilesView, FilesViewOutput},
    bookmarks::BookmarksView,
};

use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt
};

use relm4::{
    component,
    gtk::{self, prelude::ApplicationExt, traits::WidgetExt},
    Controller,
    ComponentParts,
    ComponentSender,
    RelmWidgetExt,
    SimpleComponent,
    Component, ComponentController
};

use dirs::home_dir;


// Main App

#[derive(Debug)]
pub enum AppMode {
    Icon,
    Detail,
}

#[derive(Debug)]
pub enum AppMsg {
    SetMode(AppMode),
    GotoBookmark,
    Close,
}

#[derive(Debug)]
pub struct App {
    mode: AppMode,
    filesview: Controller<FilesView>,
    bookmarksview: Controller<BookmarksView>,
}

#[component(pub)]
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
            gtk::Paned {
                set_orientation: gtk::Orientation::Horizontal,
                //set_spacing: 5,
                set_margin_all: 5,
                set_size_request: (200, -1),

                //#[wrap(Some)]
                //set_start_child = model.bookmarks.widget(),

                set_halign: gtk::Align::Start,
                #[wrap(Some)]
                set_start_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_halign: gtk::Align::Start,
                        //set_hexpand: false,
                        //set_vexpand: false,

                        //set_hexpand: false,
                        //set_vexpand: true,
                        //set_size_request: (200, 200),
                        //set_size_request: (100, 200),
                        //model.bookmarksview.widget(),
                        set_size_request: (100, 100),
                        model.bookmarksview.widget() {
                            set_vexpand: true,
                            //set_size_request: (-1, 400),
                            //set_hexpand: false,
                            //set_vexpand: false,
                            //set_vexpand: true,
                            //set_size_request: (200, 200),
                        },

                        gtk::Button {
                            set_hexpand: false,
                            set_label: "Quit",
                            // Emit quit signal
                            connect_clicked[sender] => move |_| {
                                sender.input(AppMsg::Close);
                            },
                        }
                },


                // Quit/Exit Button
                //set_start_child = &gtk::Button {
                    //set_label: "Quit",
                    //// Emit quit signal
                    //connect_clicked[sender] => move |_| {
                        //sender.input(AppMsg::Close);
                    //}
                //},

                // Custom Widgets
                #[wrap(Some)]
                set_end_child = model.filesview.widget(),
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
        
        let default_dir = home_dir().expect("User has no $HOME directory");
        let filesview = FilesView::builder()
            .launch(Some(default_dir))
            .forward(sender.input_sender(),
            |msg| match msg {
                FilesViewOutput::Icon => AppMsg::SetMode(AppMode::Icon),
                FilesViewOutput::Detail => AppMsg::SetMode(AppMode::Detail),
        });

        let bookmarksview = BookmarksView::builder()
            .launch(()).forward(sender.input_sender(), |msg| AppMsg::GotoBookmark);
        
        let model = App { mode, filesview, bookmarksview };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
            AppMsg::GotoBookmark => {
                // Navigate to bookmark?
            }
        }
    }
}
