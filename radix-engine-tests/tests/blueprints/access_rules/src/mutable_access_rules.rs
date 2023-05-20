use scrypto::prelude::*;

#[blueprint]
mod mutable_access_rules_component {
    struct MutableAccessRulesComponent {}

    impl MutableAccessRulesComponent {
        pub fn new(authority_rules: AuthorityRules) -> Global<MutableAccessRulesComponent> {
            Self {}
                .instantiate()
                .prepare_to_globalize()
                .define_roles(authority_rules)
                .protect_methods(btreemap!(
                    "borrow_funds" => vec!["borrow_funds_auth".to_string()],
                    "deposit_funds" => vec!["deposit_funds_auth".to_string()],
                ))
                .globalize()
        }

        pub fn access_rules_function(component_address: ComponentAddress) {
            let component: Global<AnyComponent> = component_address.into();
            let _access_rules = component.access_rules();
        }

        pub fn set_authority_rules(&self, authority: String, rule: AccessRule) {
            let access_rules = Runtime::access_rules();
            access_rules.set_authority_rule(authority.as_str(), rule);
        }

        pub fn lock_authority(&self, authority: String) {
            let access_rules = Runtime::access_rules();
            access_rules.set_authority_mutability(authority.as_str(), AccessRule::DenyAll);
        }

        // The methods that the access rules will be added to
        pub fn borrow_funds(&self) {}

        pub fn deposit_funds(&self) {}
    }
}
