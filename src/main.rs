use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError};
use validator_derive::Validate;

lazy_static! {
    static ref PACKAGE_NAME_REGEX: Regex = Regex::new(r"^[[:lower:]][[:word:]]*$").unwrap();
    static ref VERSION_REGEX: Regex =
        Regex::new(r"^[[:digit:]]+.[[:digit:]]+.[[:digit:]]+$").unwrap();
}

#[derive(Debug, Clone, Deserialize, Validate)]
struct PackageFormat {
    #[serde(default = "PackageFormat::default_format")]
    format: u32,
}

impl PackageFormat {
    fn default_format() -> u32 {
        1
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "format", content = "$value")]
enum Package {
    #[serde(rename = "1")]
    Package1(Package1),
    #[serde(rename = "2")]
    Package2(Package2),
}

#[derive(Debug, Clone, Deserialize, Validate)]
struct Package1 {
    #[serde(default = "PackageFormat::default_format")]
    format: u32,
    #[validate(regex = "PACKAGE_NAME_REGEX")]
    name: String,
    #[validate(regex = "VERSION_REGEX")]
    version: String,
    description: String, // FIXME
    #[validate(length(min = "1"))]
    maintainer: Vec<Maintainer>,
    #[validate(length(min = "1"))]
    license: Vec<String>,
    #[serde(default)]
    url: Vec<Url>,
    #[serde(default)]
    author: Vec<Author>,
    #[serde(default)]
    build_depend: Vec<Dependency>,
    #[serde(default)]
    buildtool_depend: Vec<Dependency>,
    #[serde(default)]
    run_depend: Vec<Dependency>,
    #[serde(default)]
    test_depend: Vec<Dependency>,
    #[serde(default)]
    conflict: Vec<Dependency>,
    #[serde(default)]
    replace: Vec<Dependency>,
    export: Option<Export>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
struct Package2 {
    #[serde(default = "PackageFormat::default_format")]
    format: u32,
    #[validate(regex = "PACKAGE_NAME_REGEX")]
    name: String,
    #[validate(regex = "VERSION_REGEX")]
    version: String,
    description: String, // FIXME
    #[validate(length(min = "1"))]
    maintainer: Vec<Maintainer>,
    #[validate(length(min = "1"))]
    license: Vec<String>,
    #[serde(default)]
    url: Vec<Url>,
    #[serde(default)]
    author: Vec<Author>,
    #[serde(default)]
    build_depend: Vec<Dependency>,
    #[serde(default)]
    build_export_depend: Vec<Dependency>,
    #[serde(default)]
    buildtool_depend: Vec<Dependency>,
    #[serde(default)]
    buildtool_export_depend: Vec<Dependency>,
    #[serde(default)]
    exec_depend: Vec<Dependency>,
    #[serde(default)]
    depend: Vec<Dependency>,
    #[serde(default)]
    doc_depend: Vec<Dependency>,
    #[serde(default)]
    test_depend: Vec<Dependency>,
    #[serde(default)]
    conflict: Vec<Dependency>,
    #[serde(default)]
    replace: Vec<Dependency>,
    export: Option<Export>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Maintainer {
    #[serde(rename = "$value")]
    name: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Url {
    #[serde(rename = "$value")]
    url: String,
    #[serde(default)]
    r#type: UrlType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum UrlType {
    Website,
    BugTracker,
    Repository,
}

impl Default for UrlType {
    fn default() -> Self {
        UrlType::Website
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Author {
    #[serde(rename = "$value")]
    name: String,
    email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Dependency {
    #[serde(rename = "$value")]
    name: String,
    version_lt: Option<String>,
    version_lte: Option<String>,
    version_eq: Option<String>,
    version_gte: Option<String>,
    version_gt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Export {
    #[serde(default)]
    architecture_independent: Option<()>,
    #[serde(default = "Export::default_build_type")]
    build_type: String,
    #[serde(default)]
    deprecated: Option<String>,
    #[serde(default)]
    message_generator: Option<String>,
    #[serde(default)]
    metapackage: Option<()>,
}

impl Export {
    fn default_build_type() -> String {
        "catkin".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Deprecated {
    text: Option<String>,
}

fn main() {
    let package_xml =
        std::fs::read_to_string("data/package.xml").expect("cannot read \"data/package.xml\"");
    println!("{:#?}", roxmltree::Document::parse(&package_xml));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_reader;

    #[test]
    fn full_test() {
        let package_xml =
            std::fs::read_to_string("data/package.xml").expect("cannot read \"data/package.xml\"");
        let package: Package = from_reader(package_xml.as_bytes()).unwrap();
        println!("{:#?}", package);
    }

    #[test]
    fn validate_package_name() {
        assert!(PACKAGE_NAME_REGEX.is_match("roscpp"));
        assert!(PACKAGE_NAME_REGEX.is_match("roscpp1"));
        assert!(PACKAGE_NAME_REGEX.is_match("roscpp1A"));
        assert!(PACKAGE_NAME_REGEX.is_match("ros_cpp"));
        assert!(!PACKAGE_NAME_REGEX.is_match("1roscpp"));
        assert!(!PACKAGE_NAME_REGEX.is_match("Aroscpp"));
        assert!(!PACKAGE_NAME_REGEX.is_match("_roscpp"));
    }

    #[test]
    fn validate_version() {
        assert!(VERSION_REGEX.is_match("1.2.3"));
        assert!(VERSION_REGEX.is_match("11.22.33"));
        assert!(!VERSION_REGEX.is_match("12.3"));
        assert!(!VERSION_REGEX.is_match("a.2.3"));
    }
}