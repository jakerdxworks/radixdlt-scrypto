use crate::api::types::*;
use crate::data::scrypto::model::Own;
use crate::data::scrypto::model::*;
use crate::schema::*;
use crate::*;
use sbor::rust::fmt;
use sbor::rust::fmt::{Debug, Formatter};
use sbor::rust::prelude::*;

pub const NATIVE_PACKAGE_CODE_ID: u8 = 0u8;
pub const RESOURCE_MANAGER_PACKAGE_CODE_ID: u8 = 1u8;
pub const IDENTITY_PACKAGE_CODE_ID: u8 = 2u8;
pub const EPOCH_MANAGER_PACKAGE_CODE_ID: u8 = 3u8;
pub const CLOCK_PACKAGE_CODE_ID: u8 = 4u8;
pub const ACCOUNT_PACKAGE_CODE_ID: u8 = 5u8;
pub const ACCESS_CONTROLLER_PACKAGE_CODE_ID: u8 = 6u8;
pub const TRANSACTION_RUNTIME_CODE_ID: u8 = 8u8;
pub const AUTH_ZONE_CODE_ID: u8 = 9u8;
pub const METADATA_CODE_ID: u8 = 10u8;
pub const ROYALTY_CODE_ID: u8 = 11u8;
pub const ACCESS_RULES_CODE_ID: u8 = 12u8;

/// A collection of blueprints, compiled and published as a single unit.
#[derive(Clone, Sbor, PartialEq, Eq)]
pub struct PackageCodeSubstate {
    pub code: Vec<u8>,
}

impl PackageCodeSubstate {
    pub fn code(&self) -> &[u8] {
        &self.code
    }
}

impl Debug for PackageCodeSubstate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackageCodeSubstate").finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct PackageInfoSubstate {
    pub schema: PackageSchema,
    pub dependent_resources: BTreeSet<ResourceAddress>,
    pub dependent_components: BTreeSet<ComponentAddress>,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct PackageRoyaltyConfigSubstate {
    pub royalty_config: BTreeMap<String, RoyaltyConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct PackageRoyaltyAccumulatorSubstate {
    /// The vault for collecting package royalties.
    ///
    /// It's optional to break circular dependency - creating package royalty vaults
    /// requires the `resource` package existing in the first place.
    pub royalty_vault: Option<Own>,
}