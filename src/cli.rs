use std::path::Path;

use clap::{Parser, Subcommand};

use crate::{
    config::SoberConfig,
    errors::{SFFError, SFFResult},
    flags::{FFlag, FFlagType},
};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add or update a FFlag value in the configuration
    #[command(alias = "set")]
    Add {
        /// The name of the FFlag (e.g., FFlagDebugGraphicsPreferD3D11)
        name: FFlag,
        /// The value to assign to the FFlag
        value: String,
    },

    /// Remove a FFlag from the configuration
    #[command(aliases = ["rm", "delete"])]
    Remove {
        /// The name of the FFlag to remove
        name: FFlag,
    },

    /// List all currently configured FFlags
    List,
}

impl Commands {
    pub fn execute(self, path: &Path) -> SFFResult<()> {
        match self {
            Commands::Add { name, value } => Self::add_flag(path, name, value)?,
            Commands::Remove { name } => Self::remove_flag(path, name)?,
            Commands::List => Self::list_flags(path)?,
        }
        Ok(())
    }

    fn add_flag(path: &Path, name: FFlag, value: String) -> SFFResult<()> {
        let mut config = SoberConfig::load(path)?;
        let name_str = name.to_string();

        config.fflags.insert(name_str, FFlagType::parse(&value));
        config.save(path)?;

        println!("[+] {} set to {}", &name, FFlagType::parse(&value));
        Ok(())
    }

    fn remove_flag(path: &Path, name: FFlag) -> SFFResult<()> {
        let mut config = SoberConfig::load(path)?;
        let name_str = name.to_string();

        config
            .fflags
            .remove(&name_str)
            .ok_or(SFFError::FlagNotFound(name_str))?;
        config.save(path)?;

        println!("[-] {} removed", name);
        Ok(())
    }

    fn list_flags(path: &Path) -> SFFResult<()> {
        let config = SoberConfig::load(path)?;

        println!("[*] Currently configured FFlags:");
        for fflag in config.fflags {
            println!("    {} = {}", fflag.0, fflag.1);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn create_test_config(file_name: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join(file_name);

        let initial_json = r#"{"fflags": {}, "other_settings": {}}"#;
        fs::write(&config_path, initial_json).expect("Failed to create test config file");

        config_path
    }

    #[test]
    fn command_add() {
        let config_path = create_test_config("sff_add_test.json");

        let add_command = Commands::Add {
            name: FFlag::FFlagDebugSkyGray,
            value: "true".to_string(),
        };
        add_command.execute(&config_path).unwrap();

        let config = SoberConfig::load(&config_path).unwrap();

        assert_eq!(
            config.fflags.get("FFlagDebugSkyGray"),
            Some(&FFlagType::Boolean(true))
        );

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn command_remove() {
        let config_path = create_test_config("sff_remove_test.json");

        let remove_command = Commands::Remove {
            name: FFlag::FFlagDebugSkyGray,
        };
        let result = remove_command.execute(&config_path);

        assert!(result.is_err());

        let add_command = Commands::Add {
            name: FFlag::FFlagDebugSkyGray,
            value: "true".to_string(),
        };
        add_command.execute(&config_path).unwrap();

        let remove_command = Commands::Remove {
            name: FFlag::FFlagDebugSkyGray,
        };
        let result = remove_command.execute(&config_path);

        assert!(result.is_ok());

        fs::remove_file(config_path).unwrap()
    }

    #[test]
    fn command_list() {
        let config_path = create_test_config("sff_list_test.json");

        let add_command = Commands::Add {
            name: FFlag::FFlagDebugSkyGray,
            value: "true".to_string(),
        };
        add_command.execute(&config_path).unwrap();

        let list_command = Commands::List;
        let result = list_command.execute(&config_path);

        assert!(result.is_ok())
    }
}
