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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCategory {
    Windows,
    Linux,
    macOS,
    Development,
    TelegramBots,
    Other,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
}
