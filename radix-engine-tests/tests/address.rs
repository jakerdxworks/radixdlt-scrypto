use radix_engine::errors::{RuntimeError, SystemError};
use radix_engine::types::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn get_global_address_in_local_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "get_address_in_local",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemError(SystemError::GlobalAddressDoesNotExist)
        )
    });
}

#[test]
fn get_global_address_in_parent_should_succeed() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "MyComponent", "create", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_method(component, "get_address_in_parent", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let get_address_component: ComponentAddress = receipt.expect_commit(true).output(1);

    // Assert
    receipt.expect_commit_success();
    assert_eq!(component, get_address_component)
}

#[test]
fn get_global_address_in_child_should_succeed() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "MyComponent", "create", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_method(component, "get_address_in_child", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let get_address_component: ComponentAddress = receipt.expect_commit(true).output(1);

    // Assert
    receipt.expect_commit_success();
    assert_eq!(component, get_address_component)
}