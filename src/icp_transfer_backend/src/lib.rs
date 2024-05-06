use candid::{candid_method, Nat, Principal};
use num_bigint::BigUint;
use ic_cdk::{self, caller, update};
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

use icrc_ledger_types::icrc1::transfer::{ TransferArg, TransferError};

#[ic_cdk::update]
async fn transfer_token(amount: u64) -> Result<BlockIndex, String> {
    let caller = caller();
    let big_int_amount: BigUint = BigUint::from(amount);
    let amount: Nat = Nat(big_int_amount);
    ic_cdk::println!("Transferring {} tokens to account {}", amount, caller,);
    let transfer_args: TransferArg = TransferArg {
        // can be used to distinguish between transactions
        // the amount we want to transfer
        amount,
        // we want to transfer tokens from the default subaccount of the canister
        from_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: caller.into(),
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
        memo: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc1_transfer",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn swap() -> Result<BlockIndex, String> {
    let caller = caller();
    let canister_id: Principal = ic_cdk::api::id();
    let account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));
    ic_cdk::println!("Caller sub-account for deducting icp  is {}", account);
    deposit_icp(caller).await?;
    ic_cdk::println!("******************Icp received to canister! Now Transfering the token*************************8");
    transfer_token(1 as u64).await?;
    Ok(44)
}

#[update]

async fn deposit_icp(caller: Principal) -> Result<BlockIndex, String> {
    let canister_id: Principal = ic_cdk::api::id();
    let canister_account: AccountIdentifier =
        AccountIdentifier::new(&canister_id, &DEFAULT_SUBACCOUNT);
    ic_cdk::println!("Canister account for receiving icp is {}", canister_account);

    let transfer_args: ic_ledger_types::TransferArgs = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(1),
        fee: Tokens::from_e8s(10000),
        from_subaccount: Some(principal_to_subaccount(&caller)),
        to: canister_account,
        created_at_time: None,
    };

    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e: ic_ledger_types::TransferError| format!("ledger transfer error {:?}", e))

  
}

pub fn principal_to_subaccount(principal_id: &Principal) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

    Subaccount(subaccount)
}
ic_cdk::export_candid!();
