use std::{
    fmt::Display,
    process::{Command, Stdio},
};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Uname {
    pub kernel_name: UnameField,
    pub nodename: UnameField,
    pub kernel_release: UnameField,
    pub kernel_version: UnameField,
    pub machine: UnameField,
    pub processor: UnameField,
    pub hardware_platform: UnameField,
    pub operating_system: UnameField,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnameField {
    pub short: char,
    pub long: String,
    pub description: String,
    pub value: Option<String>,
}

impl Uname {
    pub fn spawn_and_parse() -> Uname {
        let fields_empty = Uname::fields();
        let mut fields: Vec<UnameField> = Vec::new();
        for field in fields_empty {
            let filled_field = fill_uname_output(&field);
            fields.push(filled_field);
        }

        let kernel_name = fields.first().expect("Failed to get kernel-name").clone();
        let nodename = fields.get(1).expect("Failed to get nodename").clone();
        let kernel_release = fields.get(2).expect("Failed to get kernel-release").clone();
        let kernel_version = fields.get(3).expect("Failed to get kernel-version").clone();
        let machine = fields.get(4).expect("Failed to get machine").clone();
        let processor = fields.get(5).expect("Failed to get processor").clone();
        let hardware_platform = fields
            .get(6)
            .expect("Failed to get hardware-platform")
            .clone();
        let operating_system = fields
            .get(7)
            .expect("Failed to get operating-system")
            .clone();

        Uname {
            kernel_name,
            nodename,
            kernel_release,
            kernel_version,
            machine,
            processor,
            hardware_platform,
            operating_system,
        }
    }

    pub fn man() -> String {
        let lines: Vec<String> = vec![];

        lines.join("\n")
    }

    pub fn fields() -> Vec<UnameField> {
        vec![
            UnameField {
                short: 's',
                long: String::from("kernel-name"),
                description: String::from("print the kernel name"),
                value: None,
            },
            UnameField {
                short: 'n',
                long: String::from("nodename"),
                description: String::from("print the network node name"),
                value: None,
            },
            UnameField {
                short: 'r',
                long: String::from("kernel-release"),
                description: String::from("print the kernel release"),
                value: None,
            },
            UnameField {
                short: 'v',
                long: String::from("kernel-version"),
                description: String::from("print the kernel version"),
                value: None,
            },
            UnameField {
                short: 'm',
                long: String::from("machine"),
                description: String::from("print the machine hardware name"),
                value: None,
            },
            UnameField {
                short: 'p',
                long: String::from("processor"),
                description: String::from("print the processor type (non-portable)"),
                value: None,
            },
            UnameField {
                short: 'i',
                long: String::from("hardware-platform"),
                description: String::from("print the hardware platform (non-portable)"),
                value: None,
            },
            UnameField {
                short: 'o',
                long: String::from("operating-system"),
                description: String::from("print the operating system"),
                value: None,
            },
        ]
    }
}

fn fill_uname_output(field: &UnameField) -> UnameField {
    let mut the_arg = "--".to_string();
    the_arg.push_str(&field.long);
    let uname_child = Command::new("uname")
        .arg(the_arg)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start uname");
    let uname_out = uname_child
        .wait_with_output()
        .expect("Failed to open uname stdout");
    let uname_ascii = uname_out
        .stdout
        .as_ascii()
        .expect("uname output aint ascii");
    let uname_str = uname_ascii.as_str();

    UnameField {
        short: field.short,
        long: field.long.clone(),
        description: field.description.clone(),
        value: Some(uname_str.replace("\n", "")),
    }
}

impl Display for Uname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = vec![
            format!("Kernel Name: {}", self.kernel_name),
            format!("Node Name: {}", self.nodename),
            format!("Kernel Release: {}", self.kernel_release),
            format!("Kernel Version: {}", self.kernel_version),
            format!("Machine: {}", self.machine),
            format!("Processor: {}", self.processor),
            format!("Hardware Platform: {}", self.hardware_platform),
            format!("Operating System: {}", self.operating_system),
        ];

        write!(f, "{}", lines.join("\n"))
    }
}

impl Display for UnameField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(val) => {
                write!(f, "{}", val)
            }
            None => {
                write!(f, "None")
            }
        }
    }
}
