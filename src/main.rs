use argo::widgets::app::App;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("io.github.jmdaemon.argo");
    app.run::<App>(());
}
