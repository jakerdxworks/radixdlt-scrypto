use super::authorization::{
    HardAuthRule, HardCount, HardDecimal, HardProofRule, HardProofRuleResourceList,
    HardResourceOrNonFungible, MethodAuthorization,
};
use crate::types::*;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::schema::BlueprintSchema;
use radix_engine_interface::types::*;
use sbor::basic_well_known_types::UNIT_ID;

fn soft_to_hard_decimal(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    soft_decimal: &SoftDecimal,
    value: &IndexedScryptoValue,
) -> HardDecimal {
    match soft_decimal {
        SoftDecimal::Static(amount) => HardDecimal::Amount(amount.clone()),
        SoftDecimal::Dynamic(schema_path) => {
            if let Some((sbor_path, _)) = schema_path.to_sbor_path(schema, type_index) {
                let root = value.as_scrypto_value();
                let value = sbor_path
                    .get_from_value(&root)
                    .expect(format!("Value missing at {:?}", schema_path).as_str());

                if let Ok(amount) = scrypto_decode::<Decimal>(&scrypto_encode(value).unwrap()) {
                    HardDecimal::Amount(amount)
                } else {
                    HardDecimal::NotDecimal
                }
            } else {
                HardDecimal::InvalidPath
            }
        }
    }
}

fn soft_to_hard_count(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    soft_count: &SoftCount,
    value: &IndexedScryptoValue,
) -> HardCount {
    match soft_count {
        SoftCount::Static(count) => HardCount::Count(count.clone()),
        SoftCount::Dynamic(schema_path) => {
            if let Some((sbor_path, _)) = schema_path.to_sbor_path(schema, type_index) {
                let root = value.as_scrypto_value();
                let value = sbor_path
                    .get_from_value(&root)
                    .expect(format!("Value missing at {:?}", schema_path).as_str());

                if let Ok(n) = scrypto_decode::<u8>(&scrypto_encode(value).unwrap()) {
                    HardCount::Count(n)
                } else {
                    HardCount::NotU8
                }
            } else {
                HardCount::InvalidPath
            }
        }
    }
}

fn soft_to_hard_resource_list(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    list: &SoftResourceOrNonFungibleList,
    value: &IndexedScryptoValue,
) -> HardProofRuleResourceList {
    match list {
        SoftResourceOrNonFungibleList::Static(resources) => {
            let mut hard_resources = Vec::new();
            for soft_resource in resources {
                let resource =
                    soft_to_hard_resource_or_non_fungible(schema, type_index, soft_resource, value);
                hard_resources.push(resource);
            }
            HardProofRuleResourceList::List(hard_resources)
        }
        SoftResourceOrNonFungibleList::Dynamic(schema_path) => {
            if let Some((sbor_path, _)) = schema_path.to_sbor_path(schema, type_index) {
                let root = value.as_scrypto_value();
                let value = sbor_path
                    .get_from_value(&root)
                    .expect(format!("Value missing at {:?}", schema_path).as_str());

                if let Ok(v) =
                    scrypto_decode::<Vec<ResourceAddress>>(&scrypto_encode(value).unwrap())
                {
                    HardProofRuleResourceList::List(
                        v.into_iter()
                            .map(|e| HardResourceOrNonFungible::Resource(e))
                            .collect(),
                    )
                } else if let Ok(v) =
                    scrypto_decode::<Vec<NonFungibleGlobalId>>(&scrypto_encode(value).unwrap())
                {
                    HardProofRuleResourceList::List(
                        v.into_iter()
                            .map(|e| HardResourceOrNonFungible::NonFungible(e))
                            .collect(),
                    )
                } else {
                    HardProofRuleResourceList::NotResourceAddressOrNonFungibleGlobalIdArray
                }
            } else {
                HardProofRuleResourceList::InvalidPath
            }
        }
    }
}

fn soft_to_hard_resource(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    soft_resource: &SoftResource,
    value: &IndexedScryptoValue,
) -> HardResourceOrNonFungible {
    match soft_resource {
        SoftResource::Dynamic(schema_path) => {
            if let Some((sbor_path, _)) = schema_path.to_sbor_path(schema, type_index) {
                let root = value.as_scrypto_value();
                let value = sbor_path
                    .get_from_value(&root)
                    .expect(format!("Value missing at {:?}", schema_path).as_str());

                if let Ok(address) =
                    scrypto_decode::<ResourceAddress>(&scrypto_encode(value).unwrap())
                {
                    HardResourceOrNonFungible::Resource(address)
                } else {
                    HardResourceOrNonFungible::NotResourceAddress
                }
            } else {
                HardResourceOrNonFungible::InvalidPath
            }
        }
        SoftResource::Static(resource) => HardResourceOrNonFungible::Resource(resource.clone()),
    }
}

fn soft_to_hard_resource_or_non_fungible(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    soft_resource_or_non_fungible: &SoftResourceOrNonFungible,
    value: &IndexedScryptoValue,
) -> HardResourceOrNonFungible {
    match soft_resource_or_non_fungible {
        SoftResourceOrNonFungible::Dynamic(schema_path) => {
            if let Some((sbor_path, _)) = schema_path.to_sbor_path(schema, type_index) {
                let root = value.as_scrypto_value();
                let value = sbor_path
                    .get_from_value(&root)
                    .expect(format!("Value missing at {:?}", schema_path).as_str());

                if let Ok(address) =
                    scrypto_decode::<ResourceAddress>(&scrypto_encode(value).unwrap())
                {
                    HardResourceOrNonFungible::Resource(address)
                } else if let Ok(non_fungible) =
                    scrypto_decode::<NonFungibleGlobalId>(&scrypto_encode(value).unwrap())
                {
                    HardResourceOrNonFungible::NonFungible(non_fungible)
                } else {
                    HardResourceOrNonFungible::NotResourceAddressOrNonFungibleGlobalId
                }
            } else {
                HardResourceOrNonFungible::InvalidPath
            }
        }
        SoftResourceOrNonFungible::StaticNonFungible(non_fungible_global_id) => {
            HardResourceOrNonFungible::NonFungible(non_fungible_global_id.clone())
        }
        SoftResourceOrNonFungible::StaticResource(resource_def_id) => {
            HardResourceOrNonFungible::Resource(resource_def_id.clone())
        }
    }
}

fn soft_to_hard_proof_rule(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    proof_rule: &ProofRule,
    value: &IndexedScryptoValue,
) -> HardProofRule {
    match proof_rule {
        ProofRule::Require(resource_or_non_fungible) => {
            let resource = soft_to_hard_resource_or_non_fungible(
                schema,
                type_index,
                resource_or_non_fungible,
                value,
            );
            HardProofRule::Require(resource)
        }
        ProofRule::AmountOf(soft_decimal, resource) => {
            let resource = soft_to_hard_resource(schema, type_index, resource, value);
            let hard_decimal = soft_to_hard_decimal(schema, type_index, soft_decimal, value);
            HardProofRule::AmountOf(hard_decimal, resource)
        }
        ProofRule::AllOf(resources) => {
            let hard_resources = soft_to_hard_resource_list(schema, type_index, resources, value);
            HardProofRule::AllOf(hard_resources)
        }
        ProofRule::AnyOf(resources) => {
            let hard_resources = soft_to_hard_resource_list(schema, type_index, resources, value);
            HardProofRule::AnyOf(hard_resources)
        }
        ProofRule::CountOf(soft_count, resources) => {
            let hard_count = soft_to_hard_count(schema, type_index, soft_count, value);
            let hard_resources = soft_to_hard_resource_list(schema, type_index, resources, value);
            HardProofRule::CountOf(hard_count, hard_resources)
        }
    }
}

fn soft_to_hard_auth_rule(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    auth_rule: &AccessRuleNode,
    value: &IndexedScryptoValue,
) -> HardAuthRule {
    match auth_rule {
        AccessRuleNode::ProofRule(proof_rule) => HardAuthRule::ProofRule(soft_to_hard_proof_rule(
            schema, type_index, proof_rule, value,
        )),
        AccessRuleNode::AnyOf(rules) => {
            let hard_rules = rules
                .iter()
                .map(|r| soft_to_hard_auth_rule(schema, type_index, r, value))
                .collect();
            HardAuthRule::AnyOf(hard_rules)
        }
        AccessRuleNode::AllOf(rules) => {
            let hard_rules = rules
                .iter()
                .map(|r| soft_to_hard_auth_rule(schema, type_index, r, value))
                .collect();
            HardAuthRule::AllOf(hard_rules)
        }
    }
}

/// Converts an `AccessRule` into a `MethodAuthorization`, with the given context of
/// Scrypto value and schema.
///
/// This method assumes that the value matches with the schema.
pub fn convert(
    schema: &ScryptoSchema,
    type_index: LocalTypeIndex,
    value: &IndexedScryptoValue,
    method_auth: &AccessRule,
) -> MethodAuthorization {
    match method_auth {
        AccessRule::Protected(auth_rule) => MethodAuthorization::Protected(soft_to_hard_auth_rule(
            schema, type_index, auth_rule, value,
        )),
        AccessRule::AllowAll => MethodAuthorization::AllowAll,
        AccessRule::DenyAll => MethodAuthorization::DenyAll,
    }
}

pub fn convert_contextless(method_auth: &AccessRule) -> MethodAuthorization {
    convert(
        &BlueprintSchema::default().schema,
        LocalTypeIndex::WellKnown(UNIT_ID),
        &IndexedScryptoValue::unit(),
        method_auth,
    )
}
