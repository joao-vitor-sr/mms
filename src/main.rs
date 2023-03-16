use std::path::PathBuf;
use ui::Ui;

mod input;
mod tree;
mod ui;
mod utils;

fn main() {
    let mut ui = Ui::new();

    let songs_path = utils::return_songs_root_path();
    if let Err(e) = songs_path {
        eprintln!("ERROR: Unable to get songs root path, error: {e}");
        return;
    }
    let songs_path = PathBuf::from(songs_path.unwrap());

    ui.draw_main_layout(songs_path);
    ui.run();
}
