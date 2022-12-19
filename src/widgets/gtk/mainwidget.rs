// Standard Library
use std::cell::RefCell;

// Third Party Libraries

// GTK
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

pub mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/jmdaemon/argo/resources/gtk/mainwidget.ui")]
    pub struct MainWidget {
        // Template components
        #[template_child]
        pub lv_bookmarks: TemplateChild<gtk::ListView>,
        #[template_child]
        pub se_searchbar: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub gv_files: TemplateChild<gtk::GridView>,
        #[template_child]
        pub nb_sidebar: TemplateChild<gtk::Notebook>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWidget {
        const NAME: &'static str = "MainWidget";
        type Type = super::MainWidget;
        type ParentType = gtk::Grid;

        // Initialize template
        fn class_init(klass: &mut Self::Class) { Self::bind_template(klass); }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) { obj.init_template(); }
    }

    impl ObjectImpl for MainWidget {
        fn constructed(&self) { }
    }
    impl WidgetImpl for MainWidget {}
    impl GridImpl for MainWidget {}
}

glib::wrapper! {
    pub struct MainWidget(ObjectSubclass<imp::MainWidget>)
        @extends gtk::Widget, gtk::Grid,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MainWidget {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
