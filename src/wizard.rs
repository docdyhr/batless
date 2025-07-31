//! Interactive configuration wizard for batless
//!
//! This module provides an interactive CLI wizard to help users configure
//! batless for their specific needs, creating custom profiles and setting
//! up optimal configurations.

use crate::config::{CustomProfile, SummaryLevel};
use crate::error::{BatlessError, BatlessResult};
use crate::language::LanguageDetector;
use crate::token_counter::AiModel;
use std::io::{self, Write};

/// Interactive configuration wizard
pub struct ConfigurationWizard;

impl ConfigurationWizard {
    /// Run the interactive configuration wizard
    pub fn run() -> BatlessResult<()> {
        println!("🧙 Welcome to the batless Configuration Wizard!");
        println!("This wizard will help you create a custom configuration profile.\n");

        let profile = Self::gather_profile_info()?;
        let save_location = Self::choose_save_location(&profile.name)?;

        // Save the profile
        profile.save_to_file(&save_location)?;

        println!("\n✅ Configuration profile saved successfully!");
        println!("📁 Location: {}", save_location.display());
        println!("\n💡 Usage tips:");
        println!(
            "  batless --custom-profile \"{}\" myfile.rs",
            save_location.display()
        );
        println!("  batless --configure  # Run this wizard again");

        Ok(())
    }

    /// Gather profile information interactively
    fn gather_profile_info() -> BatlessResult<CustomProfile> {
        // Profile name and description
        let name = Self::prompt_string("Profile name", Some("my-profile"))?;
        let description = Self::prompt_optional_string("Profile description (optional)")?;

        println!("\n📊 File Processing Settings");

        // Max lines
        let max_lines = Self::prompt_number(
            "Maximum lines to process (0 for unlimited)",
            Some(10000),
            0,
            1_000_000,
        )?;
        let max_lines = if max_lines == 0 {
            None
        } else {
            Some(max_lines)
        };

        // Max bytes
        let max_bytes =
            Self::prompt_optional_number("Maximum bytes to process (optional, MB)", None, 1, 100)?;
        let max_bytes = max_bytes.map(|mb| mb * 1024 * 1024); // Convert MB to bytes

        println!("\n🎨 Display Settings");

        // Language override
        let language = if Self::prompt_yes_no("Override language detection?", false)? {
            let available_languages = LanguageDetector::list_languages();
            println!("Available languages: {}", available_languages.join(", "));
            Self::prompt_optional_string("Language")?
        } else {
            None
        };

        // Theme
        let theme = if Self::prompt_yes_no("Customize syntax highlighting theme?", false)? {
            let available_themes = crate::language::ThemeManager::list_themes();
            println!("Available themes: {}", available_themes.join(", "));
            Self::prompt_optional_string("Theme")?
        } else {
            None
        };

        // Color and ANSI settings
        let use_color = Self::prompt_optional_yes_no("Use color output?")?;
        let strip_ansi = Self::prompt_optional_yes_no("Strip ANSI escape codes?")?;

        println!("\n🤖 AI Integration Settings");

        // AI model preference
        let ai_model = if Self::prompt_yes_no("Set AI model preference?", false)? {
            println!("Available models:");
            for model in AiModel::all() {
                println!(
                    "  {} - {} tokens context",
                    model.as_str(),
                    model.context_window()
                );
            }
            Self::prompt_optional_string("AI model")?
        } else {
            None
        };

        // Token extraction
        let include_tokens =
            Self::prompt_optional_yes_no("Include token extraction in JSON output?")?;

        // Summary level
        let summary_level = if Self::prompt_yes_no("Configure code summarization?", false)? {
            println!("Summary levels:");
            println!("  none - No summarization");
            println!("  minimal - Functions and exports only");
            println!("  standard - Functions, classes, imports");
            println!("  detailed - Includes comments and complexity");

            let level_str = Self::prompt_string("Summary level", Some("standard"))?;
            Some(SummaryLevel::parse(&level_str).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Invalid summary level: {e}"),
                    Some("Valid levels: none, minimal, standard, detailed".to_string()),
                )
            })?)
        } else {
            None
        };

        // Output mode preference
        let output_mode = if Self::prompt_yes_no("Set default output mode?", false)? {
            println!("Output modes: plain, highlight, json, summary");
            Self::prompt_optional_string("Output mode")?
        } else {
            None
        };

        println!("\n⚡ Performance Settings");

        // Streaming settings
        let streaming_json =
            Self::prompt_optional_yes_no("Enable streaming JSON for large files?")?;
        let streaming_chunk_size = if streaming_json == Some(true) {
            Some(Self::prompt_number(
                "Streaming chunk size (lines)",
                Some(1000),
                100,
                10000,
            )?)
        } else {
            None
        };

        let enable_resume = if streaming_json == Some(true) {
            Self::prompt_optional_yes_no("Enable resume capability?")?
        } else {
            None
        };

        // Tags for organization
        let tags = if Self::prompt_yes_no("Add tags for organization?", false)? {
            Self::prompt_tags()?
        } else {
            Vec::new()
        };

        // Create the profile
        let mut profile = CustomProfile::new(name, description);
        profile.max_lines = max_lines;
        profile.max_bytes = max_bytes;
        profile.language = language;
        profile.theme = theme;
        profile.use_color = use_color;
        profile.strip_ansi = strip_ansi;
        profile.ai_model = ai_model;
        profile.include_tokens = include_tokens;
        profile.summary_level = summary_level;
        profile.output_mode = output_mode;
        profile.streaming_json = streaming_json;
        profile.streaming_chunk_size = streaming_chunk_size;
        profile.enable_resume = enable_resume;
        profile.tags = tags;

        // Set timestamps
        let now = chrono::Utc::now().to_rfc3339();
        profile.created_at = Some(now.clone());
        profile.updated_at = Some(now);

        Ok(profile)
    }

    /// Choose where to save the profile
    fn choose_save_location(profile_name: &str) -> BatlessResult<std::path::PathBuf> {
        let default_dir = dirs::home_dir()
            .ok_or_else(|| {
                BatlessError::config_error_with_help(
                    "Could not determine home directory".to_string(),
                    Some("Please specify a custom save location".to_string()),
                )
            })?
            .join(".batless")
            .join("profiles");

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(&default_dir).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Could not create profiles directory: {e}"),
                Some("Check permissions or specify a different location".to_string()),
            )
        })?;

        let default_path = default_dir.join(format!("{profile_name}.json"));

        println!("\n💾 Save Location");
        println!("Default: {}", default_path.display());

        if Self::prompt_yes_no("Use default location?", true)? {
            Ok(default_path)
        } else {
            let custom_path = Self::prompt_string("Custom path", None)?;
            Ok(std::path::PathBuf::from(custom_path))
        }
    }

    /// Prompt for a string value
    fn prompt_string(prompt: &str, default: Option<&str>) -> BatlessResult<String> {
        loop {
            print!("🔹 {prompt}");
            if let Some(default_val) = default {
                print!(" [{default_val}]");
            }
            print!(": ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to read input: {e}"),
                    Some("Please try again".to_string()),
                )
            })?;

            let input = input.trim();
            if !input.is_empty() {
                return Ok(input.to_string());
            } else if let Some(default_val) = default {
                return Ok(default_val.to_string());
            }

            println!("❌ Please provide a value or press Enter for default");
        }
    }

    /// Prompt for an optional string value
    fn prompt_optional_string(prompt: &str) -> BatlessResult<Option<String>> {
        print!("🔹 {prompt}: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read input: {e}"),
                Some("Please try again".to_string()),
            )
        })?;

        let input = input.trim();
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(input.to_string()))
        }
    }

    /// Prompt for a yes/no answer
    fn prompt_yes_no(prompt: &str, default: bool) -> BatlessResult<bool> {
        let default_str = if default { "Y/n" } else { "y/N" };

        loop {
            print!("🔹 {prompt} [{default_str}]: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to read input: {e}"),
                    Some("Please try again".to_string()),
                )
            })?;

            let input = input.trim().to_lowercase();
            match input.as_str() {
                "" => return Ok(default),
                "y" | "yes" | "true" | "1" => return Ok(true),
                "n" | "no" | "false" | "0" => return Ok(false),
                _ => println!("❌ Please answer y/yes or n/no"),
            }
        }
    }

    /// Prompt for an optional yes/no answer
    fn prompt_optional_yes_no(prompt: &str) -> BatlessResult<Option<bool>> {
        print!("🔹 {prompt} [y/n/skip]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read input: {e}"),
                Some("Please try again".to_string()),
            )
        })?;

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "" | "skip" | "s" => Ok(None),
            "y" | "yes" | "true" | "1" => Ok(Some(true)),
            "n" | "no" | "false" | "0" => Ok(Some(false)),
            _ => {
                println!("❌ Please answer y/yes, n/no, or skip");
                Self::prompt_optional_yes_no(prompt)
            }
        }
    }

    /// Prompt for a number within a range
    fn prompt_number(
        prompt: &str,
        default: Option<usize>,
        min: usize,
        max: usize,
    ) -> BatlessResult<usize> {
        loop {
            print!("🔹 {prompt}");
            if let Some(default_val) = default {
                print!(" [{default_val}]");
            }
            print!(" ({min}-{max}): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Failed to read input: {e}"),
                    Some("Please try again".to_string()),
                )
            })?;

            let input = input.trim();
            if input.is_empty() {
                if let Some(default_val) = default {
                    return Ok(default_val);
                }
            } else if let Ok(num) = input.parse::<usize>() {
                if num >= min && num <= max {
                    return Ok(num);
                } else {
                    println!("❌ Number must be between {min} and {max}");
                    continue;
                }
            }

            println!("❌ Please enter a valid number between {min} and {max}");
        }
    }

    /// Prompt for an optional number
    fn prompt_optional_number(
        prompt: &str,
        default: Option<usize>,
        min: usize,
        max: usize,
    ) -> BatlessResult<Option<usize>> {
        print!("🔹 {prompt}");
        if let Some(default_val) = default {
            print!(" [{default_val}]");
        }
        print!(" ({min}-{max}, or skip): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read input: {e}"),
                Some("Please try again".to_string()),
            )
        })?;

        let input = input.trim();
        if input.is_empty() || input.eq_ignore_ascii_case("skip") {
            Ok(default)
        } else if let Ok(num) = input.parse::<usize>() {
            if num >= min && num <= max {
                Ok(Some(num))
            } else {
                println!("❌ Number must be between {min} and {max}");
                Self::prompt_optional_number(prompt, default, min, max)
            }
        } else {
            println!("❌ Please enter a valid number, or 'skip'");
            Self::prompt_optional_number(prompt, default, min, max)
        }
    }

    /// Prompt for tags
    fn prompt_tags() -> BatlessResult<Vec<String>> {
        println!("🔹 Enter tags separated by commas (e.g., 'coding, rust, development'):");
        print!("Tags: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read input: {e}"),
                Some("Please try again".to_string()),
            )
        })?;

        let tags: Vec<String> = input
            .trim()
            .split(',')
            .filter_map(|tag| {
                let tag = tag.trim();
                if tag.is_empty() {
                    None
                } else {
                    Some(tag.to_string())
                }
            })
            .collect();

        Ok(tags)
    }

    /// Show a summary of existing profiles
    pub fn list_profiles() -> BatlessResult<()> {
        let profiles_dir = dirs::home_dir()
            .ok_or_else(|| {
                BatlessError::config_error_with_help(
                    "Could not determine home directory".to_string(),
                    Some("Profiles directory not accessible".to_string()),
                )
            })?
            .join(".batless")
            .join("profiles");

        if !profiles_dir.exists() {
            println!("📁 No profiles directory found. Run 'batless --configure' to create your first profile.");
            return Ok(());
        }

        let entries = std::fs::read_dir(&profiles_dir).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Could not read profiles directory: {e}"),
                Some("Check permissions on the profiles directory".to_string()),
            )
        })?;

        let mut profiles = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                BatlessError::config_error_with_help(
                    format!("Error reading directory entry: {e}"),
                    Some("Check profiles directory permissions".to_string()),
                )
            })?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match CustomProfile::load_from_file(&path) {
                    Ok(profile) => profiles.push((path, profile)),
                    Err(_) => continue, // Skip invalid profiles
                }
            }
        }

        if profiles.is_empty() {
            println!("📁 No valid profiles found. Run 'batless --configure' to create your first profile.");
            return Ok(());
        }

        println!("📋 Available Profiles:");
        println!("{}", "─".repeat(80));

        for (path, profile) in profiles {
            println!(
                "🔹 {} ({})",
                profile.name,
                path.file_name().unwrap().to_string_lossy()
            );

            if let Some(ref description) = profile.description {
                println!("   Description: {description}");
            }

            println!(
                "   Settings: {} lines max, {} mode",
                profile
                    .max_lines
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "unlimited".to_string()),
                profile.output_mode.as_deref().unwrap_or("default")
            );

            if !profile.tags.is_empty() {
                println!("   Tags: {}", profile.tags.join(", "));
            }

            if let Some(ref updated) = profile.updated_at {
                println!("   Updated: {updated}");
            }

            println!(
                "   Usage: batless --custom-profile \"{}\" myfile.rs",
                path.display()
            );
            println!();
        }

        Ok(())
    }

    /// Interactive profile editor for existing profiles
    pub fn edit_profile(profile_path: &str) -> BatlessResult<()> {
        let path = std::path::Path::new(profile_path);
        if !path.exists() {
            return Err(BatlessError::config_error_with_help(
                format!("Profile not found: {profile_path}"),
                Some("Use 'batless --list-profiles' to see available profiles".to_string()),
            ));
        }

        let mut profile = CustomProfile::load_from_file(path)?;

        println!("✏️  Editing profile: {}", profile.name);
        println!(
            "Current description: {}",
            profile.description.as_deref().unwrap_or("None")
        );
        println!();

        // Allow editing each field
        if Self::prompt_yes_no("Update description?", false)? {
            profile.description = Self::prompt_optional_string("New description")?;
        }

        if Self::prompt_yes_no("Update max lines?", false)? {
            let max_lines = Self::prompt_optional_number(
                "Max lines (0 for unlimited)",
                profile.max_lines,
                0,
                1_000_000,
            )?;
            profile.max_lines = if max_lines == Some(0) {
                None
            } else {
                max_lines
            };
        }

        // Add more fields as needed...

        // Update timestamp
        profile.updated_at = Some(chrono::Utc::now().to_rfc3339());

        // Save the updated profile
        profile.save_to_file(path)?;

        println!("✅ Profile updated successfully!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigurationWizard;

    #[test]
    fn test_wizard_module_exists() {
        // Basic test to ensure the module compiles and types are accessible
        let _wizard = ConfigurationWizard;
    }

    // Note: Interactive tests are difficult to unit test
    // Integration tests would be better for the wizard functionality
}
