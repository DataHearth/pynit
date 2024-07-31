use anyhow::Result;
use dialoguer::Select;
use dialoguer::{theme::ColorfulTheme, Input};
use std::fmt::Debug;
use std::str::FromStr;

pub fn input<T>(theme: &ColorfulTheme, prompt: &str, empty: bool, default: Option<T>) -> Result<T>
where
    T: Clone + FromStr + ToString,
    <T as FromStr>::Err: Debug + ToString,
{
    let mut input = Input::<T>::with_theme(theme);

    input.with_prompt(prompt).allow_empty(empty);
    if let Some(default) = default {
        input.default(default);
    }

    Ok(input.interact_text()?)
}

pub fn input_list<F, T>(
    theme: &ColorfulTheme,
    prompt: &str,
    empty: bool,
    default: Option<String>,
    map_fn: F,
) -> Result<Vec<T>>
where
    F: FnMut(&str) -> T,
{
    Ok(input(theme, prompt, empty, default)?
        .split(';')
        .filter(|v| !v.is_empty())
        .map(map_fn)
        .collect())
}

pub fn select(
    theme: &ColorfulTheme,
    prompt: &str,
    default: usize,
    items: &[String],
) -> Result<usize> {
    Ok(Select::with_theme(theme)
        .with_prompt(prompt)
        .default(default)
        .items(items)
        .interact()?)
}
