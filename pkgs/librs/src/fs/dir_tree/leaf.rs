use super::{
    error::LeafError,
    util::{CHAR_DOWN_END, CHAR_DOWN_JOIN},
};
use crate::fs::{dir_tree::util::make_stripy_prefix, extension::Extension};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf {
    Dir {
        name: String,
        fullpath: String,
        children: Vec<Leaf>,
    },
    File {
        name: String,
        fullpath: String,
        extension: Option<Extension>,
    },
}

/// Construction
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
                    extension: Extension::extract(&name_string),
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

    /// Gathers all the children of the given path.
    /// Assumes that the given path is definitely a directory and will panic if that's not the
    /// case.
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
}

/// Property getters
impl Leaf {
    /// Get the name of the leaf.
    pub fn name(&self) -> &str {
        match self {
            Leaf::Dir { name, .. } => name,
            Leaf::File { name, .. } => name,
        }
    }

    /// Get the full path of the leaf.
    pub fn fullpath(&self) -> &str {
        match self {
            Leaf::Dir { fullpath, .. } => fullpath,
            Leaf::File { fullpath, .. } => fullpath,
        }
    }

    pub fn full_path(&self) -> PathBuf {
        PathBuf::from(self.fullpath())
    }

    /// Get the children of the leaf, if the leaf is a directory.
    pub fn children(&self) -> Option<Vec<Leaf>> {
        match self {
            Leaf::File { .. } => None,
            Leaf::Dir { children, .. } => Some(children.clone()),
        }
    }
}

/// Checks
impl Leaf {
    /// Get the LeafType of this leaf.
    pub fn leaf_type(&self) -> LeafType {
        match self {
            Leaf::Dir { .. } => LeafType::Dir,
            Leaf::File { .. } => LeafType::File,
        }
    }

    /// Check if this leaf references a file.
    pub fn is_file(&self) -> bool {
        match self {
            Leaf::File { .. } => true,
            _ => false,
        }
    }

    /// Check if this leaf references a directory.
    pub fn is_dir(&self) -> bool {
        match self {
            Leaf::Dir { .. } => true,
            _ => false,
        }
    }
}

/// Printing
impl Leaf {
    /// A very ugly way of printing the leaf.
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
            Self::File { name, fullpath, .. } => {
                vec![format!("File: {}", name), format!("Fullpath: {}", fullpath)]
            }
        };

        lines.join("\n")
    }

    /// A pretty way of printing the leaf as part of a tree.
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

pub enum LeafType {
    File,
    Dir,
}

/// Recursively remove leaves which are not directories.
pub fn dirs_only(leaf: &Leaf) -> Option<Leaf> {
    if leaf.is_file() {
        return None;
    }

    let mut output_children: Vec<Leaf> = vec![];

    for child in leaf.children().expect("we know this is a dir") {
        match dirs_only(&child) {
            Some(child_dir) => {
                output_children.push(child_dir);
            }
            None => (),
        }
    }

    Some(Leaf::Dir {
        name: leaf.name().to_owned(),
        fullpath: leaf.fullpath().to_owned(),
        children: output_children,
    })
}

/// If the leaf is a file, check if its extension is in the given list.
/// If the leaf is a dir, check its children.
pub fn filter_extensions(leaf: &Leaf, extensions: &Vec<Extension>) -> Option<Leaf> {
    if leaf.is_file() {
        match leaf {
            Leaf::File { extension, .. } => {
                if let Some(ext) = extension {
                    if extensions.contains(ext) {
                        return Some(leaf.clone());
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => (),
        }
    }

    if leaf.is_dir() {
        match leaf {
            Leaf::Dir {
                name,
                fullpath,
                children,
            } => {
                let mut new_children: Vec<Leaf> = vec![];

                for child in children {
                    if let Some(filtered) = filter_extensions(child, extensions) {
                        new_children.push(filtered.clone());
                    }
                }

                return Some(Leaf::Dir {
                    name: name.to_owned(),
                    fullpath: fullpath.to_owned(),
                    children: new_children,
                });
            }
            _ => (),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod filter_extensions {
        use super::*;

        #[test]
        fn leaf_file_matches() {
            let leaf = Leaf::File {
                name: "something.txt".to_string(),
                fullpath: "/home/user/something.txt".to_string(),
                extension: Some(Extension::Txt),
            };

            let result = filter_extensions(&leaf, &vec![Extension::Txt]);
            assert_eq!(result, Some(leaf));
        }

        #[test]
        fn leaf_file_no_match() {
            let leaf = Leaf::File {
                name: "something.txt".to_string(),
                fullpath: "/home/user/something.txt".to_string(),
                extension: Some(Extension::Txt),
            };

            let result = filter_extensions(&leaf, &vec![Extension::Png]);
            assert_eq!(result, None);
        }

        #[test]
        fn leaf_dir_one_extension() {
            let leaf = Leaf::Dir {
                name: "something".to_string(),
                fullpath: "/home/user/something".to_string(),
                children: vec![
                    Leaf::File {
                        name: "document.txt".to_string(),
                        fullpath: "/home/user/something/document.txt".to_string(),
                        extension: Some(Extension::Txt),
                    },
                    Leaf::File {
                        name: "image.jpg".to_string(),
                        fullpath: "/home/user/something/image.jpg".to_string(),
                        extension: Some(Extension::Jpg),
                    },
                    Leaf::File {
                        name: "image.png".to_string(),
                        fullpath: "/home/user/something/image.png".to_string(),
                        extension: Some(Extension::Png),
                    },
                ],
            };

            let result_txt = filter_extensions(&leaf, &vec![Extension::Txt]).unwrap();
            let result_jpg = filter_extensions(&leaf, &vec![Extension::Jpg]).unwrap();
            let result_png = filter_extensions(&leaf, &vec![Extension::Png]).unwrap();

            assert_eq!(result_txt.children().unwrap().len(), 1);
            assert_eq!(result_jpg.children().unwrap().len(), 1);
            assert_eq!(result_png.children().unwrap().len(), 1);
        }

        #[test]
        fn leaf_dir_two_extensions() {
            let leaf = Leaf::Dir {
                name: "something".to_string(),
                fullpath: "/home/user/something".to_string(),
                children: vec![
                    Leaf::File {
                        name: "document.txt".to_string(),
                        fullpath: "/home/user/something/document.txt".to_string(),
                        extension: Some(Extension::Txt),
                    },
                    Leaf::File {
                        name: "image.jpg".to_string(),
                        fullpath: "/home/user/something/image.jpg".to_string(),
                        extension: Some(Extension::Jpg),
                    },
                    Leaf::File {
                        name: "image.png".to_string(),
                        fullpath: "/home/user/something/image.png".to_string(),
                        extension: Some(Extension::Png),
                    },
                ],
            };

            let result = filter_extensions(&leaf, &vec![Extension::Jpg, Extension::Png]).unwrap();
            assert_eq!(result.children().unwrap().len(), 2);
        }
    }
}
