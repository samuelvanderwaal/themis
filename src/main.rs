use anyhow::Result;
use clap::Parser;

use spl_governance::state::proposal::VoteType;
use themis::{
    args::{self, Commands},
    processor::{
        deposit, execute, propose, vote, withdraw, DepositArgs, ExecuteArgs, ProposeArgs, VoteArgs,
        WithdrawArgs,
    },
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

    dotenv::dotenv().ok();

    let args = args::Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::Propose {
            source_buffer,
            spill_account,
            name,
            description,
            mint_type,
            options,
        } => propose(ProposeArgs {
            keypair_path,
            rpc_url,
            source_buffer,
            spill_account,
            name,
            description,
            mint_type,
            vote_type: VoteType::SingleChoice,
            options,
        }),
        Commands::Vote {
            proposal_id,
            vote_choice,
            mint_type,
            latest,
        } => vote(VoteArgs {
            keypair_path,
            rpc_url,
            proposal_id,
            vote_choice,
            mint_type,
            latest,
        }),
        Commands::Execute {
            proposal_id,
            mint_type,
            latest,
        } => execute(ExecuteArgs {
            keypair_path,
            rpc_url,
            proposal_id,
            mint_type,
            latest,
        }),
        Commands::Deposit { amount, mint_type } => deposit(DepositArgs {
            keypair_path,
            rpc_url,
            amount,
            mint_type,
        }),
        Commands::Withdraw { mint_type } => withdraw(WithdrawArgs {
            keypair_path,
            rpc_url,
            mint_type,
        }),
    }
}
