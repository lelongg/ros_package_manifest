use crate::{
    package::common::{PackageCommon, PackageCommonMethods},
    tags::{Dependency, GetTag},
    PackageError,

};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Package1 {
    pub common: PackageCommon,
    pub run_depend: Vec<Dependency>,
}

impl PackageCommonMethods for Package1 {
    fn common(&self) -> &PackageCommon {
        &self.common
    }
}

impl Package1 {
    pub fn run_depends(&self) -> &Vec<Dependency> {
        &self.run_depend
    }
}

impl FromStr for Package1 {
    type Err = PackageError;
    fn from_str(package_xml: &str) -> Result<Self, Self::Err> {
        use PackageError::*;
        let xml_tree = roxmltree::Document::parse(&package_xml).map_err(ParsingError)?;
        let root = xml_tree.root();
        let run_depend: Vec<Dependency> = root.get_tags("run_depend").map_err(InvalidDependency)?;

        Ok(Self {
            common: PackageCommon::try_from(root)?,
            run_depend,
        })
    }
}
