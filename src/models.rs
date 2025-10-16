/*
 *  This file is part of nzbget. See <https://github.com/DKlintukhov/PCDoctor>.
 *
 *  Copyright (C) 2025 Denis <denis.klintukhov@gmail.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuConfig {
    pub version: String,
    pub menus: HashMap<String, Menu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub title: String,
    pub description: String,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: Option<f64>,
    pub category: ServiceCategory,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceCategory {
    Windows,
    Linux,
    #[serde(rename = "macOS")]
    MacOS,
    Development,
    TelegramBots,
    Other,
}

pub fn parse_menu(path: &PathBuf) -> Result<MenuConfig, String> {
    let json_str = fs::read_to_string(path).map_err(|e| format!("Failed to read config: {e}"))?;

    let menu =
        serde_json::from_str::<MenuConfig>(&json_str).map_err(|e| format!("Invalid JSON: {e}"))?;

    Ok(menu)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_temp_file(content: &str) -> std::path::PathBuf {
        let mut temp_file = std::env::temp_dir();
        temp_file.push(format!("test_menu_{}.json", std::process::id()));

        let mut file = fs::File::create(&temp_file).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        temp_file
    }

    #[test]
    fn test_parse_menu_file_not_found() {
        let none_existing_file = PathBuf::from("/nonexistent/path/menu.json");
        let result = parse_menu(&none_existing_file);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.contains("Failed to read config"));
    }

    #[test]
    fn test_parse_menu_invalid_json() {
        let temp_path = create_temp_file("invalid json content");
        let result = parse_menu(&temp_path);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid JSON"));
        assert!(fs::remove_file(temp_path).is_ok());
    }

    #[test]
    fn test_service_category_serialization() {
        let test_cases = [
            (ServiceCategory::Windows, "\"Windows\""),
            (ServiceCategory::Linux, "\"Linux\""),
            (ServiceCategory::MacOS, "\"macOS\""),
            (ServiceCategory::Development, "\"Development\""),
            (ServiceCategory::TelegramBots, "\"TelegramBots\""),
            (ServiceCategory::Other, "\"Other\""),
        ];

        for (category, expected) in test_cases {
            let serialized = serde_json::to_string(&category).unwrap();
            assert_eq!(serialized, expected);

            let deserialized: ServiceCategory = serde_json::from_str(expected).unwrap();
            assert_eq!(deserialized, category);
        }
    }
}
