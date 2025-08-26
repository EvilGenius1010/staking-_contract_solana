use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, msg, program_error::ProgramError::{self, InvalidInstructionData, MissingRequiredSignature}, pubkey::Pubkey, stake::instruction::create_account
    // progra
};

#[derive(BorshSerialize,BorshDeserialize)]
struct SetupAccountStruct{
    payer:Pubkey,
    amount:u64
}


#[derive(BorshSerialize,BorshDeserialize)]
struct DelegateAccountStruct{
    validator:Pubkey,
    payer:Pubkey
}

// #[derive(BorshSerialize,BorshDeserialize)]
// struct RevokeAuthority{

// }

// // #[derive(BorshSerialize,BorshDeserialize)]
// // struct UpdateAmounts{

// // }

// // #[derive(BorshSerialize,BorshDeserialize)]
// // struct SetupAndDelegate{

// // }


#[derive(BorshSerialize,BorshDeserialize)]
enum Instructions{
  Setup(SetupAccountStruct),
  Delegate(DelegateAccountStruct),
//   Update(UpdateAmounts),
//   SetupAndDelegate(SetupAndDelegate)
}


entrypoint!(entry_instruction);

pub fn entry_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {

    // create_account(from_pubkey, stake_pubkey, authorized, lockup, lamports)

    let accounts_iterator = &mut accounts.iter();
    let payer = next_account_info(accounts_iterator)?;
    let stake_account = next_account_info(accounts_iterator)?;
    let system_program = next_account_info(accounts_iterator)?;

    let instruction_type = Instructions::try_from_slice(ix_data);


    match instruction_type{
        Ok(Instructions::Setup(SetupAccountStruct))=>{
            SetupAccount(SetupAccountStruct,payer);
        },
        Ok(Instructions::Delegate(DelegateAccountStruct))=>{
            Delegate(DelegateAccountStruct);
        },
        Err(errormsg)=>{
            msg!("Invalid Instruction! Error is { }",errormsg);
            return Err(InvalidInstructionData);
        }
    }   

    if !payer.is_signer { return Err(MissingRequiredSignature); }

    // if stake_account.owner != program_id { msg!("Not our account"); return Err(ProgramError::IncorrectProgramId); }
    let mut data = stake_account.data.borrow_mut();    


    msg!("Hello, world!");
    Ok(())
}


fn SetupAccount(ix_data:SetupAccountStruct,payer:&AccountData)->Result<Instructions, ProgramError>{
    assert!(payer.is_signer);

}

fn Delegate(ixdata:DelegateAccountStruct)->Result<Instructions,ProgramError>{

}