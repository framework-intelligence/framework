#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod nexus {

    use ink::storage::Mapping;
    use poll_market::PollMarketRef;
    use ink::primitives::AccountId;

    #[ink(storage)]
    pub struct Nexus {
        poll_market: PollMarketRef,
        // index_to_job: Mapping<AccountId, i64>,
        // user_to_amount_funded: Mapping<AccountId, u32>,
        // min_stake: u8
    }

    impl Nexus {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                poll_index: 0,
                // index_to_job: Mapping::default()
            }
        }

        #[ink(message)]
        pub fn add_job(&mut self) {
            let caller = Self::env().caller();

            self.index_to_job.insert(&caller, &self.job_index);
            self.job_index += 1;
        }

        // #[ink(message)]
        // pub fn get_job_num(self) -> i64 {
        //     self.job_index
        // }
    }
}