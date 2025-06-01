use super::{difference::Difference, similarity::Similarity, tree::Tree};

pub struct TreeComparison<'a> {
    tree1: &'a Tree,
    tree2: &'a Tree,
}

impl<'a> TreeComparison<'a> {
    pub fn new<'b>(tree1: &'b Tree, tree2: &'b Tree) -> TreeComparison<'a>
    where
        'b: 'a,
    {
        Self { tree1, tree2 }
    }

    pub fn is_identical(&self) -> bool {
        self.tree1 == self.tree2
    }

    pub fn is_same_name(&self) -> bool {
        self.tree1.root_name() == self.tree2.root_name()
    }

    pub fn find_differences(&self) -> Vec<Difference> {
        let mut differences: Vec<Difference> = Vec::new();

        if !self.is_same_name() {
            differences.push(Difference::TreeName(
                self.tree1.root_name().to_owned(),
                self.tree2.root_name().to_owned(),
            ));
        }

        differences
    }

    pub fn find_similarities(&self) -> Vec<Similarity> {
        let mut similarities: Vec<Similarity> = Vec::new();

        if self.is_same_name() {
            similarities.push(Similarity::TreeName(
                self.tree1.root_name().to_owned(),
                self.tree2.root_name().to_owned(),
            ));
        }

        for leaf1 in self.tree1.leaves() {
            for leaf2 in self.tree2.leaves() {
                if leaf1.name() == leaf2.name() {
                    similarities.push(Similarity::SameFileName(
                        leaf1.fullpath().to_owned(),
                        leaf2.fullpath().to_owned(),
                    ));
                }
            }
        }

        similarities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::dir_tree::leaf::Leaf;

    mod is_same_name {
        use super::*;

        #[test]
        fn it_fails_with_different_names() {
            let tree1 = Tree::new(String::from("testings"), vec![]);
            let tree2 = Tree::new(String::from("some other name"), vec![]);
            let tree_comparison = TreeComparison::new(&tree1, &tree2);
            assert_eq!(tree_comparison.is_same_name(), false);
        }

        #[test]
        fn it_succeeds_with_same_names() {
            let tree1 = Tree::new(String::from("testings"), vec![]);
            let tree2 = Tree::new(String::from("testings"), vec![]);
            let tree_comparison = TreeComparison::new(&tree1, &tree2);
            assert_eq!(tree_comparison.is_same_name(), true);
        }
    }

    mod is_identical {
        use super::*;
        use crate::fs::extension::Extension;

        #[test]
        fn it_doesnt_include_name() {
            let tree1 = Tree::new(String::from("testings"), vec![]);
            let tree2 = Tree::new(String::from("some other name"), vec![]);
            let tree_comparison_1 = TreeComparison::new(&tree1, &tree2);

            let tree3 = Tree::new(String::from("testings"), vec![]);
            let tree4 = Tree::new(String::from("testings"), vec![]);
            let tree_comparison_2 = TreeComparison::new(&tree3, &tree4);

            assert_eq!(tree_comparison_1.is_identical(), true);
            assert_eq!(tree_comparison_2.is_identical(), true);
        }

        #[test]
        fn it_succeeds_with_one_file() {
            let tree1 = Tree::new(
                String::from("testings"),
                vec![Leaf::File {
                    name: String::from("untitled.doc"),
                    fullpath: String::from("/testings/untitled.doc"),
                    extension: Some(Extension::Doc),
                }],
            );
            let tree2 = Tree::new(
                String::from("testings"),
                vec![Leaf::File {
                    name: String::from("untitled.doc"),
                    fullpath: String::from("/testings/untitled.doc"),
                    extension: Some(Extension::Doc),
                }],
            );
            let tree_comparison = TreeComparison::new(&tree1, &tree2);
            assert_eq!(tree_comparison.is_identical(), true);
        }

        #[test]
        fn it_fails_with_one_file() {
            let tree1 = Tree::new(
                String::from("testings"),
                vec![Leaf::File {
                    name: String::from("untitled.doc"),
                    fullpath: String::from("/testings/untitled.doc"),
                    extension: Some(Extension::Doc),
                }],
            );
            let tree2 = Tree::new(
                String::from("testings"),
                vec![Leaf::File {
                    name: String::from("untitled.txt"),
                    fullpath: String::from("/testings/untitled.txt"),
                    extension: Some(Extension::Txt),
                }],
            );
            let tree_comparison = TreeComparison::new(&tree1, &tree2);
            assert_eq!(tree_comparison.is_identical(), false);
        }
    }
}
