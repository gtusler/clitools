use std::fmt::Display;

#[derive(Debug)]
pub enum Difference {
    TreeName(String, String),
}

impl Display for Difference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::TreeName(name1, name2) => format!("Different Names: {} - {}", name1, name2),
        };

        write!(f, "{}", msg)
    }
}

pub fn print_differences(differences: &Vec<Difference>) {
    println!("{} differences", differences.len());

    for difference in differences {
        println!("{}", difference);
    }
}
