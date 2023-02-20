use crate::widgets::{
    files::{FilesView, FilesViewOutput},
    bookmarks::BookmarksView,
    toolbar::Toolbar,
    navbar::NavigationBar,
};

use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt,
    ApplicationExt,
    WidgetExt,
};

use relm4::{
    component,
    //gtk::{self, prelude::ApplicationExt, traits::WidgetExt},
    //gtk::{self, prelude::ApplicationExt, traits::WidgetExt},
    gtk,
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
    toolbar: Controller<Toolbar>,
    navbar: Controller<NavigationBar>,
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

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                // Main App View
                #[name="toolbar"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_all: 5,
                    model.toolbar.widget() {
                        set_size_request: (-1, 32),
                    }
                },

                // Main Widget
                gtk::CenterBox {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_vexpand: true,

                    #[name="bookmarks_sidebar"]
                    #[wrap(Some)]
                    set_start_widget = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        gtk::Label {
                            set_label: "Bookmarks",
                            set_height_request: 32,
                        },
                        model.bookmarksview.widget() {
                            set_min_content_width: 180,
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                        },

                        gtk::Button {
                            set_label: "Quit",
                            // Emit quit signal
                            connect_clicked[sender] => move |_| {
                                sender.input(AppMsg::Close);
                            },
                        }
                    },

                    #[name="filesview_panel"]
                    #[wrap(Some)]
                    set_center_widget = &gtk::Box {
                        set_size_request: (200, -1),
                        //set_size_request: (400, 480),
                        set_hexpand: true,
                        set_orientation: gtk::Orientation::Vertical,

                        #[name="navbar"]
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_margin_all: 5,
                            set_height_request: 32,
                            //set_size_request: (-1, 32),
                            //set_hexpand: true,

                            model.navbar.widget(),
                            gtk::SearchEntry {
                                set_hexpand: true,
                            }
                        },
                        model.filesview.widget() {
                            set_min_content_width: 200,
                            set_vexpand: true,
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                        }
                    },

                    #[name="argo_panels"]
                    #[wrap(Some)]
                    set_end_widget = &gtk::Box {
                        set_hexpand: true,
                        set_size_request: (400, -1),
                        gtk::Label {
                            set_label: "TODO: Implement Notebook Panel",
                        }
                    },
                },

                #[name="statusbar"]
                gtk::Box {
                    set_height_request: 32,
                    set_hexpand: true,
                    gtk::Frame {
                        set_hexpand: true,
                        gtk::Label {
                            set_label: "Status Bar",
                        }
                    }
                },
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

        let toolbar = Toolbar::builder()
            .launch(()).forward(sender.input_sender(), |msg| AppMsg::GotoBookmark);

        let navbar = NavigationBar::builder()
            .launch(()).forward(sender.input_sender(), |msg| AppMsg::GotoBookmark);
        
        let model = App { mode, filesview, bookmarksview, toolbar, navbar};
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
