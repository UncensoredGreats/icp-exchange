
## Set up local ledger canister.

1. Configure Defualt & Minter IDs

dfx identity use minter
export MINTER=$(dfx identity get-principal)
export MINTER_ACCOUNT_ID=$(dfx ledger account-id)

dfx identity use default
export DEFAULT=$(dfx identity get-principal)
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

2. DEPLOY THE ICRC & LEDGER CANISTERS (locally)

  dfx deploy --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister --argument "
    (variant {
      Init = record {
        minting_account = \"$MINTER_ACCOUNT_ID\";
        initial_values = vec {
          record {
            \"$DEFAULT_ACCOUNT_ID\";
            record {
              e8s = 10_000_000_000 : nat64;
            };
          };
        };
        send_whitelist = vec {};
        transfer_fee = opt record {
          e8s = 10_000 : nat64;
        };
        token_symbol = opt \"LICP\";
        token_name = opt \"Local ICP\";
      }
    })
  "

  dfx deploy LBRY --specified-id hdtfn-naaaa-aaaam-aciva-cai --argument '
  (variant {
    Init = record {
      token_name = "LBRYs";
      token_symbol = "LBRY";
      minting_account = record {
        owner = principal "'${DEFAULT}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${MINTER}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10_000;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${DEFAULT}'";
      };
      feature_flags = opt record {
        icrc2 = true;
      };
    }
  })
'

dfx deploy icrc1_ledger_canister --argument "(variant { Init =
record {
     token_symbol = \"ICRC1\";
     token_name = \"L-ICRC1\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10_000;
     metadata = vec {};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; 10_000_000_000; }; };
     archive_options = record {
         num_blocks_to_archive = 1000;
         trigger_threshold = 2000;
         controller_id = principal \"${MINTER}\";
     };
 }
})"