// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::env::{
    call::{
        utils::ReturnType,
        CallParams,
        CreateParams,
    },
    hash::{
        CryptoHash,
        HashOutput,
    },
    EnvTypes,
    Result,
    Topics,
};
use ink_primitives::Key;

/// The flags to indicate further information about the end of a contract execution.
pub struct ReturnFlags {
    value: u32,
}

impl Default for ReturnFlags {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl ReturnFlags {
    /// Sets the bit to indicate that the execution is going to be reverted.
    pub fn set_reverted(mut self, has_reverted: bool) -> Self {
        match has_reverted {
            true => self.value |= has_reverted as u32,
            false => self.value &= !has_reverted as u32,
        }
        self
    }

    /// Returns the underlying `u32` representation.
    pub(crate) fn into_u32(self) -> u32 {
        self.value
    }
}

/// Environmental contract functionality that does not require `EnvTypes`.
pub trait Env {
    /// Writes the value to the contract storage under the given key.
    fn set_contract_storage<V>(&mut self, key: &Key, value: &V)
    where
        V: scale::Encode;

    /// Returns the value stored under the given key in the contract's storage if any.
    ///
    /// # Errors
    ///
    /// - If the decoding of the typed value failed
    fn get_contract_storage<R>(&mut self, key: &Key) -> Result<Option<R>>
    where
        R: scale::Decode;

    /// Clears the contract's storage key entry.
    fn clear_contract_storage(&mut self, key: &Key);

    /// Returns the execution input to the executed contract and decodes it as `T`.
    ///
    /// # Note
    ///
    /// - The input is the 4-bytes selector followed by the arguments
    ///   of the called function in their SCALE encoded representation.
    /// - No prior interaction with the environment must take place before
    ///   calling this procedure.
    ///
    /// # Usage
    ///
    /// Normally contracts define their own `enum` dispatch types respective
    /// to their exported contructors and messages that implement `scale::Decode`
    /// according to the contructors or messages selectors and their arguments.
    /// These `enum` dispatch types are then given to this procedure as the `T`.
    ///
    /// When using ink! users do not have to construct those enum dispatch types
    /// themselves as they are normally generated by the ink! code generation
    /// automatically.
    ///
    /// # Errors
    ///
    /// If the given `T` cannot be properly decoded from the expected input.
    fn decode_input<T>(&mut self) -> Result<T>
    where
        T: scale::Decode;

    /// Returns the value back to the caller of the executed contract.
    ///
    /// # Note
    ///
    /// Calling this method will end contract execution immediately.
    /// It will return the given return value back to its caller.
    ///
    /// The `flags` parameter can be used to revert the state changes of the
    /// entire execution if necessary.
    fn return_value<R>(&mut self, flags: ReturnFlags, return_value: &R) -> !
    where
        R: scale::Encode;

    /// Prints the given contents to the console log.
    fn println(&mut self, content: &str);

    /// Conducts the crypto hash of the given input and stores the result in `output`.
    fn hash_bytes<H>(&mut self, input: &[u8], output: &mut <H as HashOutput>::Type)
    where
        H: CryptoHash;

    /// Conducts the crypto hash of the given encoded input and stores the result in `output`.
    fn hash_encoded<H, T>(&mut self, input: &T, output: &mut <H as HashOutput>::Type)
    where
        H: CryptoHash,
        T: scale::Encode;

    /// Calls the chain extension with the given ID and inputs.
    ///
    /// Returns the output of the chain extension of the specified type.
    ///
    /// # Errors
    ///
    /// - If the chain extension with the given ID does not exist.
    /// - If the inputs had an unexpected encoding.
    /// - If the output could not be properly decoded.
    /// - If some extension specific condition has not been met.
    #[cfg(feature = "ink-unstable-chain-extensions")]
    fn call_chain_extension<I, O>(&mut self, func_id: u32, input: &I) -> Result<O>
    where
        I: scale::Codec + 'static,
        O: scale::Codec + 'static;
}

/// Environmental contract functionality.
pub trait TypedEnv: Env {
    /// Returns the address of the caller of the executed contract.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::caller`]
    fn caller<T: EnvTypes>(&mut self) -> Result<T::AccountId>;

    /// Returns the transferred balance for the contract execution.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::transferred_balance`]
    fn transferred_balance<T: EnvTypes>(&mut self) -> Result<T::Balance>;

    /// Returns the price for the specified amount of gas.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::gas_price`]
    fn weight_to_fee<T: EnvTypes>(&mut self, gas: u64) -> Result<T::Balance>;

    /// Returns the amount of gas left for the contract execution.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::gas_left`]
    fn gas_left<T: EnvTypes>(&mut self) -> Result<T::Balance>;

    /// Returns the timestamp of the current block.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::block_timestamp`]
    fn block_timestamp<T: EnvTypes>(&mut self) -> Result<T::Timestamp>;

    /// Returns the address of the executed contract.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::account_id`]
    fn account_id<T: EnvTypes>(&mut self) -> Result<T::AccountId>;

    /// Returns the balance of the executed contract.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::balance`]
    fn balance<T: EnvTypes>(&mut self) -> Result<T::Balance>;

    /// Returns the current rent allowance for the executed contract.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::rent_allowance`]
    fn rent_allowance<T: EnvTypes>(&mut self) -> Result<T::Balance>;

    /// Returns the current block number.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::block_number`]
    fn block_number<T: EnvTypes>(&mut self) -> Result<T::BlockNumber>;

    /// Returns the minimum balance of the contracts chain.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::minimum_balance`]
    fn minimum_balance<T: EnvTypes>(&mut self) -> Result<T::Balance>;

    /// Returns the tombstone deposit of the contract chain.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::tombstone_deposit`]
    fn tombstone_deposit<T: EnvTypes>(&mut self) -> Result<T::Balance>;

    /// Emits an event with the given event data.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::emit_event`]
    fn emit_event<T, Event>(&mut self, event: Event)
    where
        T: EnvTypes,
        Event: Topics<T> + scale::Encode;

    /// Sets the rent allowance of the executed contract to the new value.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::set_rent_allowance`]
    fn set_rent_allowance<T>(&mut self, new_value: T::Balance)
    where
        T: EnvTypes;

    /// Invokes a contract message.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::invoke_contract`]
    fn invoke_contract<T, Args>(
        &mut self,
        call_data: &CallParams<T, Args, ()>,
    ) -> Result<()>
    where
        T: EnvTypes,
        Args: scale::Encode;

    /// Evaluates a contract message and returns its result.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::eval_contract`]
    fn eval_contract<T, Args, R>(
        &mut self,
        call_data: &CallParams<T, Args, ReturnType<R>>,
    ) -> Result<R>
    where
        T: EnvTypes,
        Args: scale::Encode,
        R: scale::Decode;

    /// Instantiates another contract.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::instantiate_contract`]
    fn instantiate_contract<T, Args, C>(
        &mut self,
        params: &CreateParams<T, Args, C>,
    ) -> Result<T::AccountId>
    where
        T: EnvTypes,
        Args: scale::Encode;

    /// Restores a smart contract tombstone.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::restore_contract`]
    fn restore_contract<T>(
        &mut self,
        account_id: T::AccountId,
        code_hash: T::Hash,
        rent_allowance: T::Balance,
        filtered_keys: &[Key],
    ) where
        T: EnvTypes;

    /// Terminates a smart contract.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::terminate_contract`]
    fn terminate_contract<T>(&mut self, beneficiary: T::AccountId) -> !
    where
        T: EnvTypes;

    /// Transfers value from the contract to the destination account ID.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::transfer`]
    fn transfer<T>(&mut self, destination: T::AccountId, value: T::Balance) -> Result<()>
    where
        T: EnvTypes;

    /// Returns a random hash seed.
    ///
    /// # Note
    ///
    /// For more details visit: [`ink_core::env::random`]
    fn random<T>(&mut self, subject: &[u8]) -> Result<T::Hash>
    where
        T: EnvTypes;
}
