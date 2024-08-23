use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use spl_token::{
    state::Mint,
    instruction::{initialize_mint, mint_to},
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let rent_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;

    // Initialize the mint
    let rent = Rent::from_account_info(rent_account)?;
    let mint_authority = *owner_account.key;

    msg!("Initializing the token mint...");
    initialize_mint(
        &program_id,
        &mint_account.key,
        &mint_authority,
        None,
        9, // Decimal places
    )?;

    // Mint tokens to the owner's account
    let mint_to_account = next_account_info(accounts_iter)?;
    let mint_amount = 1000;

    msg!("Minting {} tokens to the owner...", mint_amount);
    mint_to(
        &program_id,
        &mint_account.key,
        &mint_to_account.key,
        &mint_authority,
        &[&mint_authority],
        mint_amount,
    )?;

    Ok(())
}
