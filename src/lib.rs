/*
 * This file is part of PROJECT.
 *
 * PROJECT is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * PROJECT is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with PROJECT.  If not, see <https://www.gnu.org/licenses/>.
 */

use encoding_rs::SHIFT_JIS;

pub fn decode(binary: Vec<u8>) -> String {
    let (decoded, _, _) = SHIFT_JIS.decode(&binary);
    decoded.to_string()
}

pub fn encode(text: &str) -> Vec<u8> {
    let (encoded, _, _) = SHIFT_JIS.encode(&text);
    encoded.to_vec()
}

pub fn is_sjis(binary: &Vec<u8>) -> bool {
    let t = String::from_utf8(binary.to_vec());
    match t {
        Ok(_) => false,
        Err(_) => true,
    }
}

