//#![cfg(feature = "test-bpf")]

use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_template::{
    self, instruction, processor,
    state::{UniswapV3Input, UniswapV3State},
};
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "solana-program-template",
        solana_program_template::id(),
        processor!(processor::Processor::process_instruction),
    )
}

#[tokio::test]
async fn test_call_example_instruction() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let account = Keypair::new();
    let rent = banks_client.get_rent().await.unwrap();
    let rent = rent.minimum_balance(1 + UniswapV3State::LEN * 5);
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &account.pubkey(),
                rent,
                (UniswapV3State::LEN as u64) * 5,
                &solana_program_template::id(),
            ),
            instruction::init_rkyv(&solana_program_template::id(), &account.pubkey()).unwrap(),
            //instruction::init_borsh(&solana_program_template::id(), &account.pubkey()).unwrap(),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[instruction::read_rkyv(&solana_program_template::id(), &account.pubkey()).unwrap()],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
