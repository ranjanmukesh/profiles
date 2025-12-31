use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::*;
use solana_sdk::{
    instruction::Instruction,
    signature:;Keypair,
    signer::Signer,
    transaction::Transaction,
};
#[tokio::test]
async fn create_and_update_profile(){
    let program_id = solana_linkedin_profiles::id();
    let mut program_test = ProgramTest::new(
        "solana_linkdin_profiles",
        program_id,
        processor!(profiles::entry),);
    let(mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let user = Keypair::new();
    let transfer_ix = solana_sdk::system_instruction::transfer(
        &payer.pubkey(),
        &user.pubkey(),
        1_000_000_000,
        );
    let mut tx = Transaction::new_with_payer(&[transfer_ix],Some(&payer.pubkey()),);
    tx.sign(&[&payer],recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();
    let create_ix = Instruction{
        program_id,
        accounts: CreateProfile{
            profile: profile_pda,
            user: user.pubkey(),
            system_program:system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::CreateProfile{
            name:"Alice Dev".to_string(),
            headline: "Rust Builder on Solana".to_string(),
            bio:"Passionate about decentralized apps".to_string(),
        }
        .data()
    };
    let mut tx = Transaction
}
