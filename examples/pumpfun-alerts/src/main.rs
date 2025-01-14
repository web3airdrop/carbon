use async_trait::async_trait;
use carbon_core::deserialize::ArrangeAccounts;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType, metrics::MetricsCollection,
    processor::Processor,
};

use carbon_pumpfun_decoder::instructions::{buy::Buy, sell::Sell, PumpfunInstruction};
use carbon_pumpfun_decoder::PumpfunDecoder;
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use solana_sdk::pubkey;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};

pub const PUMPFUN_PROGRAM_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "pumpfun_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![PUMPFUN_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![PUMPFUN_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("raydium_transaction_filter".to_string(), transaction_filter);

    println!("GEYSER_URL: {}", env::var("GEYSER_URL").unwrap_or_default());
    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let pumpfun_instruction: PumpfunInstruction = instruction.data;
        let accounts = instruction.accounts;

        // get slot and signature
        let transaction_metadata = metadata.transaction_metadata;
        let _slot = transaction_metadata.slot;
        let signature = transaction_metadata.signature;
        let _fee_payer = transaction_metadata.fee_payer;

        match pumpfun_instruction {
            PumpfunInstruction::CreateEvent(create_event) => {
                println!("\nNew token created: {:#?}", create_event);
            }

            PumpfunInstruction::TradeEvent(trade_event) => {
                if trade_event.sol_amount >= 5 * LAMPORTS_PER_SOL {
                    println!("\nBig trade occured: {:#?}", trade_event);
                }
            }

            PumpfunInstruction::CompleteEvent(complete_event) => {
                println!("\nBonded: {:#?}", complete_event);
            }

            PumpfunInstruction::Buy(buy) => {
                println!("\n signature: {:#?}\n Buy: {:#?}", signature, buy);
                println!(
                    "\n Buy ArrangedAccounts: {:#?}\n",
                    Buy::arrange_accounts(accounts)
                );
            }

            PumpfunInstruction::Sell(sell) => {
                println!("\n signature: {:#?}\n Sell: {:#?}", signature, sell);
                println!(
                    "\n Sell ArrangedAccounts: {:#?}\n",
                    Sell::arrange_accounts(accounts)
                );
            }

            _ => {}
        };

        Ok(())
    }
}
