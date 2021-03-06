/*
 * Created on Mon Jul 20 2020
 *
 * This file is a part of Skybase
 * Skybase (formerly known as TerrabaseDB) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2020, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

//! The core library for the Skybase
//!
//! This contains modules which are shared by both the `cli` and the `server` modules

pub mod terrapipe;
pub mod util;
use std::error::Error;
/// A generic result
pub type TResult<T> = Result<T, Box<dyn Error>>;
/// The size of the read buffer in bytes
pub const BUF_CAP: usize = 8 * 1024; // 8 KB per-connection
