//! Interactive configuration wizard for batless
//! This module provides an interactive CLI wizard to help users configure
//! batless for their specific needs, creating custom profiles and setting
//! up optimal configurations.

use crate::error::{BatlessError, BatlessResult};
use crate::language::LanguageDetector;
use crate::profile::CustomProfile;
use crate::summary::SummaryLevel;
use crate::tokens::AiModel;
use std::io::{self, Write};

/// Interactive configuration wizard
pub struct ConfigurationWizard;

impl ConfigurationWizard {
    /// Run the interactive configuration wizard
    pub fn run() -> BatlessResult<()> {
        loop {
            println!("\n🧙 Welcome to the batless Configuration Wizard!\n");
            println!("1. Create a new profile");
            println!("2. List existing profiles");
            println!("3. Edit a profile");
            println!("4. Delete a profile");
            println!("5. Exit");

            let choice = Self::prompt_string("\nEnter your choice", Some("1"))?;
            match choice.as_str() {
                "1" => Self::create_profile()?,
                "2" => Self::list_profiles()?,
                "3" => Self::edit_profile_interactive()?,
                "4" => Self::delete_profile_interactive()?,
                "5" => return Ok(()),
                _ => println!("❌ Invalid choice, please try again."),
            }
        }
    }

    fn create_profile() -> BatlessResult<()> {
        println!("\n✨ Creating a new profile...\n");
        let profile = Self::gather_profile_info()?;
        let save_location = Self::choose_save_location(&profile.name)?;
        profile.save_to_file(&save_location)?;
        println!(
            "\n✅ Profile '{}' created successfully at {}.",
            profile.name,
            save_location.display()
        );
        Ok(())
    }

    /// Gather profile information interactively
    fn gather_profile_info() -> BatlessResult<CustomProfile> {
        // Profile name and description
        let name = Self::prompt_string("Profile name", Some("my-profile"))?;
        let description = Self::prompt_optional_string("Profile description (optional)", None)?;

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
            Self::prompt_optional_string("Language", None)?
        } else {
            None
        };

        // Theme
        let theme = if Self::prompt_yes_no("Customize syntax highlighting theme?", false)? {
            let available_themes = crate::language::ThemeManager::list_themes();
            println!("Available themes: {}", available_themes.join(", "));
            Self::prompt_optional_string("Theme", None)?
        } else {
            None
        };

        // Color and ANSI settings
        let use_color = Self::prompt_optional_yes_no("Use color output?", None)?;
        let strip_ansi = Self::prompt_optional_yes_no("Strip ANSI escape codes?", None)?;

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
            Self::prompt_optional_string("AI model", None)?
        } else {
            None
        };

        // Token extraction
        let include_tokens =
            Self::prompt_optional_yes_no("Include token extraction in JSON output?", None)?;

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
            Self::prompt_optional_string("Output mode", None)?
        } else {
            None
        };

        println!("\n⚡ Performance Settings");

        // Streaming settings
        let streaming_json =
            Self::prompt_optional_yes_no("Enable streaming JSON for large files?", None)?;
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
            Self::prompt_optional_yes_no("Enable resume capability?", None)?
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
            if let Err(e) = io::stdout().flush() {
                return Err(BatlessError::config_error_with_help(
                    format!("Failed to flush output: {e}"),
                    Some("Terminal output error".to_string()),
                ));
            }

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
    fn prompt_optional_string(
        prompt: &str,
        default: Option<&str>,
    ) -> BatlessResult<Option<String>> {
        print!("🔹 {prompt}");
        if let Some(default_val) = default {
            print!(" [{default_val}]");
        }
        print!(": ");
        if let Err(e) = io::stdout().flush() {
            return Err(BatlessError::config_error_with_help(
                format!("Failed to flush output: {e}"),
                Some("Terminal output error".to_string()),
            ));
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read input: {e}"),
                Some("Please try again".to_string()),
            )
        })?;

        let input = input.trim();
        if input.is_empty() {
            Ok(default.map(|s| s.to_string()))
        } else {
            Ok(Some(input.to_string()))
        }
    }

    /// Prompt for a yes/no answer
    fn prompt_yes_no(prompt: &str, default: bool) -> BatlessResult<bool> {
        let default_str = if default { "Y/n" } else { "y/N" };

        loop {
            print!("🔹 {prompt} [{default_str}]: ");
            if let Err(e) = io::stdout().flush() {
                return Err(BatlessError::config_error_with_help(
                    format!("Failed to flush output: {e}"),
                    Some("Terminal output error".to_string()),
                ));
            }

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
    fn prompt_optional_yes_no(prompt: &str, default: Option<bool>) -> BatlessResult<Option<bool>> {
        let default_str = match default {
            Some(true) => "Y/n/skip",
            Some(false) => "y/N/skip",
            None => "y/n/skip",
        };

        print!("🔹 {prompt} [{default_str}]: ");
        if let Err(e) = io::stdout().flush() {
            return Err(BatlessError::config_error_with_help(
                format!("Failed to flush output: {e}"),
                Some("Terminal output error".to_string()),
            ));
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            BatlessError::config_error_with_help(
                format!("Failed to read input: {e}"),
                Some("Please try again".to_string()),
            )
        })?;

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "" | "skip" | "s" => Ok(default),
            "y" | "yes" | "true" | "1" => Ok(Some(true)),
            "n" | "no" | "false" | "0" => Ok(Some(false)),
            _ => {
                println!("❌ Please answer y/yes, n/no, or skip");
                Self::prompt_optional_yes_no(prompt, default)
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
            if let Err(e) = io::stdout().flush() {
                return Err(BatlessError::config_error_with_help(
                    format!("Failed to flush output: {e}"),
                    Some("Terminal output error".to_string()),
                ));
            }

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
        if let Err(e) = io::stdout().flush() {
            return Err(BatlessError::config_error_with_help(
                format!("Failed to flush output: {e}"),
                Some("Terminal output error".to_string()),
            ));
        }

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

    fn delete_profile_interactive() -> BatlessResult<()> {
        println!("\n🗑️ Select a profile to delete:");
        let profiles = Self::get_available_profiles()?;
        if profiles.is_empty() {
            println!("No profiles found.");
            return Ok(());
        }

        for (i, (path, profile)) in profiles.iter().enumerate() {
            println!("{}. {} ({})", i + 1, profile.name, path.display());
        }

        let choice = Self::prompt_number("Enter your choice", None, 1, profiles.len())?;
        let (path, profile) = &profiles[choice - 1];

        if Self::prompt_yes_no(
            &format!(
                "Are you sure you want to delete profile '{}'?",
                profile.name
            ),
            false,
        )? {
            std::fs::remove_file(path)?;
            println!("✅ Profile deleted successfully.");
        }

        Ok(())
    }

    fn get_available_profiles() -> BatlessResult<Vec<(std::path::PathBuf, CustomProfile)>> {
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
            return Ok(Vec::new());
        }

        let entries = std::fs::read_dir(&profiles_dir)?;
        let mut profiles = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(profile) = CustomProfile::load_from_file(&path) {
                    profiles.push((path, profile));
                }
            }
        }
        Ok(profiles)
    }

    /// Show a summary of existing profiles
    pub fn list_profiles() -> BatlessResult<()> {
        let profiles = Self::get_available_profiles()?;
        if profiles.is_empty() {
            println!("📁 No valid profiles found. Run 'batless --configure' to create your first profile.");
            return Ok(());
        }

        let count = profiles.len();
        let latest_updated = profiles
            .iter()
            .filter_map(|(_, p)| p.updated_at.as_ref())
            .max()
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());

        println!("\n📋 Available Profiles (total: {count}, latest update: {latest_updated}):");
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

    fn edit_profile_interactive() -> BatlessResult<()> {
        println!("\n✏️ Select a profile to edit:");
        let profiles = Self::get_available_profiles()?;
        if profiles.is_empty() {
            println!("No profiles found. Create one first!");
            return Ok(());
        }

        for (i, (path, profile)) in profiles.iter().enumerate() {
            println!("{}. {} ({})", i + 1, profile.name, path.display());
        }

        let choice = Self::prompt_number("Enter your choice", None, 1, profiles.len())?;
        let (path, _) = &profiles[choice - 1];

        Self::edit_profile_by_path(path.to_str().unwrap())
    }

    /// Interactive profile editor for existing profiles
    pub fn edit_profile_by_path(profile_path: &str) -> BatlessResult<()> {
        let path = std::path::Path::new(profile_path);
        if !path.exists() {
            return Err(BatlessError::config_error_with_help(
                format!("Profile not found: {profile_path}"),
                Some("Use 'batless --list-profiles' to see available profiles".to_string()),
            ));
        }

        let mut profile = CustomProfile::load_from_file(path)?;

        println!("\n✏️  Editing profile: {}", profile.name);
        println!("Press Enter to keep the current value.\n");

        profile.description = Self::prompt_optional_string(
            &format!(
                "Description [{}]",
                profile.description.as_deref().unwrap_or("")
            ),
            profile.description.as_deref(),
        )?;

        let max_lines = Self::prompt_optional_number(
            &format!(
                "Max lines (0 for unlimited) [{}]",
                profile
                    .max_lines
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "0".to_string())
            ),
            profile.max_lines,
            0,
            1_000_000,
        )?;
        profile.max_lines = if max_lines == Some(0) {
            None
        } else {
            max_lines
        };

        let max_bytes = Self::prompt_optional_number(
            &format!(
                "Max bytes (MB) [{}]",
                profile
                    .max_bytes
                    .map(|b| (b / (1024 * 1024)).to_string())
                    .unwrap_or_else(|| "None".to_string())
            ),
            profile.max_bytes.map(|b| b / (1024 * 1024)),
            1,
            100,
        )?;
        profile.max_bytes = max_bytes.map(|mb| mb * 1024 * 1024);

        profile.language = Self::prompt_optional_string(
            &format!(
                "Language [{}]",
                profile.language.as_deref().unwrap_or("auto")
            ),
            profile.language.as_deref(),
        )?;
        profile.theme = Self::prompt_optional_string(
            &format!("Theme [{}]", profile.theme.as_deref().unwrap_or("default")),
            profile.theme.as_deref(),
        )?;
        profile.use_color = Self::prompt_optional_yes_no(
            &format!(
                "Use color? [{}]",
                profile
                    .use_color
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "auto".to_string())
            ),
            profile.use_color,
        )?;
        profile.strip_ansi = Self::prompt_optional_yes_no(
            &format!(
                "Strip ANSI? [{}]",
                profile
                    .strip_ansi
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "auto".to_string())
            ),
            profile.strip_ansi,
        )?;
        profile.ai_model = Self::prompt_optional_string(
            &format!(
                "AI model [{}]",
                profile.ai_model.as_deref().unwrap_or("none")
            ),
            profile.ai_model.as_deref(),
        )?;
        profile.include_tokens = Self::prompt_optional_yes_no(
            &format!(
                "Include tokens? [{}]",
                profile
                    .include_tokens
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "auto".to_string())
            ),
            profile.include_tokens,
        )?;
        profile.summary_level = Self::prompt_optional_string(
            &format!(
                "Summary level [{}]",
                profile
                    .summary_level
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("none")
            ),
            profile.summary_level.as_ref().map(|s| s.as_str()),
        )?
        .map(|s| SummaryLevel::parse(&s).unwrap());
        profile.output_mode = Self::prompt_optional_string(
            &format!(
                "Output mode [{}]",
                profile.output_mode.as_deref().unwrap_or("default")
            ),
            profile.output_mode.as_deref(),
        )?;
        profile.streaming_json = Self::prompt_optional_yes_no(
            &format!(
                "Streaming JSON? [{}]",
                profile
                    .streaming_json
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "auto".to_string())
            ),
            profile.streaming_json,
        )?;
        profile.streaming_chunk_size = Self::prompt_optional_number(
            &format!(
                "Streaming chunk size [{}]",
                profile
                    .streaming_chunk_size
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "1000".to_string())
            ),
            profile.streaming_chunk_size,
            100,
            10000,
        )?;
        profile.enable_resume = Self::prompt_optional_yes_no(
            &format!(
                "Enable resume? [{}]",
                profile
                    .enable_resume
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "auto".to_string())
            ),
            profile.enable_resume,
        )?;
        profile.tags = Self::prompt_tags()?;

        profile.updated_at = Some(chrono::Utc::now().to_rfc3339());

        profile.save_to_file(path)?;

        println!("\n✅ Profile updated successfully!");
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
