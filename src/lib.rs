use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, msg, pubkey::Pubkey,
    stake::instruction::create_account,
    program_error::ProgramError::MissingRequiredSignature,
    progra
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _ix_data: &[u8],
) -> ProgramResult {

    // create_account(from_pubkey, stake_pubkey, authorized, lockup, lamports)

    let accounts_iterator = &mut accounts.iter();
    let payer = next_account_info(accounts_iterator)?;
    let stake_account = next_account_info(accounts_iterator)?;
    let system_program = next_account_info(accounts_iterator)?;

    if !payer.is_signer { return Err(MissingRequiredSignature); }

    if stake_account.owner != program_id { msg!("Not our account"); return Err(ProgramError::IncorrectProgramId); }
    let mut data = stake_account.data.borrow_mut();    


    msg!("Hello, world!");
    Ok(())
}