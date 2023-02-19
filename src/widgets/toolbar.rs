use gtk::prelude::ButtonExt;
use relm4::{
    component,
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts
};

#[derive(Debug)]
pub struct Toolbar { }

#[component(pub)]
impl SimpleComponent for Toolbar {
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Box {

            gtk::Button {
                set_label: "Settings",
            },

            gtk::Button {
                set_label: "New Folder",
            },

            gtk::Button {
                set_label: "Delete",
            },

            gtk::Button {
                set_label: "Rename",
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Toolbar {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    //fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        //match message {

        //}
    //}
}
