use alloc::{string::String, vec::Vec};
use alloy_primitives::Bytes;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::{sol, SolError},
    evm, msg,
    prelude::*
};
use core::marker::PhantomData;

pub trait Erc721Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
}

sol_storage! {
    pub struct Erc721<T> {
        mapping(address => uint256) balances;
        mapping(uint256 => address) owners;
        mapping(uint256 => address) token_approvals;
        mapping(address => mapping(address => bool)) operator_approvals;
        PhantomData<T> phantom; // Will add zeroes and of storage. So we will use these empty slots for Erc721params
    }
}

sol! { // events & errors directly inherited from OZ impl
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);

    error ERC721InvalidOwner(address owner);
    error ERC721NonexistentToken(uint256 tokenId);
    error ERC721IncorrectOwner(address sender, uint256 tokenId, address owner);
    error ERC721InvalidSender(address sender);
    error ERC721InvalidReceiver(address receiver);
    error ERC721InsufficientApproval(address operator, uint256 tokenId);
    error ERC721InvalidApprover(address approver);
    error ERC721InvalidOperator(address operator);

}

pub enum Erc721Error {
        ERC721InvalidOwner(ERC721InvalidOwner),
        ERC721NonexistentToken(ERC721NonexistentToken),
}

// Internal methods
impl<T: Erc721Params> Erc721<T> {
    pub fn _ownerOf(&self, tokenId: U256) -> Result<U256, Erc721Error> {
        let owners = self.owners.setter(tokenId);

        Ok(owners.get())
    }

    pub fn _getApproved(&self, tokenId: U256) -> Result<Address, Erc721Error> {
        let approvals = self.tokenApprovals.setter(tokenId);

        Ok(approvals.get())
    }

    pub fn _requireMinted(&self, tokenId: U256) -> Result<(), Erc721Error> {
        if(self._ownerOf(tokenId).into() == 0) {
            return Err(Erc721Error::ERC721NonexistentToken(ERC721NonexistentToken {
                tokenId
            }));
        }

        Ok(())
    }

    pub fn _baseURI(&self) -> Result<String, Erc721Error> {
        Ok(String::from(""))
    }

    pub fn _approve(&self, to: Address, tokenId: U256) -> Result<(), Erc721Error> {
        let owner = self._ownerOf(tokenId);
        if(owner.into() == 0) {
            return Err(Erc721Error::ERC721NonexistentToken(ERC721NonexistentToken {
                tokenId
            }));
        }

        if(owner == msg::sender()) {
            self.token_approvals.setter(tokenId).set(to);
        }

        evm::log(Approval { owner: msg::sender(), approved: to, tokenId } );
        Ok(())
    }

}

#[external]
impl<T: Erc721Params> Erc721<T> {
    pub fn balanceOf(&self, address: Address) -> Result<U256, Erc721Error> {
        if(address.into() == 0) {
            return Err(ERC721Error::ERC721InvalidOwner(ERC721InvalidOwner {
                owner: address
            }));
        }
        let balances = self.balances.setter(address);
        Ok(balances.get())
    }

    pub fn ownerOf(&self, tokenId: U256) -> Result<Address, ERC721Error> {
        let owner = self._ownerOf(tokenId)?;
        if(owner.into() == 0) {
            return Err(Erc721Error::ERC721NonexistentToken(ERC721NonexistentToken {
                tokenId
            }));
        }
        Ok(owner)
    }

    pub fn name() -> Result<String, Erc721Error> {
        Ok(T::NAME::into())
    }

    pub fn symbol() -> Result<String, Erc20Error> {
        Ok(T::SYMBOL.into())
    }

    pub fn tokenURI(&self, tokenId: U256) -> Result<String, ERC721Error> {
        self._requireMinted(tokenId)?;

        let base_uri = self._baseURI()?;
        let token_uri = base_uri.push_str(tokenId.into());
        
        Ok(token_uri)
    }

    pub fn approve(&self, to: Address, tokenId: U256) -> Result<(), Erc721Error> {
        self._approve(to, tokenId);
        Ok(())
    }

    pub fn getApproved(&self, tokenId: U256) -> Result<Address, Erc721Error> {
        self._requireMinted(tokenId)?;

        let approver = self._getApproved(tokenId)?;

        Ok(approver)
    }

    pub fn setApprovalForAll(&self, owner: Address, approved: bool) -> Result<(), Erc721Error> {

    }

    pub fn isApprovedForAll(&self, owner: Address, operator: Address) -> Result<bool, Erc721Error> {

    }

    pub fn transferFrom(&self, from: Address, to: Address, tokenId: U256) -> Result<(), Erc721Error> {

    }

    pub fn safeTransferFrom(&self, from: Address, to: Address, tokenId: U256) -> Result<(), Erc721Error> {

    }

    pub fn safeTransferFrom(&self, from: Address, to: Address, tokenId: U256, data: Bytes) -> Result<(), Erc721Error> {

    }

    
}