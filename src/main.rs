/*
 *  This file is part of nzbget. See <https://github.com/DKlintukhov/PCDoctorBot>.
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

use teloxide::{
    prelude::*,
    types::{KeyboardButton, KeyboardMarkup},
};

fn main_menu_keyboard() -> KeyboardMarkup {
    let buttons = vec![
        vec![KeyboardButton::new("🖥️ Настройка ПК")],
        vec![KeyboardButton::new("🐧 Установка ОС")],
        vec![KeyboardButton::new("🌐 Разработка сайтов")],
        vec![KeyboardButton::new("🖥️ Десктопные приложения")],
        vec![KeyboardButton::new("🤖 Телеграм-боты")],
        vec![
            KeyboardButton::new("📞 Заказать звонок"),
            KeyboardButton::new("📨 Оставить сообщение"),
        ],
    ];

    KeyboardMarkup::new(buttons).resize_keyboard()
}

fn os_install_keyboard() -> KeyboardMarkup {
    let buttons = vec![
        vec![KeyboardButton::new("Windows")],
        vec![KeyboardButton::new("Linux")],
        vec![KeyboardButton::new("⬅️ Назад")],
    ];

    KeyboardMarkup::new(buttons).resize_keyboard()
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let text = msg.text().unwrap_or_default();

        match text {
            "/start" => {
                let keyboard = main_menu_keyboard();
                bot.send_message(msg.chat.id, "Выберите категорию помощи:")
                    .reply_markup(keyboard)
                    .await?;
            }
            "🖥️ Настройка ПК" => {
                bot.send_message(msg.chat.id, "Опишите проблему с настройкой ПК:")
                    .await?;
            }
            "🐧 Установка ОС" => {
                let keyboard = os_install_keyboard();
                bot.send_message(msg.chat.id, "Выберите ОС:")
                    .reply_markup(keyboard)
                    .await?;
            }
            "📞 Заказать звонок" => {
                bot.send_message(msg.chat.id, "Введите ваш номер телефона, и мы перезвоним:")
                    .await?;
            }
            "🎤 Оставить голосовое сообщение" => {
                bot.send_message(msg.chat.id, "Запишите голосовое сообщение:")
                    .await?;
            }
            "❓ Другое" => {
                bot.send_message(msg.chat.id, "Опишите ваш запрос:").await?;
            }
            "⬅️ Назад" => {
                let keyboard = main_menu_keyboard();
                bot.send_message(msg.chat.id, "Главное меню:")
                    .reply_markup(keyboard)
                    .await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Неизвестная команда. Используйте меню.")
                    .await?;
            }
        }

        Ok(())
    })
    .await;
}
