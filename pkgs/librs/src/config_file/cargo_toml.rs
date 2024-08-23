use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoToml {
    /// Defines a package
    package: Package,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    /// The name of the package
    name: String,
    /// The version of the package
    version: Option<String>,
    /// The authors of the package
    authors: Option<Vec<String>>,
    /// The rust edition
    edition: Option<String>,
    /// The minimal supported Rust version
    rust_version: Option<String>,
    /// A description of the package
    description: Option<String>,
    /// URL of the package documentation
    documentation: Option<String>,
    /// Path to the package's README file
    readme: Option<String>,
    /// URL of the package homepage
    homepage: Option<String>,
    /// URL of the package source repository
    repository: Option<String>,
    /// The package license
    license: Option<String>,
    /// Path to the text of the license
    license_file: Option<String>,
    /// Keywords for the package
    keywords: Option<Vec<String>>,
    /// Categories of the package
    categories: Option<Vec<String>>,
    /// Path to the workspace of the package
    workspace: Option<String>,
    /// Path to the package build script
    build: Option<String>,
    /// Name of the native library the package links with
    links: Option<String>,
    /// Files to exculde when publishing
    exclude: Option<Vec<String>>,
    /// Files to include when publishing
    include: Option<Vec<String>>,
    /// Can be used to prevent publishing the package
    publish: Option<bool>,
    /// The default binary to run by `cargo run`
    default_run: Option<String>,
    /// Disables binary auto discovery
    autobins: Option<bool>,
    /// Disables example auto discovery
    autoexamples: Option<bool>,
    /// Disables test auto discovery
    autotests: Option<bool>,
    /// Disables bench auto discovery
    autobenches: Option<bool>,
    /// Sets the dependency resolver to use
    resolver: Option<String>,
}

impl CargoToml {
    pub fn from_dir(path: &PathBuf) -> Result<CargoToml, String> {
        let file_contents = read_to_string(path);
        if let Err(e) = file_contents {
            return Err(format!("{}", e));
        }
        let file_contents = file_contents.unwrap();
        let cargo_toml = toml::from_str::<CargoToml>(&file_contents);

        if let Err(e) = cargo_toml {
            return Err(format!("{}", e));
        }

        Ok(cargo_toml.unwrap())
    }
}

impl Display for CargoToml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[package]\n{}", self.package)
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![format!("name = \"{}\"", self.name)];
        if let Some(version) = &self.version {
            lines.push(format_string("version", version));
        }
        if let Some(authors) = &self.authors {
            lines.push(format_vec_string("authors", authors));
        }
        if let Some(edition) = &self.edition {
            lines.push(format_string("edition", edition));
        }
        if let Some(rust_version) = &self.rust_version {
            lines.push(format_string("rust-version", rust_version));
        }
        if let Some(edition) = &self.edition {
            lines.push(format_string("edition", edition));
        }
        if let Some(description) = &self.description {
            lines.push(format_string("description", description));
        }
        if let Some(documentation) = &self.documentation {
            lines.push(format_string("documentation", documentation));
        }
        if let Some(readme) = &self.readme {
            lines.push(format_string("readme", readme));
        }
        if let Some(homepage) = &self.homepage {
            lines.push(format_string("homepage", homepage));
        }
        if let Some(repository) = &self.repository {
            lines.push(format_string("repository", repository));
        }
        if let Some(license) = &self.license {
            lines.push(format_string("license", license));
        }
        if let Some(license_file) = &self.license_file {
            lines.push(format_string("license-file", license_file));
        }
        if let Some(keywords) = &self.keywords {
            lines.push(format_vec_string("keywords", keywords));
        }
        if let Some(categories) = &self.categories {
            lines.push(format_vec_string("categories", categories));
        }
        if let Some(workspace) = &self.workspace {
            lines.push(format_string("workspace", workspace));
        }
        if let Some(build) = &self.build {
            lines.push(format_string("build", build));
        }
        if let Some(links) = &self.links {
            lines.push(format_string("links", links));
        }
        if let Some(exclude) = &self.exclude {
            lines.push(format_vec_string("exclude", exclude));
        }
        if let Some(include) = &self.include {
            lines.push(format_vec_string("include", include));
        }
        if let Some(publish) = &self.publish {
            lines.push(format_bool("publish", publish));
        }
        if let Some(default_run) = &self.default_run {
            lines.push(format_string("default-run", default_run));
        }
        if let Some(autobins) = &self.autobins {
            lines.push(format_bool("autobins", autobins));
        }
        if let Some(autoexamples) = &self.autoexamples {
            lines.push(format_bool("autoexamples", autoexamples));
        }
        if let Some(autotests) = &self.autotests {
            lines.push(format_bool("autotests", autotests));
        }
        if let Some(autobenches) = &self.autobenches {
            lines.push(format_bool("autobenches", autobenches));
        }
        if let Some(resolver) = &self.resolver {
            lines.push(format_string("resolver", resolver));
        }
        write!(f, "{}", lines.join("\n"))
    }
}

fn format_bool(name: &str, value: &bool) -> String {
    if *value {
        format!("{} = true", name)
    } else {
        format!("{} = false", name)
    }
}

fn format_string(name: &str, value: &String) -> String {
    format!("{} = \"{}\"", name, value)
}

/// Apply toml formatting to a Vec<String>
fn format_vec_string(name: &str, input: &Vec<String>) -> String {
    if input.len() == 0 {
        return format!("{} = []", name);
    }

    if input.len() < 3 {
        return format!("{}", name);
    }

    let lines: Vec<String> = input
        .into_iter()
        .map(|line| format!("    \"{}\"", line))
        .collect();

    format!("{} = [\n{}\n]", name, lines.join("\n"))
}
