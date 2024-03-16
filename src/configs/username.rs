use serde::{Deserialize, Serialize};
use toml::{map::Map, Value};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct UsernameConfig<'a> {
    pub detect_env_vars: Vec<&'a str>,
    pub format: &'a str,
    pub style_root: &'a str,
    pub style_user: &'a str,
    pub show_always: bool,
    pub disabled: bool,
    // pub aliases: Option<Vec<(&'a str, &'a str)>>,
    pub aliases: Map<String, Value>,
}

impl<'a> Default for UsernameConfig<'a> {
    fn default() -> Self {
        UsernameConfig {
            detect_env_vars: vec![],
            format: "[$user]($style) in ",
            style_root: "red bold",
            style_user: "yellow bold",
            show_always: false,
            disabled: false,
            aliases: Map::new(),
        }
    }
}
