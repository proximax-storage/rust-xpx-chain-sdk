// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

//! Common constants such as buffer sizes for keypairs and signatures.

/// The length of a ed25519 `Signature`, in bytes.
pub const SIGNATURE_LENGTH: usize = 64;

/// The length of a ed25519 `SecretKey`, in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

/// The length of an ed25519 `PublicKey`, in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// The length of an ed25519 `Keypair`, in bytes.
pub const KEYPAIR_LENGTH: usize = SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH;

/// The length of the "key" portion of an "expanded" ed25519 secret key, in bytes.
const EXPANDED_SECRET_KEY_KEY_LENGTH: usize = 32;

/// The length of the "nonce" portion of an "expanded" ed25519 secret key, in bytes.
const EXPANDED_SECRET_KEY_NONCE_LENGTH: usize = 32;

/// The length of an "expanded" ed25519 key, `ExpandedSecretKey`, in bytes.
pub const EXPANDED_SECRET_KEY_LENGTH: usize = EXPANDED_SECRET_KEY_KEY_LENGTH + EXPANDED_SECRET_KEY_NONCE_LENGTH;
