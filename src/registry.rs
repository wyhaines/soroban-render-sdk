//! Registry module for multi-contract applications.
//!
//! Provides a base registry implementation that contracts can use for
//! managing contract address aliases, enabling the `form:@alias:method`
//! and `tx:@alias:method` protocols in soroban-render.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use soroban_render_sdk::registry::{BaseRegistry, RegistryKey};
//! use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol};
//!
//! #[contract]
//! pub struct MyRegistry;
//!
//! #[contractimpl]
//! impl MyRegistry {
//!     pub fn init(env: Env, admin: Address, theme: Address, content: Address) {
//!         let mut contracts = Map::new(&env);
//!         contracts.set(symbol_short!("theme"), theme);
//!         contracts.set(symbol_short!("content"), content);
//!         BaseRegistry::init(&env, &admin, contracts);
//!     }
//!
//!     pub fn get_contract_by_alias(env: Env, alias: Symbol) -> Option<Address> {
//!         if alias == symbol_short!("registry") {
//!             return Some(env.current_contract_address());
//!         }
//!         BaseRegistry::get_by_alias(&env, alias)
//!     }
//! }
//! ```

use soroban_sdk::{contracttype, Address, Env, Map, Symbol};

/// Storage keys used by the base registry.
///
/// These keys are used in instance storage to store the contract
/// address mappings and admin address.
#[contracttype]
#[derive(Clone)]
pub enum RegistryKey {
    /// Map of alias Symbol -> contract Address
    Contracts,
    /// Admin address for registry management
    Admin,
}

/// Trait for contracts that serve as a registry for other contracts.
///
/// Implement this trait to provide a consistent interface for contract
/// discovery in multi-contract applications.
pub trait ContractRegistry {
    /// Register a contract under an alias.
    fn register_contract(env: &Env, alias: Symbol, address: Address);

    /// Get a contract address by its alias.
    fn get_contract_by_alias(env: &Env, alias: Symbol) -> Option<Address>;

    /// Get all registered contracts.
    fn get_all_contracts(env: &Env) -> Map<Symbol, Address>;
}

/// Default implementation for contract registry functionality.
///
/// This struct provides static methods that can be used by any contract
/// that wants to implement registry functionality. It handles storage
/// of contract aliases and admin management.
///
/// ## Storage
///
/// - `RegistryKey::Contracts` - Map of alias Symbol -> contract Address
/// - `RegistryKey::Admin` - Admin address with permission to modify registry
///
/// ## Example
///
/// ```rust,ignore
/// use soroban_render_sdk::registry::BaseRegistry;
/// use soroban_sdk::{symbol_short, Address, Env, Map};
///
/// // Initialize with admin and initial contracts
/// let mut contracts = Map::new(&env);
/// contracts.set(symbol_short!("theme"), theme_address);
/// contracts.set(symbol_short!("content"), content_address);
/// BaseRegistry::init(&env, &admin, contracts);
///
/// // Later, look up a contract
/// let theme = BaseRegistry::get_by_alias(&env, symbol_short!("theme"));
/// ```
pub struct BaseRegistry;

impl BaseRegistry {
    /// Initialize the registry with an admin and initial set of contracts.
    ///
    /// This can only be called once. Subsequent calls will panic.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `admin` - The admin address (must authorize this call)
    /// * `contracts` - Initial map of alias -> contract address
    ///
    /// # Panics
    ///
    /// Panics if the registry has already been initialized.
    pub fn init(env: &Env, admin: &Address, contracts: Map<Symbol, Address>) {
        if env.storage().instance().has(&RegistryKey::Admin) {
            panic!("Registry already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&RegistryKey::Admin, admin);
        env.storage().instance().set(&RegistryKey::Contracts, &contracts);
    }

    /// Register or update a contract alias.
    ///
    /// Only the admin can call this function.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `alias` - The alias Symbol (e.g., `symbol_short!("theme")`)
    /// * `address` - The contract address to register
    ///
    /// # Panics
    ///
    /// Panics if the registry has not been initialized.
    pub fn register(env: &Env, alias: Symbol, address: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&RegistryKey::Admin)
            .expect("Registry not initialized");
        admin.require_auth();

        let mut contracts: Map<Symbol, Address> = env
            .storage()
            .instance()
            .get(&RegistryKey::Contracts)
            .unwrap_or(Map::new(env));
        contracts.set(alias, address);
        env.storage().instance().set(&RegistryKey::Contracts, &contracts);
    }

    /// Look up a contract by its alias.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `alias` - The alias Symbol to look up
    ///
    /// # Returns
    ///
    /// `Some(Address)` if the alias is registered, `None` otherwise.
    pub fn get_by_alias(env: &Env, alias: Symbol) -> Option<Address> {
        let contracts: Map<Symbol, Address> = env.storage().instance().get(&RegistryKey::Contracts)?;
        contracts.get(alias)
    }

    /// Get all registered contracts.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    ///
    /// A Map of all alias -> address mappings, or an empty map if none registered.
    pub fn get_all(env: &Env) -> Map<Symbol, Address> {
        env.storage()
            .instance()
            .get(&RegistryKey::Contracts)
            .unwrap_or(Map::new(env))
    }

    /// Get the admin address.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    ///
    /// `Some(Address)` if initialized, `None` otherwise.
    pub fn get_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&RegistryKey::Admin)
    }

    /// Remove a contract alias.
    ///
    /// Only the admin can call this function.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `alias` - The alias Symbol to remove
    ///
    /// # Panics
    ///
    /// Panics if the registry has not been initialized.
    pub fn unregister(env: &Env, alias: Symbol) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&RegistryKey::Admin)
            .expect("Registry not initialized");
        admin.require_auth();

        let mut contracts: Map<Symbol, Address> = env
            .storage()
            .instance()
            .get(&RegistryKey::Contracts)
            .unwrap_or(Map::new(env));
        contracts.remove(alias);
        env.storage().instance().set(&RegistryKey::Contracts, &contracts);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{contract, contractimpl, symbol_short, testutils::Address as _, Env};

    // Minimal test contract that uses BaseRegistry
    #[contract]
    pub struct TestRegistry;

    #[contractimpl]
    impl TestRegistry {
        pub fn init(env: Env, admin: Address, contracts: Map<Symbol, Address>) {
            BaseRegistry::init(&env, &admin, contracts);
        }

        pub fn register(env: Env, alias: Symbol, address: Address) {
            BaseRegistry::register(&env, alias, address);
        }

        pub fn get_by_alias(env: Env, alias: Symbol) -> Option<Address> {
            BaseRegistry::get_by_alias(&env, alias)
        }

        pub fn get_all(env: Env) -> Map<Symbol, Address> {
            BaseRegistry::get_all(&env)
        }

        pub fn get_admin(env: Env) -> Option<Address> {
            BaseRegistry::get_admin(&env)
        }

        pub fn unregister(env: Env, alias: Symbol) {
            BaseRegistry::unregister(&env, alias);
        }
    }

    #[test]
    fn test_init_and_get() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(TestRegistry, ());
        let client = TestRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let theme = Address::generate(&env);
        let content = Address::generate(&env);

        let mut contracts = Map::new(&env);
        contracts.set(symbol_short!("theme"), theme.clone());
        contracts.set(symbol_short!("content"), content.clone());

        client.init(&admin, &contracts);

        // Verify we can look up contracts
        assert_eq!(client.get_by_alias(&symbol_short!("theme")), Some(theme));
        assert_eq!(
            client.get_by_alias(&symbol_short!("content")),
            Some(content)
        );
        assert_eq!(client.get_by_alias(&symbol_short!("unknown")), None);
    }

    #[test]
    fn test_register() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(TestRegistry, ());
        let client = TestRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let theme = Address::generate(&env);
        let new_contract = Address::generate(&env);

        let mut contracts = Map::new(&env);
        contracts.set(symbol_short!("theme"), theme);

        client.init(&admin, &contracts);

        // Register a new contract
        client.register(&symbol_short!("new"), &new_contract);

        assert_eq!(
            client.get_by_alias(&symbol_short!("new")),
            Some(new_contract)
        );
    }

    #[test]
    fn test_get_all() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(TestRegistry, ());
        let client = TestRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let theme = Address::generate(&env);
        let content = Address::generate(&env);

        let mut contracts = Map::new(&env);
        contracts.set(symbol_short!("theme"), theme);
        contracts.set(symbol_short!("content"), content);

        client.init(&admin, &contracts);

        let all = client.get_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_get_admin() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(TestRegistry, ());
        let client = TestRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let contracts = Map::new(&env);

        client.init(&admin, &contracts);

        assert_eq!(client.get_admin(), Some(admin));
    }

    #[test]
    fn test_unregister() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(TestRegistry, ());
        let client = TestRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let theme = Address::generate(&env);

        let mut contracts = Map::new(&env);
        contracts.set(symbol_short!("theme"), theme);

        client.init(&admin, &contracts);

        // Verify it exists
        assert!(client.get_by_alias(&symbol_short!("theme")).is_some());

        // Unregister
        client.unregister(&symbol_short!("theme"));

        // Verify it's gone
        assert!(client.get_by_alias(&symbol_short!("theme")).is_none());
    }

    #[test]
    #[should_panic(expected = "Registry already initialized")]
    fn test_double_init_panics() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(TestRegistry, ());
        let client = TestRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let contracts = Map::new(&env);

        client.init(&admin, &contracts);
        client.init(&admin, &contracts); // Should panic
    }
}
