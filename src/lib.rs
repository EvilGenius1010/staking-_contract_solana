#![allow(unused_imports)]


use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError::{self, InvalidInstructionData, MissingRequiredSignature}, pubkey::Pubkey, rent::Rent, stake::instruction::{create_account, delegate_stake},
    system_instruction, sysvar::Sysvar,
    entrypoint
};

#[derive(BorshSerialize,BorshDeserialize)]
struct SetupAccountStruct{
    payer:Pubkey,
    amount:u64,
    pda_seeds:Vec<u8>,
    pda_bump:u8,
    space:u64
}


#[derive(BorshSerialize,BorshDeserialize)]
struct DelegateAccountStruct{
    validator:Pubkey,
    payer:Pubkey,
    pda_seeds:Vec<u8>,
    pda_bump:u8,
    stake_pubkey:Pubkey,
    authorized_pubkey:Pubkey,
    vote_pubkey:Pubkey
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
            SetupAccount(SetupAccountStruct,program_id,payer);
        },
        Ok(Instructions::Delegate(DelegateAccountStruct))=>{
            Delegate(DelegateAccountStruct,program_id,payer);
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


pub fn SetupAccount(ix_data:SetupAccountStruct,program_id:&Pubkey,payer:&AccountInfo)->Result<(), ProgramError>{
    // assert!(payer.is_signer);

    if payer.key!=&ix_data.payer{
        panic!("Incorrect account used!");
    }

    //get rent for account
    let account_rent = Rent::from_account_info(payer)?.lamports_per_byte_year;

    let ix = system_instruction::create_account(
        payer.key, 
payer.key, 
account_rent, 
        ix_data.space, 
        program_id);

    //how does this concatenate?
    let seeds = &[b"test",&ix_data.pda_seeds[..],&[ix_data.pda_bump]];
    
    //cpi call to deposit rent needed
    invoke_signed(&ix, 
        &[payer.clone()],
    &[seeds])

}

fn Delegate(ix_data:DelegateAccountStruct,program_id:&Pubkey,payer:&AccountInfo)->Result<(),ProgramError>{
    let seeds = &[b"test",&ix_data.pda_seeds[..],&[ix_data.pda_bump]];

    let ix = delegate_stake(&ix_data.stake_pubkey, &ix_data.authorized_pubkey, &ix_data.vote_pubkey);

    invoke_signed(&ix,
    &[payer.clone()],
    &[seeds])



}


#[cfg(test)]
mod tests{
    use super::*;  
    use borsh::BorshSerialize;
    use litesvm::LiteSVM;
    use solana_sdk::{
        message::{AccountMeta,Instruction},
        pubkey::{Pubkey,Message},
        signer::{keypair::Keypair, Signer},
        transaction::Transaction,
        system_program
    };
    use crate::{Instructions, SetupAccountStruct};


    #[test]
    fn setup_staking(){

    //create new test env
    let svm=LiteSVM::new();

    //Create keypairs
    let payer = Keypair::new();
    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();


        struct ValidatorInfo {
        vote_account: Keypair,
        validator_identity: Keypair,
        withdrawal_authority: Keypair,
    }

    struct StakerInfo {
        keypair: Keypair,
        stake_accounts: Vec<Keypair>,
    }

    struct StakingTestEnv{
        svm: LiteSVM,
        program_id: Pubkey,
        validators: Vec<ValidatorInfo>,
        stakers: Vec<StakerInfo>,
    }

    //add funds
    svm.airdrop(payer.pubkey(),1_000_000_000);

    //load compiled program
    svm.add_program_from_file(program_id, "target/deploy/staking_contract.so")
    .unwrap();

    let setup_account_data = SetupAccountStruct{
        payer: payer.pubkey(),
        amount: 1_000_000_000,
        pda_seeds: b"test_seed".to_vec(),
        pda_bump: 255,
        space: 200,
    };

    let ix_data = Instructions::Setup(setup_account_data);
    let serialized_instruction = ix_data.serialize(writer);

    let ix = Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(Pubkey::new_unique(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data:serialized_data
    };


     // Execute transaction
    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());
    let result = svm.send_transaction(tx);



    }

}