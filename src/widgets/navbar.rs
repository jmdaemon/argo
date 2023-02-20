use gtk::prelude::ButtonExt;
use relm4::{
    component,
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts
};

#[derive(Debug)]
pub struct NavigationBar {}

#[component(pub)]
impl SimpleComponent for NavigationBar {
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Box {

            gtk::Button {
                set_label: "<-",
            },
            gtk::Button {
                set_label: "->",
            },
            gtk::Button {
                set_label: "^",
            },
            gtk::Button {
                set_label: "Home",
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NavigationBar {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    //fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        //match message {

        //}
    //}
}
