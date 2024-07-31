use std::{collections::HashMap, env};

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use regex::Regex;
use serde::Serialize;
use toml::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub struct Pyproject {
    pub build_system: BuildSystem,
    pub project: Project,
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

    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

pub fn ask_user_inputs(minimum: bool) -> Result<Pyproject> {
    let mut project = Project::default();
    let mut build_system = BuildSystem::default();
    let theme = ColorfulTheme::default();

    build_system.requires = Input::<String>::with_theme(&theme)
        .with_prompt("build dependencies (comma separated)")
        .default("setuptools,wheel".to_string())
        .interact_text()?
        .split(',')
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect();
    build_system.build_backend = Input::with_theme(&theme)
        .with_prompt("build back-end")
        .default("setuptools.build_meta".to_string())
        .interact_text()?;

    project.name = Input::with_theme(&theme)
        .with_prompt("project name")
        .default(
            env::current_dir()?
                .file_name()
                .expect("invalid current directory")
                .to_str()
                .expect("invalid UTF-8 cwd name")
                .to_string(),
        )
        .interact_text()?;
    project.version = Input::with_theme(&theme)
        .with_prompt("version")
        .default("0.1.0".to_string())
        .interact_text()?;

    if !minimum {
        project.description = Input::with_theme(&theme)
            .with_prompt("description")
            .allow_empty(true)
            .interact_text()?;
        project.readme = Input::with_theme(&theme)
            .with_prompt("readme")
            .allow_empty(true)
            .interact_text()?;
        project.requires_python = Input::with_theme(&theme)
            .with_prompt("minimum python version")
            .allow_empty(true)
            .interact_text()?;
        project.license = Input::with_theme(&theme)
            .with_prompt("license")
            .allow_empty(true)
            .interact_text()?;
        project.authors = Input::<String>::with_theme(&theme)
            .with_prompt(
                r#"authors (e.g: "Antoine Langlois";"name="Antoine L",email="email@domain.net"")"#,
            )
            .allow_empty(true)
            .interact_text()?
            .split(';')
            .filter(|v| !v.is_empty())
            .map(|v| parse_contributor(v))
            .collect();
        project.maintainers = Input::<String>::with_theme(&theme)
            .with_prompt(
                r#"maintainers (e.g: "Antoine Langlois";"name="Antoine L",email="email@domain.net"")"#,
            )
            .allow_empty(true)
            .interact_text()?
            .split(';')
            .filter(|v| !v.is_empty())
            .map(|v| parse_contributor(v))
            .collect();
        project.keywords = Input::<String>::with_theme(&theme)
            .with_prompt("keywords (e.g: KEYW1;KEYW2)")
            .allow_empty(true)
            .interact_text()?
            .split(';')
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect();
        project.classifiers = Input::<String>::with_theme(&theme)
            .with_prompt("classifiers (e.g: CLASS1;CLASS2)")
            .allow_empty(true)
            .interact_text()?
            .split(';')
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect();
    }

    Ok(Pyproject {
        build_system,
        project,
    })
}

fn parse_contributor(contributor: &str) -> Contributor {
    let name_regex = Regex::new(r#"name="([\w\s\-\.]*)""#).expect("invalid name regex expression");
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
