use cursive::{
    view::Scrollable,
    views::{Button, Dialog, LinearLayout, SelectView, TextView},
    Cursive,
};
use cursive_tree_view::TreeView;

use crate::{
    clear_queue, push_song, return_queue_songs,
    tree::TreeEntry,
    ui::{Ui, TREE_SONGS_PATH},
    ModelData,
};

pub fn handle_global_callback(c: &mut Cursive) {
    c.add_global_callback('q', quit);
    c.add_global_callback('j', select_down);
    c.add_global_callback('k', select_up);
    c.add_global_callback('l', push_song_tree);
    c.add_global_callback('?', help);
    c.add_global_callback('u', display_queue);
    c.add_global_callback('c', clear_queue_action);
}

fn display_queue(c: &mut Cursive) {
    let data: Option<&mut ModelData> = c.user_data();
    if let None = data {
        return;
    }
    let data = data.unwrap();
    let addr = data.mpd_addr.clone();
    let queue = return_queue_songs(&addr);
    if let Ok(queue) = queue {
        c.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(Button::new("Cancel", |c| {
                        c.pop_layer();
                    }))
                    .child(SelectView::new().with_all_str(queue).scrollable()),
            )
            .title("Queue"),
        )
    }
}

fn clear_queue_action(c: &mut Cursive) {
    let data: Option<&mut ModelData> = c.user_data();
    if let None = data {
        return;
    }
    let data = data.unwrap();
    let addr = data.mpd_addr.clone();
    if let Err(_) = clear_queue(&addr) {
        return;
    };
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
        c.add_layer(Ui::return_pop_up_text("Failed to get mpd connection data"));
        return;
    }
    let data = data.unwrap();
    let addr = data.mpd_addr.clone();

    let result = c.call_on_name(TREE_SONGS_PATH, move |tree: &mut TreeView<TreeEntry>| {
        let row = tree.row();
        if let None = row {
            return Some("Failed to get selected row");
        }
        let row = row.unwrap();
        let item = tree.borrow_item(row);
        if let None = item {
            return Some("Failed to retrieve item from the row");
        }
        let item = item.unwrap();

        let result = push_song(&addr, &item.path);
        if let Err(_) = result {
            return Some("Failed to push song into queue");
        }
        return None;
    });
    let result = result.unwrap();

    if let Some(err) = result {
        c.add_layer(Ui::return_pop_up_text(err));
    }
}

fn help(c: &mut Cursive) {
    c.add_layer(
        Dialog::new()
            .title("HELP")
            .content(TextView::new(include_str!("../help.md")))
            .dismiss_button("Close"),
    )
}
