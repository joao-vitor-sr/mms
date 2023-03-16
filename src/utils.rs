use std::{
    env::var,
    fs::File,
    io::{self, BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

use cursive::{
    theme::{BorderStyle, Palette, Theme},
    With,
};

pub fn return_default_cursive_theme() -> Theme {
    Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
            }
        }),
    }
}

pub fn return_songs_root_path() -> Result<String, io::Error> {
    let bail_msg = "Unable to get mpd path";

    let prefix =
        var("XDG_CONFIG_HOME").unwrap_or_else(|_| [&var("HOME").unwrap(), ".config"].join("/"));
    let mpd_conf_path = Path::new(&[&prefix, "mpd", "mpd.conf"].join("/")).to_owned();
    let mpd_conf = File::open(mpd_conf_path)?;

    for x in BufReader::new(mpd_conf).lines().flatten() {
        if x.starts_with("music_directory") {
            let value_vec = x.split_whitespace().collect::<Vec<_>>();
            let mut value = value_vec[1].to_owned();
            value.remove(0);
            value.remove(value.len() - 1);
            if value.starts_with('/') {
                return Ok(value.to_owned());
            } else if value.starts_with('~') {
                value.remove(0);
                return Ok([var("HOME").unwrap(), value].join(""));
            } else {
                return Err(Error::new(ErrorKind::Other, bail_msg));
            }
        }
    }

    return Err(Error::new(ErrorKind::Other, bail_msg));
}
