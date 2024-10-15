use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use crate::{
    error::CarbonResult, metrics::Metrics, processor::Processor, transaction::TransactionMetadata,
};

#[derive(Debug, Clone)]
pub struct InstructionMetadata {
    pub transaction_metadata: TransactionMetadata,
    pub stack_height: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecodedInstruction<T> {
    pub program_id: Pubkey,
    pub data: T,
    pub accounts: Vec<AccountMeta>,
}

pub trait InstructionDecoder<'a> {
    type InstructionType;

    fn decode_instruction(
        &self,
        instruction: &'a solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>>;
}

pub struct InstructionPipe<T: Send> {
    pub decoder:
        Box<dyn for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static>,
    pub processor: Box<
        dyn Processor<
                InputType = (
                    InstructionMetadata,
                    DecodedInstruction<T>,
                    Vec<NestedInstruction>,
                ),
            > + Send
            + Sync
            + 'static,
    >,
}

#[async_trait]
pub trait InstructionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T: Send + 'static> InstructionPipes<'_> for InstructionPipe<T> {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
        if let Some(decoded_instruction) = self
            .decoder
            .decode_instruction(&nested_instruction.instruction)
        {
            self.processor
                .process(
                    (
                        nested_instruction.metadata.clone(),
                        decoded_instruction,
                        nested_instruction.inner_instructions.clone(),
                    ),
                    metrics.clone(),
                )
                .await?;
        }

        for nested_inner_instruction in nested_instruction.inner_instructions.iter() {
            self.run(nested_inner_instruction, metrics.clone()).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NestedInstruction {
    pub metadata: InstructionMetadata,
    pub instruction: solana_sdk::instruction::Instruction,
    pub inner_instructions: Vec<NestedInstruction>,
}
