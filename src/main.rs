#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

use crate::erc721::{Erc721, Erc721Params};
use alloc::{string::String, vec::Vec};
use stylus_sdk::{alloy_primitives::U256, call, msg, prelude::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod erc721;

struct SampleERC721Params;

impl Erc721Params for SampleERC721Params {
    const NAME: &'static str = "Sample ERC721 Tokens";
    const SYMBOL: &'static str = "SMPL";
}

sol_storage! {
    #[entrypoint]
    struct SampleErc721 {
        #[borrow]
        Erc721<SampleERC721Params> erc721;
    }
}

#[external]
#[inherit(Erc721<SampleERC721Params>)]
impl SampleErc721 {
    
}