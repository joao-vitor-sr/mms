use cursive::Cursive;
use cursive_tree_view::TreeView;

use crate::{tree::TreeEntry, ui::TREE_SONGS_PATH};

pub fn handle_global_callback(c: &mut Cursive) {
    c.add_global_callback('q', quit);
    c.add_global_callback('j', select_down);
    c.add_global_callback('k', select_up);
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
