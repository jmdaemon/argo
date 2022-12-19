// Import widgets
use crate::widgets::gtk::mainwidget::imp::MainWidget;
use crate::widgets::gtk::mainwidget::MainWidget as MW;

// GTK
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jmdaemon/argo/resources/gtk/window.ui")]
    pub struct ArgoWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        
        pub mainwidget: MainWidget,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ArgoWindow {
        const NAME: &'static str = "ArgoWindow";
        type Type = super::ArgoWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) { klass.bind_template(); }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) { obj.init_template(); }
    }

    impl ObjectImpl for ArgoWindow {}
    impl WidgetImpl for ArgoWindow {}
    impl WindowImpl for ArgoWindow {}
    impl ApplicationWindowImpl for ArgoWindow {}
}

glib::wrapper! {
    pub struct ArgoWindow(ObjectSubclass<imp::ArgoWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl ArgoWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: ArgoWindow = glib::Object::new(&[("application", application)]);
        let mw = MW::new();

        window.set_title(Some(&"Argo"));
        window.set_child(Some(&mw));
        window
    }
}
