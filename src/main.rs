//use relm::Widget;
//use argo::Win;

use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use gtk::Inhibit;

struct AppModel {
    selected_path: String,
}

#[derive(Debug)]
enum AppInput {
    Quit,
}

struct AppWidgets {
    label: gtk::Label,
}

impl SimpleComponent for AppModel {

    type Input = AppInput;
    type Output = ();
    type Init = u8;
    type Root = gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Argo") // TOOD: The title shown will be the current file path instead
            .default_width(640)
            .default_height(480)
            .build()
    }

    fn init(
        counter: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { selected_path: "/home/jmd/".to_owned() };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let quit_button = gtk::Button::with_label("Quit");

        //let label = gtk::Label::new(Some(&format!("Test Label: {}", model.counter)));
        let label = gtk::Label::new(Some(&format!("Test Label: {}", model.selected_path)));
        //label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        //vbox.append(&inc_button);
        //vbox.append(&dec_button);
        //vbox.append(&label);
        vbox.append(&quit_button);

        // Emit quit signal
        quit_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Quit);
        }));

        let widgets = AppWidgets { label };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Quit => {
                //self.close();
                //return (Some(AppInput::Quit), Inhibit(false))
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        //widgets
            //.label
            //.set_label(&format!("Counter: {}", self.counter));
    }
}


fn main() {
    //Win::run("/home/jmd/test").unwrap();
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
