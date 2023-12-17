// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::sync::Arc;

use nostr::nips::nip19::{self, FromBech32, ToBech32};
use nostr::nips::nip21::NostrURI;
use uniffi::Object;

use crate::error::Result;
use crate::PublicKey;

#[derive(Object)]
pub struct Nip19Profile {
    inner: nip19::Nip19Profile,
}

impl From<nip19::Nip19Profile> for Nip19Profile {
    fn from(inner: nip19::Nip19Profile) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Nip19Profile {
    /// New NIP19 profile
    #[uniffi::constructor]
    pub fn new(public_key: Arc<PublicKey>, relays: Vec<String>) -> Self {
        Self {
            inner: nip19::Nip19Profile::new(**public_key, relays),
        }
    }

    #[uniffi::constructor]
    pub fn from_bech32(bech32: String) -> Result<Self> {
        Ok(Self {
            inner: nip19::Nip19Profile::from_bech32(bech32)?,
        })
    }

    #[uniffi::constructor]
    pub fn from_nostr_uri(uri: String) -> Result<Self> {
        Ok(Self {
            inner: nip19::Nip19Profile::from_nostr_uri(uri)?,
        })
    }

    pub fn to_bech32(&self) -> Result<String> {
        Ok(self.inner.to_bech32()?)
    }

    pub fn to_nostr_uri(&self) -> Result<String> {
        Ok(self.inner.to_nostr_uri()?)
    }

    pub fn public_key(&self) -> Arc<PublicKey> {
        Arc::new(self.inner.public_key.into())
    }

    pub fn relays(&self) -> Vec<String> {
        self.inner.relays.clone()
    }
}
