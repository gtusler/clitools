use super::{
    error::TreeError,
    leaf::{dirs_only, filter_extensions, Leaf},
    util::{CHAR_DOWN_END, CHAR_DOWN_JOIN},
};
use crate::fs::extension::Extension;
use std::{fmt::Display, path::Path};

/// Essentially a wrapper around `Leaf` providing some convenience functions.
#[derive(Debug)]
pub struct Tree {
    root_name: String,
    leaves: Vec<Leaf>,
}

/// Construction
impl Tree {
    pub fn new(root_name: String, leaves: Vec<Leaf>) -> Self {
        Self { root_name, leaves }
    }

    /// Assumes that the path exists and is a dir
    pub fn from_path(dir: &Path, recursive: bool) -> Result<Self, TreeError> {
        let maybe_leaves = Leaf::gather_children(dir, recursive);

        let leaves = match maybe_leaves {
            Ok(ls) => ls,
            Err(e) => return Err(TreeError::LeafError(e)),
        };

        Ok(Self {
            root_name: dir.display().to_string(),
            leaves,
        })
    }
}

/// Getting props
impl Tree {
    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn leaves(&self) -> &Vec<Leaf> {
        &self.leaves
    }
}

/// Filters
impl Tree {
    /// Remove any leaves which represent files.
    pub fn dirs_only(&mut self) {
        let mut new_leaves: Vec<Leaf> = vec![];

        for leaf in self.leaves.iter() {
            match dirs_only(leaf) {
                Some(new_leaf) => {
                    new_leaves.push(new_leaf);
                }
                None => (),
            }
        }

        self.leaves = new_leaves
    }

    /// Filter the tree so that it only contains files with the given extension
    /// and the directories that contain them.
    pub fn with_extension(&mut self, extension: &Extension) {
        let mut new_leaves: Vec<Leaf> = vec![];
        let extensions: Vec<Extension> = vec![extension.clone()];

        for leaf in self.leaves.iter() {
            if let Some(new_leaf) = filter_extensions(leaf, &extensions) {
                new_leaves.push(new_leaf);
            }
        }

        self.leaves = new_leaves;
    }

    /// Filter the tree so that it only contains files with the given extensions
    /// and the directories that contain them.
    pub fn with_extensions(&mut self, extensions: &Vec<Extension>) {
        let mut new_leaves: Vec<Leaf> = vec![];

        for leaf in self.leaves.iter() {
            if let Some(new_leaf) = filter_extensions(leaf, &extensions) {
                new_leaves.push(new_leaf);
            }
        }

        self.leaves = new_leaves;
    }
}

/// Printing
impl Tree {
    pub fn print_top_level(&self) {
        for leaf in self.leaves.iter() {
            leaf.print_ugly();
        }
    }

    pub fn print_tree(&self) -> String {
        let mut output: Vec<String> = vec![self.root_name.clone()];

        let leaves_len = self.leaves.len();
        let mut stripes: Vec<u8> = Vec::new();

        for (i, leaf) in self.leaves.iter().enumerate() {
            let is_last = i == leaves_len - 1;
            let joiner_char = if is_last {
                CHAR_DOWN_END
            } else {
                CHAR_DOWN_JOIN
            };

            output.push(leaf.print_as_tree_leaf(is_last, joiner_char, 0, &mut stripes));
        }

        output.join("\n")
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_tree())
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.leaves == other.leaves
    }
}

impl Eq for Tree {}
