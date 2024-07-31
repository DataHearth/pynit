use anyhow::Result;
use reqwest::{blocking::Client, header::USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LicenseDetails {
    #[serde(flatten)]
    base_license: License,

    pub body: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub spdx_id: String,
}

pub fn get_license_spdx() -> Result<Vec<String>> {
    let licenses = Client::new()
        .get(&format!("https://api.github.com/licenses",))
        .header(USER_AGENT, format!("pynit-{}", env!("CARGO_PKG_VERSION")))
        .send()?
        .json::<Vec<License>>()?;

    Ok(licenses
        .iter()
        .map(|license| license.spdx_id.clone())
        .collect())
}

pub fn get_license(spdx: String) -> Result<LicenseDetails> {
    Ok(Client::new()
        .get(&format!("https://api.github.com/licenses/{spdx}"))
        .header(USER_AGENT, format!("pynit-{}", env!("CARGO_PKG_VERSION")))
        .send()?
        .json::<LicenseDetails>()?)
}
