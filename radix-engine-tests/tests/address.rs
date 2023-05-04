use radix_engine::errors::{RuntimeError, SystemError};
use radix_engine::types::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn get_global_address_in_local_in_function_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "get_global_address_in_local",
            manifest_args!(called_component),
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
fn get_global_address_in_local_in_method_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_method(
            component,
            "get_global_address_in_local_of_parent_method",
            manifest_args!(called_component),
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
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_method(component, "get_global_address_in_parent", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let get_global_address_component: ComponentAddress = receipt.expect_commit(true).output(1);

    // Assert
    receipt.expect_commit_success();
    assert_eq!(component, get_global_address_component)
}

#[test]
fn get_global_address_in_child_should_succeed() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_method(component, "get_global_address_in_owned", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let get_global_address_component: ComponentAddress = receipt.expect_commit(true).output(1);

    // Assert
    receipt.expect_commit_success();
    assert_eq!(component, get_global_address_component)
}

#[test]
fn call_component_address_protected_method_in_parent_should_succeed() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_method(
            component,
            "call_other_component_in_parent",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn call_component_address_protected_method_in_child_should_succeed() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_method(component, "call_other_component_in_child", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn call_component_address_protected_method_in_parent_with_wrong_address_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_method(
            component,
            "call_other_component_with_wrong_address",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemError(SystemError::AssertAccessRuleFailed)
        )
    });
}

#[test]
fn can_instantiate_with_preallocated_address() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "CalledComponent",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];
    // Act + Assert
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "MyComponent",
            "create_with_preallocated_address",
            manifest_args!(called_component),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
}

#[test]
fn errors_if_unused_preallocated_address() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");

    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(test_runner.faucet_component(), 10.into())
            .call_function(
                package_address,
                "CalledComponent",
                "create",
                manifest_args!(),
            )
            .build(),
        vec![],
    );
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act + Assert 1
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(test_runner.faucet_component(), 10.into())
            .call_function(
                package_address,
                "MyComponent",
                "create_with_unused_preallocated_address_1",
                manifest_args!(called_component),
            )
            .build(),
        vec![],
    );
    receipt.expect_commit_failure();

    // Act + Assert 2
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(test_runner.faucet_component(), 10.into())
            .call_function(
                package_address,
                "MyComponent",
                "create_with_unused_preallocated_address_2",
                manifest_args!(called_component),
            )
            .build(),
        vec![],
    );
    receipt.expect_commit_failure();
}

#[test]
fn errors_if_assigns_same_address_to_two_components() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/address");

    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(test_runner.faucet_component(), 10.into())
            .call_function(
                package_address,
                "CalledComponent",
                "create",
                manifest_args!(),
            )
            .build(),
        vec![],
    );
    let called_component = receipt.expect_commit(true).new_component_addresses()[0];

    // Act + Assert
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(test_runner.faucet_component(), 10.into())
            .call_function(
                package_address,
                "MyComponent",
                "create_two_with_same_address",
                manifest_args!(called_component),
            )
            .build(),
        vec![],
    );
    receipt.expect_commit_failure();
}
