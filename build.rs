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

use nextver::{CalSem, CalSemLevel, Date};
use regex::Regex;
use std::fs;

const FORMAT: &str = "<YYYY>.<0W>.<PATCH>"; // ISO 8601 week dates

fn main() {
    let version = fs::read_to_string("VERSION")
        .expect("Could not read VERSION")
        .trim()
        .to_string();

    let version =
        CalSem::next_version_string(FORMAT, &version, Date::utc_now(), CalSemLevel::Patch)
            .expect("Failed to parse VERSION");

    fs::write("VERSION", &version).expect("Failed to write VERSION file");

    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Could not read Cargo.toml");

    let cargo_toml = Regex::new("version = \"[0-9.]+\"")
        .unwrap()
        .replace(cargo_toml.as_str(), &format!("version = \"{}\"", version));

    fs::write("Cargo.toml", cargo_toml.as_ref()).expect("Could not write updated Cargo.toml");
}
