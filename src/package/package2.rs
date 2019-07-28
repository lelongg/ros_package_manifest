use crate::{
    package::common::PackageCommonMethods,
    package::PackageCommon,
    tags::{Dependency, GetTag},
    PackageError,
};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Package2 {
    pub common: PackageCommon,
    pub build_export_depend: Vec<Dependency>,
    pub buildtool_export_depend: Vec<Dependency>,
    pub exec_depend: Vec<Dependency>,
    pub depend: Vec<Dependency>,
}

impl PackageCommonMethods for Package2 {
    fn common(&self) -> &PackageCommon {
        &self.common
    }
}

impl Package2 {
    pub fn build_export_depend(&self) -> &Vec<Dependency> {
        &self.build_export_depend
    }

    pub fn buildtool_export_depend(&self) -> &Vec<Dependency> {
        &self.buildtool_export_depend
    }

    pub fn exec_depend(&self) -> &Vec<Dependency> {
        &self.exec_depend
    }

    pub fn depend(&self) -> &Vec<Dependency> {
        &self.depend
    }
}

impl FromStr for Package2 {
    type Err = PackageError;
    fn from_str(package_xml: &str) -> Result<Self, Self::Err> {
        use PackageError::*;
        let xml_tree = roxmltree::Document::parse(&package_xml).map_err(ParsingError)?;
        let root = xml_tree.root();
        let build_export_depend: Vec<Dependency> = root
            .get_tags("build_export_depend")
            .map_err(InvalidDependency)?;
        let buildtool_export_depend: Vec<Dependency> = root
            .get_tags("buildtool_export_depend")
            .map_err(InvalidDependency)?;
        let exec_depend: Vec<Dependency> =
            root.get_tags("exec_depend").map_err(InvalidDependency)?;
        let depend: Vec<Dependency> = root.get_tags("depend").map_err(InvalidDependency)?;

        Ok(Self {
            common: PackageCommon::try_from(root)?,
            build_export_depend,
            buildtool_export_depend,
            exec_depend,
            depend,
        })
    }
}
