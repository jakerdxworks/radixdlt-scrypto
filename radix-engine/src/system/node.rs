use crate::blueprints::package::PackageCodeTypeSubstate;
use crate::system::node_modules::access_rules::*;
use crate::system::node_modules::type_info::TypeInfoSubstate;
use crate::system::node_substates::*;
use crate::types::*;
use radix_engine_interface::api::component::*;
use radix_engine_interface::api::types::{AuthZoneStackOffset, PackageOffset, SubstateOffset};
use radix_engine_interface::blueprints::package::*;

use super::node_modules::event_schema::PackageEventSchemaSubstate;

#[derive(Debug)]
pub enum RENodeModuleInit {
    /* Type info */
    TypeInfo(TypeInfoSubstate),

    /* Metadata */
    Metadata(BTreeMap<SubstateOffset, RuntimeSubstate>),

    /* Access rules */
    MethodAccessRules(MethodAccessRulesSubstate),

    /* Royalty */
    ComponentRoyalty(
        ComponentRoyaltyConfigSubstate,
        ComponentRoyaltyAccumulatorSubstate,
    ),
}

impl RENodeModuleInit {
    pub fn to_substates(self) -> HashMap<SubstateOffset, RuntimeSubstate> {
        let mut substates = HashMap::<SubstateOffset, RuntimeSubstate>::new();
        match self {
            RENodeModuleInit::Metadata(metadata_substates) => {
                substates.extend(metadata_substates);
            }
            RENodeModuleInit::MethodAccessRules(access_rules) => {
                substates.insert(
                    SubstateOffset::AccessRules(AccessRulesOffset::AccessRules),
                    access_rules.into(),
                );
            }
            RENodeModuleInit::TypeInfo(type_info) => {
                substates.insert(
                    SubstateOffset::TypeInfo(TypeInfoOffset::TypeInfo),
                    type_info.into(),
                );
            }
            RENodeModuleInit::ComponentRoyalty(config, accumulator) => {
                substates.insert(
                    SubstateOffset::Royalty(RoyaltyOffset::RoyaltyConfig),
                    config.into(),
                );
                substates.insert(
                    SubstateOffset::Royalty(RoyaltyOffset::RoyaltyAccumulator),
                    accumulator.into(),
                );
            }
        }

        substates
    }
}

#[derive(Debug)]
pub enum RENodeInit {
    GlobalObject(BTreeMap<SubstateOffset, RuntimeSubstate>),
    Object(BTreeMap<SubstateOffset, RuntimeSubstate>),
    PackageObject(
        PackageInfoSubstate,
        PackageCodeTypeSubstate,
        PackageCodeSubstate,
        PackageRoyaltySubstate,
        FunctionAccessRulesSubstate,
        PackageEventSchemaSubstate,
    ),
    AuthZoneStack(AuthZoneStackSubstate),
    KeyValueStore,
    NonFungibleStore,
}

impl RENodeInit {
    pub fn to_substates(self) -> HashMap<SubstateOffset, RuntimeSubstate> {
        let mut substates = HashMap::<SubstateOffset, RuntimeSubstate>::new();
        match self {
            RENodeInit::AuthZoneStack(auth_zone) => {
                substates.insert(
                    SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
                    RuntimeSubstate::AuthZoneStack(auth_zone),
                );
            }
            RENodeInit::GlobalObject(object_substates) | RENodeInit::Object(object_substates) => {
                substates.extend(object_substates);
            }
            RENodeInit::KeyValueStore | RENodeInit::NonFungibleStore => {}
            RENodeInit::PackageObject(
                package_info,
                code_type,
                code,
                royalty,
                function_access_rules,
                event_schema,
            ) => {
                substates.insert(
                    SubstateOffset::Package(PackageOffset::Info),
                    package_info.into(),
                );
                substates.insert(
                    SubstateOffset::Package(PackageOffset::CodeType),
                    code_type.into(),
                );
                substates.insert(SubstateOffset::Package(PackageOffset::Code), code.into());
                substates.insert(
                    SubstateOffset::Package(PackageOffset::Royalty),
                    royalty.into(),
                );
                substates.insert(
                    SubstateOffset::Package(PackageOffset::FunctionAccessRules),
                    function_access_rules.into(),
                );
                substates.insert(
                    SubstateOffset::Package(PackageOffset::EventSchema),
                    event_schema.into(),
                );
            }
        };

        substates
    }
}
