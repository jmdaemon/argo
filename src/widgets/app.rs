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
    gtk,
    Controller,
    ComponentParts,
    ComponentSender,
    RelmWidgetExt,
    SimpleComponent,
    Component, ComponentController
};

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
    Quit,
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
