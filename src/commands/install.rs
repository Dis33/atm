/*
 * Copyright (C) 2025  Yeong-won Seo
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::commands::Command;
use async_trait::async_trait;

pub struct Install;

impl Install {
    pub const fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for Install {
    fn is_target(&self, arg: &str) -> bool {
        arg == "sync"
    }

    async fn run(&self, args: &[String]) {
        todo!()
    }
}
