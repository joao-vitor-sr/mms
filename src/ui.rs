use crate::{
    input::handle_global_callback,
    tree::{expand_tree, TreeEntry},
    utils, ModelData,
};
use cursive::{
    view::{Nameable, Scrollable},
    views::{Dialog, TextView},
    Cursive, CursiveExt,
};
use cursive_tree_view::{Placement, TreeView};
use std::path::PathBuf;

pub struct Ui {
    c: Cursive,
}

pub const TREE_SONGS_PATH: &str = "tree_songs";

impl Ui {
    pub fn set_user_data(&mut self, data: ModelData) {
        self.c.set_user_data(data);
    }

    pub fn new() -> Self {
        let mut c = Cursive::default();
        c.set_theme(utils::return_default_cursive_theme());
        Self { c }
    }

    pub fn draw_main_layout(&mut self, path: PathBuf) {
        // Create TreeView with initial working directory
        let mut tree = TreeView::<TreeEntry>::new();

        tree.insert_item(
            TreeEntry {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                dir: Some(path.clone()),
                path: path.clone(),
            },
            Placement::After,
            0,
        );

        expand_tree(&mut tree, 0, &path);

        // Lazily insert directory listings for sub nodes
        tree.set_on_collapse(|siv: &mut Cursive, row, is_collapsed, children| {
            if !is_collapsed && children == 0 {
                siv.call_on_name(TREE_SONGS_PATH, move |tree: &mut TreeView<TreeEntry>| {
                    if let Some(dir) = tree.borrow_item(row).unwrap().dir.clone() {
                        expand_tree(tree, row, &dir);
                    }
                });
            }
        });

        self.c
            .add_layer(Dialog::around(tree.with_name(TREE_SONGS_PATH).scrollable()));
    }

    pub fn run(&mut self) {
        handle_global_callback(&mut self.c);
        self.c.run();
    }
}
