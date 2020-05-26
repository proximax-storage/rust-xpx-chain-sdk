// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use regex::Regex;

pub fn is_hex(input: &str) -> bool {
    if input == "" {
        return false;
    }

    let re = Regex::new(r"^[a-fA-F0-9]+$").unwrap();

    re.is_match(input)
}
