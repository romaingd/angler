use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Represents a hook in the pre-commit configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    /// The ID of the hook
    pub id: String,
    /// The name of the hook (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The entry point to execute (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// The language to use when running the hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Additional arguments for the hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Files to include for the hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    /// Files to exclude for the hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,
    /// Types of files to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// Types of files to exclude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_types: Option<Vec<String>>,
    /// Whether to pass staged files to the hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_filenames: Option<bool>,
    /// Whether the hook should always run regardless of file filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    /// Whether the hook can be skipped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
    /// Additional configuration options that aren't explicitly defined
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_yaml::Value>,
}

/// Represents a repository in the pre-commit configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    /// The repository URL or local path
    pub repo: String,
    /// The revision (branch, tag, or commit) to use
    pub rev: String,
    /// The hooks defined in this repository
    pub hooks: Vec<Hook>,
}

/// Represents the top-level pre-commit configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PreCommitConfig {
    /// The minimum compatible version of pre-commit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_pre_commit_version: Option<String>,
    /// The repositories containing hooks
    pub repos: Vec<Repository>,
    /// Whether to use default stages when not specified by a hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_stages: Option<Vec<String>>,
    /// Additional configuration options that aren't explicitly defined
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_yaml::Value>,
}

impl PreCommitConfig {
    /// Parse a pre-commit config file from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Parse a pre-commit config from a string
    pub fn from_str(content: &str) -> Result<Self, Box<dyn Error>> {
        let config: PreCommitConfig = serde_yaml::from_str(content)?;
        Ok(config)
    }

    /// Get all hook IDs from all repositories
    pub fn get_all_hook_ids(&self) -> Vec<String> {
        self.repos
            .iter()
            .flat_map(|repo| repo.hooks.iter().map(|hook| hook.id.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let yaml = r#"
repos:
-   repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
    -   id: trailing-whitespace
    -   id: end-of-file-fixer
    -   id: check-yaml
-   repo: https://github.com/psf/black
    rev: 23.1.0
    hooks:
    -   id: black
        "#;

        let config = PreCommitConfig::from_str(yaml).unwrap();

        assert_eq!(config.repos.len(), 2);
        assert_eq!(
            config.repos[0].repo,
            "https://github.com/pre-commit/pre-commit-hooks"
        );
        assert_eq!(config.repos[0].rev, "v4.4.0");
        assert_eq!(config.repos[0].hooks.len(), 3);
        assert_eq!(config.repos[0].hooks[0].id, "trailing-whitespace");
        assert_eq!(config.repos[0].hooks[1].id, "end-of-file-fixer");
        assert_eq!(config.repos[0].hooks[2].id, "check-yaml");

        assert_eq!(config.repos[1].repo, "https://github.com/psf/black");
        assert_eq!(config.repos[1].rev, "23.1.0");
        assert_eq!(config.repos[1].hooks.len(), 1);
        assert_eq!(config.repos[1].hooks[0].id, "black");
    }

    #[test]
    fn test_parse_complex_config() {
        let yaml = r#"
minimum_pre_commit_version: '2.9.2'
default_stages: [commit, push]
repos:
-   repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
    -   id: trailing-whitespace
        args: [--markdown-linebreak-ext=md]
        exclude: ^docs/CHANGELOG.md$
    -   id: check-toml
        files: ^(pyproject.toml|Cargo.toml)$
-   repo: local
    hooks:
    -   id: cargo-check
        name: cargo check
        entry: cargo check
        language: system
        pass_filenames: false
        always_run: true
        types: [rust]
        "#;

        let config = PreCommitConfig::from_str(yaml).unwrap();

        assert_eq!(config.minimum_pre_commit_version, Some("2.9.2".to_string()));
        assert_eq!(
            config.default_stages,
            Some(vec!["commit".to_string(), "push".to_string()])
        );
        assert_eq!(config.repos.len(), 2);

        // Check first repo
        let repo1 = &config.repos[0];
        assert_eq!(repo1.repo, "https://github.com/pre-commit/pre-commit-hooks");
        assert_eq!(repo1.rev, "v4.4.0");
        assert_eq!(repo1.hooks.len(), 2);

        // Check trailing-whitespace hook
        let hook1 = &repo1.hooks[0];
        assert_eq!(hook1.id, "trailing-whitespace");
        assert_eq!(
            hook1.args,
            Some(vec!["--markdown-linebreak-ext=md".to_string()])
        );
        assert_eq!(hook1.exclude, Some("^docs/CHANGELOG.md$".to_string()));

        // Check local repo
        let repo2 = &config.repos[1];
        assert_eq!(repo2.repo, "local");
        assert_eq!(repo2.hooks.len(), 1);

        // Check cargo-check hook
        let hook3 = &repo2.hooks[0];
        assert_eq!(hook3.id, "cargo-check");
        assert_eq!(hook3.name, Some("cargo check".to_string()));
        assert_eq!(hook3.entry, Some("cargo check".to_string()));
        assert_eq!(hook3.language, Some("system".to_string()));
        assert_eq!(hook3.pass_filenames, Some(false));
        assert_eq!(hook3.always_run, Some(true));
        assert_eq!(hook3.types, Some(vec!["rust".to_string()]));
    }
}
