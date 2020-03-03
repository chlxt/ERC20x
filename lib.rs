#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

/// [EIP-20 Etherum standard](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md)
/// [EIP-20](https://en.bitcoinwiki.org/wiki/ERC20)
/// [ERC-20 Token standard](https://github.com/ethereum/EIPs/issues/20)
/// [ERC-20 Attack Vector on approve/tansferFrom methods](https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729)
#[ink::contract(version = "0.1.0")]
mod erc20x {
    use ink_core::storage;


    //pub const name: &[u8] = b"DXT";
    //pub const symbol: &[u8] = b"DXT";
    pub const decimals: u8 = 18;


    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        operator: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        #[ink(topic)]
        old_value: Balance,
        #[ink(topic)]
        value: Balance,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct ERC20x {
        total_supply: storage::Value<Balance>, // The total supply.
        balances: storage::HashMap<AccountId, Balance>, // The balance of each user.
        allowances: storage::HashMap<(AccountId, AccountId), Balance>, // Balances that are spendable by non-owners: (owner, spender) -> allowed
    }


    impl ERC20x {
        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                operator: Some(caller),
                value: initial_supply,
            });
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_of_or_zero(&owner, &spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            //let from = self.env().caller();
            self.transfer_from_to_safely(self.env().caller(), to, value)
        }

        /// Approve the passed AccountId to spend the specified amount of tokens
        /// on the behalf of the message's sender.
        #[ink(message)] 
        fn approve(&mut self, spender: AccountId, old_value: Balance, value: Balance) -> bool {
            let owner = self.env().caller();
            let old_value_onchain = self.allowances.insert((owner, spender), value).unwrap_or(0);
            if old_value_onchain != old_value {
                return false;
            }
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                old_value: old_value,
                value: value,
            });
            true
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            let allowance = self.allowance_of_or_zero(&from, &caller);
            if allowance < value {
                return false;
            }
            self.allowances.insert((from, caller), allowance - value);
            self.transfer_from_to_safely(from, to, value)
        }


        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) {
            self.balances.insert(from, self.balances.get(&from).unwrap() - value);
            self.balances.insert(to, self.balances.get(&to).unwrap() + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                operator: Some(from),
                value,
            });
        }

        fn transfer_from_to_safely(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false;
            }
            if !self.balances.contains_key(&to) {
                self.balances.insert(to, 0);
                //self.balances.entry(to).or_insert(0);
            }
            self.transfer_from_to(from, to, value);
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_works() {
            let contract = ERC20x::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[test]
        fn balance_works() {
            let contract = ERC20x::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[test]
        fn transfer_works() {
            let mut contract = ERC20x::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }

        #[test]
        fn transfer_from_works() {
            let mut contract = ERC20x::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 0, 20);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
        }
    }
}
