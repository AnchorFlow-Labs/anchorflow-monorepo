#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
pub struct Flow {
    pub id: u64,
    pub owner: Address,
    pub name: String,
}

#[contract]
pub struct AnchorFlowContract;

#[contractimpl]
impl AnchorFlowContract {
    /// Create a new flow entry owned by `owner`.
    pub fn create_flow(env: Env, owner: Address, name: String) -> u64 {
        owner.require_auth();
        let id: u64 = env.storage().instance().get(&"count").unwrap_or(0) + 1;
        env.storage().instance().set(&"count", &id);
        let flow = Flow { id, owner, name };
        let mut flows: Vec<Flow> = env.storage().instance().get(&"flows").unwrap_or(Vec::new(&env));
        flows.push_back(flow);
        env.storage().instance().set(&"flows", &flows);
        id
    }

    /// Return all flows.
    pub fn get_flows(env: Env) -> Vec<Flow> {
        env.storage().instance().get(&"flows").unwrap_or(Vec::new(&env))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_create_and_get_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorFlowContract);
        let client = AnchorFlowContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let id = client.create_flow(&owner, &String::from_str(&env, "My Flow"));
        assert_eq!(id, 1);

        let flows = client.get_flows();
        assert_eq!(flows.len(), 1);
    }
}
