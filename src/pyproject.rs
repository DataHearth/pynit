use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Input};
use regex::Regex;
use serde::Serialize;

const PYPROJECT: &str = "pyproject.toml";

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub struct Pyproject {
    #[serde(skip_serializing)]
    folder: PathBuf,
    #[serde(skip_serializing)]
    minimum: bool,
    pub build_system: BuildSystem,
    pub project: Project,
}

impl Pyproject {
    pub fn new(folder: PathBuf, minimum: bool) -> Self {
        Pyproject {
            folder,
            minimum,
            build_system: BuildSystem::default(),
            project: Project::default(),
        }
    }

    /// Ask user inputs to create a basic (or not) pyproject.toml file.
    /// `minimum` will only ask for the `build-system` fields and the `project.name`
    /// and `project.version` fields
    pub fn ask_inputs(&mut self) -> Result<()> {
        let theme = ColorfulTheme::default();

        self.build_system.requires = Input::<String>::with_theme(&theme)
            .with_prompt("build dependencies (comma separated)")
            .default("setuptools,wheel".to_string())
            .interact_text()?
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect();
        self.build_system.build_backend = Input::with_theme(&theme)
            .with_prompt("build back-end")
            .default("setuptools.build_meta".to_string())
            .interact_text()?;

        // ? might want to switch to OsString instead, if the Serialize macro supports it
        let folder = match self
            .folder
            .file_name()
            .ok_or(anyhow!("project can't terminate by \"..\""))?
            .to_str()
        {
            Some(v) => Some(v.to_string()),
            None => None,
        };
        let mut input: Input<String> = Input::with_theme(&theme);
        if let Some(folder) = folder {
            self.project.name = input
                .with_prompt("project name")
                .default(folder)
                .interact_text()?;
        } else {
            self.project.name = input.with_prompt("project name").interact_text()?;
        }

        self.project.version = Input::with_theme(&theme)
            .with_prompt("version")
            .default("0.1.0".to_string())
            .interact_text()?;

        if !self.minimum {
            self.project.description = Input::with_theme(&theme)
                .with_prompt("description")
                .allow_empty(true)
                .interact_text()?;
            self.project.readme = Input::with_theme(&theme)
                .with_prompt("readme")
                .allow_empty(true)
                .interact_text()?;
            self.project.requires_python = Input::with_theme(&theme)
                .with_prompt("minimum python version")
                .allow_empty(true)
                .interact_text()?;
            self.project.license = Input::with_theme(&theme)
                .with_prompt("license")
                .allow_empty(true)
                .interact_text()?;
            self.project.authors = Input::<String>::with_theme(&theme)
            .with_prompt(
                r#"authors (e.g: "Antoine Langlois";"name="Antoine L",email="email@domain.net"")"#,
            )
            .allow_empty(true)
            .interact_text()?
            .split(';')
            .filter(|v| !v.is_empty())
            .map(|v| self.parse_contributor(v))
            .collect();
            self.project.maintainers = Input::<String>::with_theme(&theme)
            .with_prompt(
                r#"maintainers (e.g: "Antoine Langlois";"name="Antoine L",email="email@domain.net"")"#,
            )
            .allow_empty(true)
            .interact_text()?
            .split(';')
            .filter(|v| !v.is_empty())
            .map(|v| self.parse_contributor(v))
            .collect();
            self.project.keywords = Input::<String>::with_theme(&theme)
                .with_prompt("keywords (e.g: KEYW1;KEYW2)")
                .allow_empty(true)
                .interact_text()?
                .split(';')
                .filter(|v| !v.is_empty())
                .map(|v| v.to_string())
                .collect();
            self.project.classifiers = Input::<String>::with_theme(&theme)
                .with_prompt("classifiers (e.g: CLASS1;CLASS2)")
                .allow_empty(true)
                .interact_text()?
                .split(';')
                .filter(|v| !v.is_empty())
                .map(|v| v.to_string())
                .collect();
        }

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
