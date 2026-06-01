use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, ErrorKind},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    errors::{SFFError, SFFResult},
    flags::FFlagType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoberConfig {
    pub fflags: HashMap<String, FFlagType>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl SoberConfig {
    pub fn load(path: &Path) -> SFFResult<Self> {
        let raw = fs::read_to_string(path).map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                SFFError::ConfigNotFound(e)
            } else {
                SFFError::IOError(e)
            }
        })?;

        let json = Self::clean_json(&raw);

        let config = serde_json::from_str(&json)?;

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> SFFResult<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(BufWriter::new(file), self)?;

        Ok(())
    }

    fn clean_json(raw: &str) -> String {
        raw.lines()
            .map(|l| l.trim())
            .filter(|l| !l.starts_with("//") && !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_cleaning() {
        let dirty_json = r#"
            // comment
            // meow
            // wow
            {
            "fflags": {
            "FFlagExample": true
            }
            }
            "#;

        let cleaned_json = SoberConfig::clean_json(dirty_json);

        assert!(!cleaned_json.contains("//"));
        assert!(cleaned_json.contains(r#""FFlagExample": true"#))
    }

    #[test]
    fn config_load_success() {
        let config_path = std::env::temp_dir().join("sff_load_success_test.json");

        let raw = r#"
                    // comment
                    {
                        "fflags": {
                            "FFlagDebugSkyGray": true,
                            "DFIntTextureQualityOverride": 3
                        },
                        "other_setting": "hi_sober"
                    }
                "#;
        fs::write(&config_path, raw).unwrap();

        let config = SoberConfig::load(&config_path).unwrap();

        assert_eq!(
            config.fflags.get("FFlagDebugSkyGray"),
            Some(&FFlagType::Boolean(true))
        );
        assert_eq!(
            config.fflags.get("DFIntTextureQualityOverride"),
            Some(&FFlagType::Integer(3))
        );
        assert_eq!(config.other.get("other_setting").unwrap(), "hi_sober");

        fs::remove_file(&config_path).unwrap();
    }

    #[test]
    fn config_load_error() {
        let config_path = std::env::temp_dir().join("not_a_real_file.json");
        let result = SoberConfig::load(&config_path);

        assert!(result.is_err())
    }

    #[test]
    fn config_save_integrity() {
        let config_path = std::env::temp_dir().join("sff_save_test.json");

        let mut fflags = std::collections::HashMap::new();
        fflags.insert("FFlagExample".to_string(), FFlagType::Boolean(false));

        let mut other = std::collections::HashMap::new();
        other.insert(
            "nested_roblox_config".to_string(),
            serde_json::json!({ "enable_something": true }),
        );

        let config = SoberConfig { fflags, other };

        config.save(&config_path).unwrap();

        let reloaded = SoberConfig::load(&config_path).unwrap();

        assert_eq!(
            reloaded.fflags.get("FFlagExample"),
            Some(&FFlagType::Boolean(false))
        );
        assert_eq!(
            reloaded.other.get("nested_roblox_config").unwrap(),
            &serde_json::json!({ "enable_something": true })
        );

        fs::remove_file(config_path).unwrap();
    }
}
