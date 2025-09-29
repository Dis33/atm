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

use constcat::concat;

const fn etc() -> &'static str {
    #[cfg(unix)]
    return "/etc";
    #[cfg(windows)]
    return "C:/ProgramData";
}

const fn run() -> &'static str {
    #[cfg(unix)]
    return "/run";
    #[cfg(windows)]
    return "C:/ProgramData";
}

pub const fn packages_file() -> &'static str {
    concat!(etc(), "/atm/packages.toml")
}

pub const fn lock_file() -> &'static str {
    #[cfg(unix)]
    return concat!(run(), "atm.lock");
    #[cfg(windows)]
    return concat!(run(), "/atm/atm.lock");
}
