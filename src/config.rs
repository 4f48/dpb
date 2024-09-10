/*
 * This file is part of dpb.
 *
 * Copyright (C) 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * dpb is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License, version 3, as published by the Free Software Foundation.
 *
 * dpb is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with dpb.
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

use serde::Deserialize;
use toml::value::Array;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Toml {
    pub config: Config,
    pub A: A,
    pub AAAA: AAAA,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub key: String,
    pub secret: String,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct A {
    pub subdomains: Array,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Deserialize)]
pub struct AAAA {
    pub subdomains: Array,
}

pub fn get_config(file_path: &str) -> Result<Toml, Box<dyn std::error::Error>> {
    Ok(toml::from_str::<Toml>(&std::fs::read_to_string(
        file_path,
    )?)?)
}
