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

use std::fs;
use std::io;
use teloxide::{
    prelude::*,
    types::{KeyboardButton, KeyboardMarkup},
};
use serde_json;
mod models;
use models::{MenuConfig};

static MENU_CONFIG: &str = "menu.json";

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();
    // pretty_env_logger::init();

    //let bot = Bot::from_env();
    let json_str = fs::read_to_string(MENU_CONFIG).unwrap();
    let res = serde_json::from_str::<MenuConfig>(&json_str);
    println!("{:?}", res);
}
