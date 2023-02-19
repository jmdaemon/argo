use std::path::PathBuf;

use relm4::{
    //component,
    gtk,
    //Component,
    //ComponentSender,
    //ComponentParts,
    factory,
    factory::FactoryView,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,

};

// Bookmark
#[derive(Debug)]
pub struct Bookmark {
    pub title: String,
    pub path: PathBuf,
}

// Stores and manages groups of bookmarks
#[derive(Debug)]
pub struct BookmarkGroup {
    pub bookmarks: Vec<Bookmark>,
}

// Bookmarks
pub struct BookmarksFactory {

}

#[derive(Debug)]
pub enum BookmarksFactoryMsg {
    //AddBookmark,
    //RemoveBookmark,
    //RemoveGroup,
    //ReorderBookmark,
    //ReorderGroup,
}

#[derive(Debug)]
pub enum BookmarksFactoryOutput {}

pub struct BookmarksFactoryInit {}

#[factory(pub)]
impl FactoryComponent for BookmarksFactory {
    type ParentWidget = gtk::ListBox;
    type ParentInput = ();
    type Input = BookmarksFactoryMsg;
    type Output = BookmarksFactoryOutput;
    type Init = BookmarksFactoryInit;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::ListBox {

        }
    }

    fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        Self {}
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
