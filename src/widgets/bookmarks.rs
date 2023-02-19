use std::path::PathBuf;

use gtk::prelude::{
    BoxExt,
    OrientableExt,
    WidgetExt,
};

use relm4::{
    component,
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts,
    factory,
    factory::{FactoryView, FactoryVecDeque},
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,

};

use dirs::{audio_dir,  home_dir, desktop_dir, download_dir, picture_dir, video_dir};

// Bookmark
#[derive(Debug)]
pub struct Bookmark {
    pub title: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum BookmarkMsg { }

#[derive(Debug)]
pub enum BookmarkOutput { }

#[factory(pub)]
impl FactoryComponent for Bookmark {
    type Init = Bookmark;
    type Input = BookmarkMsg;
    type Output = BookmarkOutput;
    type CommandOutput = ();
    type ParentInput = BookmarksViewMsg;
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBox {
            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.title,
                set_width_chars: 24,
            },
        }
    }

    fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        init
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    //fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        //match message {}
    //}

    //fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        //let output = match output {};
        //Some(output)
    //}
}

// Handling multiple bookmark types
// TODO:
// All bookmarks may have
//      - a title (required)
//      - a path (optional)
//      - callback (optional)
#[derive(Debug)]
pub struct BookmarkEntry {
    bookmark_type: BookmarkType,
}

#[derive(Debug)]
pub enum BookmarkType {
    Group(String, Vec<BookmarkType>),
    Label(String, gtk::Label),
    Separator(String, gtk::Separator),
    Bookmark(String, PathBuf),
}

// TODO: Figure out how to handle multiple bookmark types including nested types

#[derive(Debug)]
pub struct BookmarksView {
    bookmarks: FactoryVecDeque<Bookmark>,
}

#[derive(Debug)]
pub enum BookmarksViewMsg {}

#[derive(Debug)]
pub enum BookmarksViewOutput {}

//pub struct BookmarksViewInit {}

#[component(pub)]
impl SimpleComponent for BookmarksView {
    type Input = BookmarksViewMsg;
    type Output = BookmarksViewOutput;
    //type Init = BookmarksViewInit;
    type Init = ();

    view! {
        #[root]
        gtk::ScrolledWindow {
            #[local_ref]
            bookmarks_box -> gtk::ListBox {
                set_vexpand: true,
                set_hexpand: true,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        // TODO: Load bookmarks from disc
        // TODO: If not found, then create default bookmarks tab
        // For now we'll just stub it
        let mut bookmarks = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());
        let defaults = vec![
            ("Home".to_owned(), home_dir().unwrap()),
            ("Music".to_owned(), audio_dir().unwrap()),
            ("Desktop".to_owned(), desktop_dir().unwrap()),
            ("Downloads".to_owned(), download_dir().unwrap()),
            ("Pictures".to_owned(), picture_dir().unwrap()),
            ("Videos".to_owned(), video_dir().unwrap()),
        ];

        for (title, path) in defaults {
            //let bookmark_type = BookmarkType::Bookmark(title, path);
            //let bookmark_entry = BookmarkEntry { bookmark_type }; 
            //bookmarks.guard().push_back(bookmark_entry);
            let bookmark = Bookmark{ title, path };
            bookmarks.guard().push_back(bookmark);
        }

        let model = BookmarksView { bookmarks };
        let bookmarks_box = model.bookmarks.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {

        }
    }
}
