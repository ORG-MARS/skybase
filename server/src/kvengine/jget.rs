/*
 * Created on Mon Aug 31 2020
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
#![allow(dead_code)]

//! #`JGET` queries
//! Functions for handling `JGET` queries

use crate::coredb::CoreDB;
use crate::dbnet::Con;
use crate::protocol::{responses, ActionGroup};
use libsky::TResult;

/// Run a `JGET` query
/// This returns a JSON key/value pair of keys and values
/// We need to write something like
/// ```json
/// &1\n
/// $15\n
/// {"key":"value"}\n
/// ```
///
pub async fn jget(_handle: &CoreDB, con: &mut Con<'_>, act: ActionGroup) -> TResult<()> {
    let howmany = act.howmany();
    if howmany != 1 {
        return con.write_response(&**responses::fresp::R_ACTION_ERR).await;
    }
    todo!()
}

mod json {
    use bytes::Bytes;
    use std::hint::unreachable_unchecked;

    pub struct BuiltJSON(Vec<u8>);
    pub struct JSONBlob(Vec<u8>);
    impl JSONBlob {
        pub fn new(size: usize) -> Self {
            let mut jblob = Vec::with_capacity(1 + size);
            jblob.push(b'{');
            JSONBlob(jblob)
        }
        pub fn insert(&mut self, key: &String, value: Option<&Bytes>) {
            self.0.push(b'"');
            self.0.extend(key.as_bytes());
            self.0.extend(b"\":");
            if let Some(value) = value {
                self.0.push(b'"');
                self.0.extend(value);
                self.0.push(b'"');
            } else {
                self.0.extend(b"null");
            }
            self.0.push(b',');
        }
        pub fn finish(mut self) -> BuiltJSON {
            *self
                .0
                .last_mut()
                .unwrap_or_else(|| unsafe { 
                    // UNSAFE(@ohsayan): There will always be a value corresponding to last_mut
                    unreachable_unchecked() 
                }) = b'}';
            BuiltJSON(self.0)
        }
    }
    #[test]
    fn test_buildjson() {
        let mut jblob = JSONBlob::new(128);
        jblob.insert(&"key".to_owned(), Some(&Bytes::from("value".as_bytes())));
        jblob.insert(&"key2".to_owned(), None);
        assert_eq!(
            "{\"key\":\"value\",\"key2\":null}",
            String::from_utf8_lossy(&jblob.finish().0)
        );
    }
}
