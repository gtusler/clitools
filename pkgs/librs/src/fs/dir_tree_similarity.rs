use crate::fs::highlight_file_name::highlight_file_name_string;
use color_print::cprintln;
use std::fmt::Display;

pub enum Similarity {
    TreeName(String, String),
    SameFileName(String, String),
}

impl Display for Similarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::TreeName(name1, name2) => format!("Same names: {} - {}", name1, name2),
            Self::SameFileName(path1, path2) => {
                format!(
                    "Same file names:\n\t{}\n\t{}",
                    highlight_file_name_string(path1),
                    highlight_file_name_string(path2)
                )
            }
        };

        write!(f, "{}", msg)
    }
}

pub fn print_similarities(similarities: &Vec<Similarity>) {
    if similarities.len() == 0 {
        return print_no_similarities();
    }

    let suffix = if similarities.len() == 1 { "y" } else { "ies" };
    cprintln!("<y>{}</y> similarit{}", similarities.len(), suffix);

    for similarity in similarities {
        println!("{}", similarity);
    }
}

fn print_no_similarities() {
    cprintln!("<g>No similarities</>");
}
