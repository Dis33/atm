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
use crate::commands::install::Install;
use async_trait::async_trait;

mod install;

#[async_trait]
pub trait Command: Sync + Send {
    fn is_target(&self, arg: &str) -> bool;
    async fn run(&self, args: &[String]);
}

static COMMANDS: [&dyn Command; 1] = [&Install::new()];

pub const fn commands() -> &'static [&'static dyn Command] {
    &COMMANDS
}
