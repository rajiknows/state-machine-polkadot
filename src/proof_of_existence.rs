use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    /// the type which represents the content can be claimed using this pallet
    /// could be the content as bytes or better yet as hashes
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            claims: BTreeMap::new(),
        }
    }

    fn get_chain(&mut self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }

        self.claims.insert(claim, caller);
        Ok(())
    }
    fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.claims.get(&claim).ok_or("claim does not exist")?;
        if *owner != caller {
            return Err("content owned by someone else");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
    // GetChain { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;
    //
    // fn dispatch(
    //     &mut self,
    //     caller: Self::Caller,
    //     call: Self::Call,
    // ) -> crate::support::DispatchResult {
    //     match call {
    //         Call::CreateClaim { claim } => {
    //             self.create_claim(caller, claim)?;
    //         }
    //         Call::RevokeClaim { claim } => {
    //             self.revoke_claim(caller, claim)?;
    //         }
    //     }
    //     Ok(())
    // }
    //
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim)?,
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim)?,
        }
        Ok(())
    }
}
