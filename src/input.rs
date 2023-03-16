use cursive::Cursive;
use cursive_tree_view::TreeView;

use crate::{push_song, tree::TreeEntry, ui::TREE_SONGS_PATH, ModelData};

pub fn handle_global_callback(c: &mut Cursive) {
    c.add_global_callback('q', quit);
    c.add_global_callback('j', select_down);
    c.add_global_callback('k', select_up);
    c.add_global_callback('l', push_song_tree);
}

fn quit(c: &mut Cursive) {
    c.quit();
}

fn select_down(c: &mut Cursive) {
    c.call_on_name(TREE_SONGS_PATH, move |tree: &mut TreeView<TreeEntry>| {
        tree.focus_down(1);
    });
}

fn select_up(c: &mut Cursive) {
    c.call_on_name(TREE_SONGS_PATH, move |tree: &mut TreeView<TreeEntry>| {
        tree.focus_up(1);
    });
}

fn push_song_tree(c: &mut Cursive) {
    let data: Option<&mut ModelData> = c.user_data();
    if let None = data {
        return;
    }
    let data = data.unwrap();
    let addr = data.mpd_addr.clone();
    c.call_on_name(TREE_SONGS_PATH, move |tree: &mut TreeView<TreeEntry>| {
        let row = tree.row();
        if let None = row {
            return;
        }
        let row = row.unwrap();
        let item = tree.borrow_item(row);
        if let None = item {
            return;
        }
        let item = item.unwrap();

        push_song(&addr, &item.path).unwrap();
    });
}
