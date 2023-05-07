extern crate core;

use radix_engine::errors::{ModuleError, RuntimeError};
use radix_engine::transaction::TransactionReceipt;
use radix_engine::types::*;
use radix_engine_interface::blueprints::resource::{FromPublicKey, FUNGIBLE_VAULT_BLUEPRINT, ObjectKey, require};
use radix_engine_interface::blueprints::resource::AccessRule::DenyAll;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

enum ResourceAuth {
    Mint,
    Burn,
    Withdraw,
    Deposit,
    Recall,
    UpdateMetadata,
}

fn lock_resource_auth_and_try_update(action: ResourceAuth, lock: bool) -> TransactionReceipt {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let (token_address, _, _, _, _, _, admin_auth) = test_runner.create_restricted_token(account);
    let (_, updated_auth) = test_runner.create_restricted_burn_token(account);
    {
        let function = match action {
            ResourceAuth::Mint => "lock_mintable",
            ResourceAuth::Burn => "lock_burnable",
            ResourceAuth::Withdraw => "lock_withdrawable",
            ResourceAuth::Deposit => "lock_depositable",
            ResourceAuth::Recall => "lock_recallable",
            ResourceAuth::UpdateMetadata => "lock_metadata_updateable",
        };
        test_runner.lock_resource_auth(function, admin_auth, token_address, account, public_key);
    }

    // Act
    let mut builder = ManifestBuilder::new();
    builder
        .lock_fee(test_runner.faucet_component(), 100u32.into())
        .create_proof_from_account(account, admin_auth);

    let (object_key, group) = match action {
        ResourceAuth::Mint => (ObjectKey::SELF, "mint"),
        ResourceAuth::Burn => (ObjectKey::SELF, "burn"),
        ResourceAuth::UpdateMetadata => (ObjectKey::SELF, "update_metadata"),
        ResourceAuth::Withdraw => (ObjectKey::ChildBlueprint(FUNGIBLE_VAULT_BLUEPRINT.to_string()), "withdraw"),
        ResourceAuth::Deposit => (ObjectKey::ChildBlueprint(FUNGIBLE_VAULT_BLUEPRINT.to_string()), "deposit"),
        ResourceAuth::Recall => (ObjectKey::ChildBlueprint(FUNGIBLE_VAULT_BLUEPRINT.to_string()), "recall"),
    };

    let builder = if lock {
        builder.set_group_mutability(
            token_address.into(),
            object_key,
            group.to_string(),
            DenyAll,
        )
    } else {
        builder.set_group_access_rule(
            token_address.into(),
            object_key,
            group.to_string(),
            rule!(require(updated_auth)),
        )
    };

    let manifest = builder
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    receipt
}

#[test]
fn locked_mint_auth_cannot_be_updated() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Mint, false);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_mint_auth_cannot_be_relocked() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Mint, true);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_burn_auth_cannot_be_updated() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Burn, false);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_burn_auth_cannot_be_relocked() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Burn, true);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_withdraw_auth_cannot_be_updated() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Withdraw, false);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_withdraw_auth_cannot_be_relocked() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Withdraw, true);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_deposit_auth_cannot_be_updated() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Deposit, false);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_deposit_auth_cannot_be_relocked() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Deposit, true);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_recall_auth_cannot_be_updated() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Recall, false);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_recall_auth_cannot_be_relocked() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::Recall, true);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_update_metadata_auth_cannot_be_updated() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::UpdateMetadata, false);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}

#[test]
fn locked_update_metadata_auth_cannot_be_relocked() {
    let receipt = lock_resource_auth_and_try_update(ResourceAuth::UpdateMetadata, true);
    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    })
}
