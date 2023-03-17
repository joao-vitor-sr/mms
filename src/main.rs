use mpd::{Client, Song};
use std::{
    collections::BTreeMap,
    fs::read_dir,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};
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

    let model_data = ModelData {
        mpd_addr: "127.0.0.1:6600".to_string(),
    };

    ui.set_user_data(model_data);
    ui.draw_main_layout(songs_path);
    ui.run();
}

pub struct ModelData {
    mpd_addr: String,
}

fn return_mpd_conn(addr: &str) -> Result<Client, io::Error> {
    let conn = Client::connect(addr);
    if let Err(e) = conn {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Unable to reach mpd server, error: {e}"),
        ));
    }
    let conn = conn.unwrap();
    return Ok(conn);
}

fn return_song(path: String) -> Song {
    Song {
        file: path,
        name: None,
        title: None,
        last_mod: None,
        duration: None,
        place: None,
        range: None,
        tags: BTreeMap::new(),
    }
}

// @TODO refactor the push
pub fn push_song(addr: &str, song_path: &Path) -> Result<(), io::Error> {
    let mut conn = return_mpd_conn(addr)?;

    // validates if it's a directory
    if song_path.is_dir() {
        for entry in read_dir(song_path)? {
            let entry = entry?;
            let song = return_song(entry.path().as_os_str().to_str().unwrap().to_string());

            let result = conn.push(song);
            if let Err(_) = result {
                continue;
            }
        }
    } else {
        let song = return_song(song_path.as_os_str().to_str().unwrap().to_string());

        let result = conn.push(song);
        if let Err(e) = result {
            return Err(Error::new(
                ErrorKind::Other,
                format!("MPD: Unable to push a song to mpd: {e}"),
            ));
        }
    }
    Ok(())
}

pub fn return_queue_songs(addr: &str) -> Result<Vec<String>, io::Error> {
    let mut conn = return_mpd_conn(addr)?;

    let queue = conn.queue();
    if let Err(e) = queue {
        return Err(Error::new(
            ErrorKind::Other,
            format!("ERROR: unable to recieve mpd queue, {e}"),
        ));
    }

    Ok(queue.unwrap().iter().map(|i| i.file.to_owned()).collect())
}

pub fn clear_queue(addr: &str) -> Result<(), io::Error> {
    let mut conn = return_mpd_conn(addr)?;

    let result = conn.clear();
    if let Err(e) = result {
        return Err(Error::new(
            ErrorKind::Other,
            format!("ERROR: unable to clear mpd queue, {e}"),
        ));
    }

    Ok(())
}
