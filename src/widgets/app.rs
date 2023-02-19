use crate::widgets::files::{
    FilesView,
    FilesViewOutput
};

use std::path::PathBuf;

use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt
};

use relm4::{
    component,
    gtk::{self, prelude::ApplicationExt},
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
    AddFileCard,
    RemoveFileCard,
    Close,
}

#[derive(Debug)]
pub struct App {
    mode: AppMode,
    filesview: Controller<FilesView>,
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
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                // Quit/Exit Button
                gtk::Button {
                    set_label: "Quit",
                    // Emit quit signal
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Close);
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

                // Custom Widgets
                model.filesview.widget(),
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
        
        let model = App { mode, filesview };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

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
            AppMsg::Close => {
                relm4::main_application().quit();
            }
        }
    }
}
