use crate::widgets::{
    files::{FilesView, FilesViewOutput},
    bookmarks::BookmarksView,
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
                set_margin_all: 5,
                set_shrink_start_child: false,

                set_halign: gtk::Align::Start,
                #[wrap(Some)]
                set_start_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_size_request: (300, 100),
                    set_hexpand: true,

                    model.bookmarksview.widget(),

                    gtk::Button {
                        set_label: "Quit",
                        // Emit quit signal
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Close);
                        },
                    }
                },

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
