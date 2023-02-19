use argo::{widgets::app::App, consts::APP_ID};
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<App>(());
}
