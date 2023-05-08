use anyhow::Result;
use clap::Parser;

use spl_governance::state::proposal::VoteType;
use themis::{
    args::{self, Commands},
    processor::*,
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

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
    }
}
