use std::{fmt::Display, fs, path::Path};
use super::dir_tree_error::{LeafError, TreeError};

const CHAR_SPACE: &str = "   ";
const CHAR_DOWN: &str = "│  ";
const CHAR_DOWN_JOIN: &str = "├─ ";
const CHAR_DOWN_END: &str = "└─ ";

#[derive(Debug)]
pub struct Tree {
    root_name: String,
    leaves: Vec<Leaf>,
}

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

    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn leaves(&self) -> &Vec<Leaf> {
        &self.leaves
    }

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

#[derive(Debug, PartialEq, Eq)]
pub enum Leaf {
    Dir {
        name: String,
        fullpath: String,
        children: Vec<Leaf>,
    },
    File {
        name: String,
        fullpath: String,
    },
}

impl Leaf {
    pub fn from_path(path: &Path, recursive: bool) -> Result<Self, LeafError> {
        if path.is_file() {
            return Self::from_file_path(path);
        }

        if path.is_dir() {
            return Self::from_dir_path(path, recursive);
        }

        Err(LeafError::UnsupportedType)
    }

    /// Assumes that the `path.is_file()` check has already been done
    pub fn from_file_path(path: &Path) -> Result<Self, LeafError> {
        match path.file_name() {
            None => return Err(LeafError::NoName),
            Some(name) => {
                let name_string = name.to_str().unwrap().to_string();
                return Ok(Leaf::File {
                    name: name_string,
                    fullpath: path.display().to_string(),
                });
            }
        }
    }

    /// Assumes that the `path.is_dir()` check has already been done
    pub fn from_dir_path(path: &Path, recursive: bool) -> Result<Self, LeafError> {
        match path.file_name() {
            None => return Err(LeafError::NoName),
            Some(name) => {
                let name = name.to_str().unwrap().to_string();
                let children = if recursive {
                    // gather children
                    match Self::gather_children(path, recursive) {
                        Ok(c) => c,
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    Vec::new()
                };
                return Ok(Leaf::Dir {
                    name,
                    fullpath: path.display().to_string(),
                    children,
                });
            }
        }
    }

    pub fn gather_children(dir: &Path, recursive: bool) -> Result<Vec<Leaf>, LeafError> {
        let mut leaves: Vec<Leaf> = Vec::new();

        let entries = match fs::read_dir(dir) {
            Ok(es) => es,
            Err(e) => return Err(LeafError::IoError(e)),
        };

        for maybe_entry in entries {
            let entry = match maybe_entry {
                Ok(e) => e,
                Err(e) => return Err(LeafError::IoError(e)),
            };
            let entry_path = entry.path();

            let maybe_leaf = Leaf::from_path(&entry_path, recursive);
            if let Ok(leaf) = maybe_leaf {
                leaves.push(leaf)
            }
        }

        Ok(leaves)
    }

    pub fn name(&self) -> &str {
        match self {
            Leaf::Dir { name, .. } => name,
            Leaf::File { name, .. } => name,
        }
    }

    pub fn fullpath(&self) -> &str {
        match self {
            Leaf::Dir { fullpath, .. } => fullpath,
            Leaf::File { fullpath, .. } => fullpath,
        }
    }

    pub fn leaf_type(&self) -> LeafType {
        match self {
            Leaf::Dir { .. } => LeafType::Dir,
            Leaf::File { .. } => LeafType::File,
        }
    }

    pub fn print_ugly(&self) -> String {
        let lines: Vec<String> = match self {
            Self::Dir {
                name,
                fullpath,
                children,
            } => {
                vec![
                    format!("Dir: {}", name),
                    format!("Fullpath: {}", fullpath),
                    format!("Children: {}", children.len()),
                ]
            }
            Self::File { name, fullpath } => {
                vec![format!("File: {}", name), format!("Fullpath: {}", fullpath)]
            }
        };

        lines.join("\n")
    }

    pub fn print_as_tree_leaf(
        &self,
        is_last: bool,
        joiner: &str,
        indent_level: u8,
        stripes: &mut Vec<u8>,
    ) -> String {
        match self {
            Leaf::File { name, .. } => {
                format!(
                    "{}{}{}",
                    make_stripy_prefix(indent_level, stripes),
                    joiner,
                    name
                )
            }
            Leaf::Dir { name, children, .. } => {
                let children_len = children.len();
                let mut output: Vec<String> = vec![format!(
                    "{}{}{} ({})",
                    make_stripy_prefix(indent_level, stripes),
                    joiner,
                    name,
                    children_len
                )];

                for (i, child) in children.iter().enumerate() {
                    let is_last_child = i == children_len - 1;
                    let child_joiner_char = if is_last_child {
                        CHAR_DOWN_END
                    } else {
                        CHAR_DOWN_JOIN
                    };
                    let child_indent_level = indent_level + 1;

                    let stripe_added: Option<u8> = if is_last {
                        None
                    } else {
                        stripes.push(child_indent_level - 1);
                        Some(child_indent_level - 1)
                    };

                    output.push(child.print_as_tree_leaf(
                        is_last_child,
                        child_joiner_char,
                        child_indent_level,
                        stripes,
                    ));

                    if let Some(added) = stripe_added {
                        if let Some(pos) = stripes.iter().position(|el| *el == added) {
                            stripes.remove(pos);
                        }
                    }
                }

                output.join("\n")
            }
        }
    }
}

fn make_stripy_prefix(indent_level: u8, stripes: &[u8]) -> String {
    let mut output = String::new();

    for i in 0..indent_level {
        let is_line_kept = stripes.contains(&i);

        if is_line_kept {
            output.push_str(CHAR_DOWN);
        } else {
            output.push_str(CHAR_SPACE);
        }
    }

    output
}

pub enum LeafType {
    File,
    Dir,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod stripy_prefix {
        use super::*;

        #[test]
        fn nothing() {
            assert_eq!(make_stripy_prefix(0, &mut Vec::new()), String::from(""));
        }

        #[test]
        fn top_level() {
            assert_eq!(make_stripy_prefix(1, &mut Vec::new()), String::from("   "));
            assert_eq!(make_stripy_prefix(1, &mut vec![0]), String::from("│  "));
        }

        #[test]
        fn two_levels() {
            assert_eq!(
                make_stripy_prefix(2, &mut Vec::new()),
                String::from("      ")
            );
            assert_eq!(make_stripy_prefix(2, &mut vec![0]), String::from("│     "));
            assert_eq!(
                make_stripy_prefix(2, &mut vec![0, 1]),
                String::from("│  │  ")
            );
        }

        #[test]
        fn three_levels() {
            assert_eq!(
                make_stripy_prefix(3, &mut vec![]),
                String::from("         ")
            );
            assert_eq!(
                make_stripy_prefix(3, &mut vec![0]),
                String::from("│        ")
            );
            assert_eq!(
                make_stripy_prefix(3, &mut vec![0, 1]),
                String::from("│  │     ")
            );
            assert_eq!(
                make_stripy_prefix(3, &mut vec![0, 1, 2]),
                String::from("│  │  │  ")
            );
        }
    }
}
