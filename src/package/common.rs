use crate::{
    tags::{
        Author, Dependency, Description, Export, GetTag, License, Maintainer, PackageName, Tag,
        Url, Version,
    },
    PackageError,
};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Default, Debug, Clone)]
pub struct PackageCommon {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub maintainer: Vec<Maintainer>,
    pub license: Vec<License>,
    pub url: Vec<Url>,
    pub author: Vec<Author>,
    pub build_depend: Vec<Dependency>,
    pub buildtool_depend: Vec<Dependency>,
    pub test_depend: Vec<Dependency>,
    pub conflict: Vec<Dependency>,
    pub replace: Vec<Dependency>,
    pub export: Option<Export>,
}

pub trait PackageCommonMethods {
    fn common(&self) -> &PackageCommon;

    fn name(&self) -> &String {
        &self.common().name
    }

    fn version(&self) -> &Version {
        &self.common().version
    }

    fn description(&self) -> &String {
        &self.common().description
    }

    fn maintainer(&self) -> &Vec<Maintainer> {
        &self.common().maintainer
    }

    fn license(&self) -> &Vec<License> {
        &self.common().license
    }

    fn url(&self) -> &Vec<Url> {
        &self.common().url
    }

    fn author(&self) -> &Vec<Author> {
        &self.common().author
    }

    fn build_depend(&self) -> &Vec<Dependency> {
        &self.common().build_depend
    }

    fn buildtool_depend(&self) -> &Vec<Dependency> {
        &self.common().buildtool_depend
    }

    fn test_depend(&self) -> &Vec<Dependency> {
        &self.common().test_depend
    }

    fn conflict(&self) -> &Vec<Dependency> {
        &self.common().conflict
    }

    fn replace(&self) -> &Vec<Dependency> {
        &self.common().replace
    }

    fn export(&self) -> &Option<Export> {
        &self.common().export
    }
}

impl TryFrom<Node<'_, '_>> for PackageCommon {
    type Error = PackageError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        use PackageError::*;
        let name: Tag<PackageName> = node.get_tag("name").ok_or(NoName)??;
        let version: Version = node
            .get_tag("version")
            .ok_or(NoVersion)?
            .map_err(InvalidVersion)?;
        let description: Description = node.get_tag("description").ok_or(NoDescription)??;
        let maintainer: Vec<Maintainer> = node.get_tags("maintainer").map_err(InvalidMaintainer)?;
        let license: Vec<License> = node.get_tags("license").map_err(InvalidLicense)?;
        let url: Vec<Url> = node.get_tags("url").map_err(InvalidUrl)?;
        let author: Vec<Author> = node.get_tags("author").map_err(InvalidAuthor)?;
        let build_depend: Vec<Dependency> =
            node.get_tags("build_depend").map_err(InvalidDependency)?;
        let buildtool_depend: Vec<Dependency> = node
            .get_tags("buildtool_depend")
            .map_err(InvalidDependency)?;
        let test_depend: Vec<Dependency> =
            node.get_tags("test_depend").map_err(InvalidDependency)?;
        let conflict: Vec<Dependency> = node.get_tags("conflict").map_err(InvalidDependency)?;
        let replace: Vec<Dependency> = node.get_tags("replace").map_err(InvalidDependency)?;
        let export: Option<Export> = node.get_tag("export").transpose()?;

        if maintainer.is_empty() {
            return Err(NoMaintainer);
        }

        if license.is_empty() {
            return Err(NoLicense);
        }

        Ok(Self {
            name: name.to_string(),
            version,
            description: description.to_string(),
            maintainer,
            license,
            url,
            author,
            build_depend,
            buildtool_depend,
            test_depend,
            conflict,
            replace,
            export,
        })
    }
}
