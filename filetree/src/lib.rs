// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_must_use)]
#![deny(dead_code)]
#![deny(clippy::all, clippy::perf, clippy::nursery, clippy::pedantic)]
#![deny(clippy::expect_used)]
#![deny(clippy::filetype_is_file)]
#![deny(clippy::cargo)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::match_like_matches_macro)]
#![deny(clippy::needless_update)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]

mod error;
mod filetree;
mod filetreeitems;
mod item;
mod tree_iter;
mod treeitems_iter;

pub use crate::{
    filetree::FileTree,
    filetree::MoveSelection,
    item::{FileTreeItem, TreeItemInfo},
};
