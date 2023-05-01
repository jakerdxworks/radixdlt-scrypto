# Native Blueprints

This folder contains Native Blueprints which are special application layer logic compiled 
with the radix engine in order to get some speed benefits as well as some additional features
which are not exposed to WASM Blueprints.


| Blueprint                    | Package                | Description                                                                                                        |
|------------------------------|------------------------|--------------------------------------------------------------------------------------------------------------------|
| AccessController             | AccessController       | Provides a State Machine for accessing/recovering a resource.                                                      |
| Account                      | Account                | Aggregates resources by ResourceAddress under one component. This is what wallets will mostly interact with.       |
| Clock                        | Clock                  | Provides a Time API                                                                                                |
| EpochManager                 | EpochManager           | Keeps track of the current round/epoch as well as maintaining the current validator set.                           | 
| Validator                    | EpochManager           | Represents and manages a validator in the Consensus layer. Allows users to delegate stake to itself.               |
| Identity                     | Identity               | Provides a clean component in which the owner is clear                                                             |
| Package                      | Package                | Maintains VM code as well as Blueprint definitions                                                                 |
| Fungible Resource Manager    | Resource               | Manages a fungible resource, or a resource which may be broken up. For example, it keeps track of the total supply |
| Fungible Vault               | Resource               | Holds a persisted amount of a fungible resource                                                                    |
| Fungible Bucket              | Resource               | Holds a transient (runtime) amount of a fungible resource                                                          |
| Fungible Proof               | Resource               | Represents proof that a given fungible resource exists at a moment in time.                                        |
| NonFungible Resource Manager | Resource               | Manages a non-fungible resource, or a resource in which each token has a unique id.                                |
| NonFungible Vault            | Resource               | Holds a persisted amount of a non-fungible resource                                                                |
| NonFungible Bucket           | Resource               | Holds a transient (runtime) amount of a non-fungible resource                                                      |
| NonFungible Proof            | Resource               | Represents proof that a given non-fungible resource exists at a moment in time.                                    |
