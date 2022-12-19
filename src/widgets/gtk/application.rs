use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::ArgoWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ArgoApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for ArgoApplication {
        const NAME: &'static str = "ArgoApplication";
        type Type = super::ArgoApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for ArgoApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.instance();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for ArgoApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.instance();
            // Get the current window or create one if necessary
            //let window = if let Some(window) = application.active_window() {
                //window
            //} else {
                //let window = ArgoWindow::new(&*application);
                //window.upcast()
            //};

            //let window = if let Some(window) = application.active_window() {
            //let window = {
                //ArgoWindow::new(&*application);
                //window.upcast()
            //}
            //if let Some(window)
            //let window = 
            //let mut window = ArgoWindow::new(&*application);
            //window = window.upcast();
            let window: ArgoWindow = ArgoWindow::new(&*application).upcast();
            window.present();
        }
    }

    impl GtkApplicationImpl for ArgoApplication {}
    }

glib::wrapper! {
    pub struct ArgoApplication(ObjectSubclass<imp::ArgoApplication>)
        @extends gio::Application, gtk::Application, 
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ArgoApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]).unwrap();
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("argo")
            .logo_icon_name("io.github.jmdaemon.argo")
            .version(VERSION)
            .authors(vec!["Unknown".into()])
            .copyright("© 2022 Unknown")
            .build();

        about.present();
    }
}
