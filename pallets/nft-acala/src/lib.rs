#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_support::sp_runtime::DispatchError;
use frame_system::ensure_signed;
use log::info;
use sp_std::vec::Vec;
//use sp_runtime::{ traits::StaticLookup, DispatchResult };

pub trait Trait: frame_system::Trait  + orml_nft::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}
// FIXME:

decl_storage! {
	trait Store for Module<T: Trait> as NftAcala {
        NftAcalaCID get(fn ntf_token_cid): Vec<u8>;
		NftAcalaClass get(fn ntf_token_class): T::ClassId;
	}
}
// FIXME:
decl_event!(
	pub enum Event<T> where
        AccountId = <T as frame_system::Trait>::AccountId, 
		ClassId = <T as orml_nft::Trait>::ClassId,
		TokenId = <T as orml_nft::Trait>::TokenId
	{
        CreatedToken(AccountId, ClassId),
		MintedToken(AccountId, TokenId),
		BurnedToken(AccountId, ClassId, TokenId),
		TransferredToken(AccountId, AccountId, ClassId, TokenId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		pub fn create(origin, metadata: Vec<u8>, data: <T as orml_nft::Trait>::ClassData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			info!("create :: Start creating NFT");
			<NftAcalaCID>::put(metadata.clone());
			let result: Result<T::ClassId, DispatchError> = orml_nft::Module::<T>::create_class(&who, metadata.clone(), data);
			info!("create :: NFT Class ID = {:?}", result);
            <NftAcalaClass<T>>::put(result.unwrap());
			Self::deposit_event(RawEvent::CreatedToken(who, result.unwrap()));
			Ok(())
		}

		#[weight = 0]
		pub fn mint(origin, data: <T as orml_nft::Trait>::TokenData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			info!("mint :: Start minting nft");
			let result: Result<T::TokenId, DispatchError> = orml_nft::Module::<T>::mint(&who, <NftAcalaClass<T>>::get(), <NftAcalaCID>::get(), data);
            info!("mint :: Minted Token = {:?}", result);
			Self::deposit_event(RawEvent::MintedToken(who, result.unwrap()));
			Ok(())
        }
        
        #[weight = 0]
		pub fn transfer(origin, to: T::AccountId, token: (T::ClassId, T::TokenId)) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            if who == to {
                return Ok(());
            }
            info!("transfer :: Start transfer nft");
			let result: Result<(), DispatchError> = orml_nft::Module::<T>::transfer(&who, &to, token);
            info!("transfer :: Transfered Token = {:?}", result);
            let token_id = token.0;
            let token_class = token.1;
			Self::deposit_event(RawEvent::TransferredToken(who, to, token_id, token_class));            
			Ok(())
        }
        
		#[weight = 0]
		pub fn burn(origin, token: (T::ClassId, T::TokenId)) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            info!("burn :: Start burning nft");
			let result: Result<(), DispatchError> = orml_nft::Module::<T>::burn(&who, token);
            info!("burn :: Burned Token = {:?}", result);
            let token_id = token.0;
            let token_class = token.1;
			Self::deposit_event(RawEvent::BurnedToken(who, token_id, token_class));   
            Ok(())
        }
	}
}
