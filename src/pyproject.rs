use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use dialoguer::theme::ColorfulTheme;
use regex::Regex;
use serde::Serialize;

use crate::{
    components::{input, input_list, select},
    license::get_license_spdx,
};

const PYPROJECT: &str = "pyproject.toml";

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub struct Pyproject {
    #[serde(skip_serializing)]
    folder: PathBuf,
    #[serde(skip_serializing)]
    complete: bool,
    pub build_system: BuildSystem,
    pub project: Project,
}

impl Pyproject {
    pub fn new(folder: PathBuf, complete: bool) -> Self {
        Pyproject {
            folder,
            complete,
            build_system: BuildSystem::default(),
            project: Project::default(),
        }
    }

    /// Ask user inputs to create a basic (or not) pyproject.toml file.
    /// `minimum` will only ask for the `build-system` fields and the `project.name`
    /// and `project.version` fields
    pub fn ask_inputs(&mut self) -> Result<()> {
        let theme = ColorfulTheme::default();

        self.build_system.requires = input_list(
            &theme,
            "build dependencies (comma separated)",
            false,
            Some("setuptools,wheel".to_string()),
            |v| v.to_string(),
        )?;
        self.build_system.build_backend = input::<String>(
            &theme,
            "build back-end",
            false,
            Some("setuptools.build_meta".to_string()),
        )?;

        // ? might want to switch to OsString instead, if the Serialize macro supports it
        self.project.name = input::<String>(
            &theme,
            "project name",
            false,
            match self
                .folder
                .file_name()
                .ok_or(anyhow!("project can't terminate by \"..\""))?
                .to_str()
            {
                Some(v) => Some(v.to_string()),
                None => None,
            },
        )?;
        self.project.version =
            input::<String>(&theme, "version", false, Some("0.1.0".to_string()))?;

        if self.complete {
            self.ask_complete(&theme)?;
        }

        Ok(())
    }

    fn ask_complete(&mut self, theme: &ColorfulTheme) -> Result<()> {
        self.project.description = input::<String>(theme, "description", true, None)?;
        self.project.readme = input::<String>(theme, "readme", true, None)?;
        self.project.requires_python =
            input::<String>(theme, "minimum python version", true, None)?;
        let license_spdx = get_license_spdx()?;
        let license_index = select(
            theme,
            "license",
            license_spdx
                .binary_search(&"MIT".into())
                .or(Err(anyhow!("MIT license not found")))?,
            &license_spdx[..],
        )?;

        self.project.license = license_spdx[license_index].clone();
        self.project.authors = input_list(
            theme,
            r#"authors (e.g: "Antoine Langlois";"name="Antoine L",email="email@domain.net"")"#,
            true,
            None,
            |v| self.parse_contributor(v),
        )?;
        self.project.maintainers = input_list(
            theme,
            r#"maintainers (e.g: "Antoine Langlois";"name="Antoine L",email="email@domain.net"")"#,
            true,
            None,
            |v| self.parse_contributor(v),
        )?;
        self.project.keywords =
            input_list(theme, "keywords (e.g: KEYW1;KEYW2)", true, None, |v| {
                v.to_string()
            })?;
        self.project.classifiers =
            input_list(theme, "classifiers (e.g: CLASS1;CLASS2)", true, None, |v| {
                v.to_string()
            })?;

        Ok(())
    }

    fn parse_contributor(&self, contributor: &str) -> Contributor {
        let name_regex =
            Regex::new(r#"name="([\w\s\-\.]*)""#).expect("invalid name regex expression");
        let email_regex =
            Regex::new(r#"email="([\w\s\-\.@]*)""#).expect("invalid email regex expression");

        let name = name_regex.captures(contributor);
        let email = email_regex.captures(contributor);

        if name.is_some() && email.is_some() {
            Contributor::Complex {
                name: name.unwrap().get(1).unwrap().as_str().to_string(),
                email: email.unwrap().get(1).unwrap().as_str().to_string(),
            }
        } else if let Some(name_match) = name {
            Contributor::Flat(name_match.get(1).unwrap().as_str().to_string())
        } else if let Some(email_match) = email {
            Contributor::Flat(email_match.get(1).unwrap().as_str().to_string())
        } else {
            Contributor::Flat(contributor.to_string())
        }
    }

    pub fn get_project_name(&self) -> String {
        // ? clone or maybe something else ?
        self.project.name.clone()
    }

    pub fn get_license_spdx(&self) -> String {
        self.project.license.clone()
    }

    /// Consume self and write everything to a `self.folder/pyproject.toml`
    pub fn create_file(self) -> Result<()> {
        fs::write(self.folder.join(PYPROJECT), toml::to_vec(&self)?)?;

        Ok(())
    }
}

#[derive(Default, Debug, Serialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub struct BuildSystem {
    pub requires: Vec<String>,
    pub build_backend: String,
}

#[derive(Debug, Serialize)]
pub enum Contributor {
    Flat(String),
    Complex { name: String, email: String },
}

#[derive(Default, Debug, Serialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub struct Project {
    pub name: String,
    pub version: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub readme: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub requires_python: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub license: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Contributor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<Contributor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classifiers: Vec<String>,
}
