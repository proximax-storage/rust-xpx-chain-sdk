/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

#[allow(deprecated)]
pub use self::metadata::*;
pub use self::metadata_entry::*;
pub use self::metadata_v2_type::*;
pub use self::search_metadata_entry::*;

#[deprecated(since = "0.1.2", note = "use `metadata_entry` instead")]
mod metadata;
mod metadata_entry;
mod metadata_v2_type;
mod search_metadata_entry;
