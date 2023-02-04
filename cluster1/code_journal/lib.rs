// From Program-Examples: https://github.com/solana-developers/program-examples/blob/main/basics/cross-program-invocation/native/programs/lever/src/lib.rs
// Borsh, a compact and well-specified format developed by the NEAR project, 
// suitable for use in protocol definitions and for archival storage. 
// Here used for serialization and deserialization of internal structs defined
use borsh::{ BorshDeserialize, BorshSerialize };
// import required solana_program crates 
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

// Because multiple crates that are linked together cannot all define program entrypoints, 
// a common convention is to use a Cargo feature called no-entrypoint 
// to allow the program entrypoint to be disabled.
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // deserialize PowerStatus from instruction_data slice of unsigned 8-bit integers
    // if sucessfule deserialized, call initialize with power_status and return its ProgramResult
    // otherwise ignore the error and continue to the next match statement
    match PowerStatus::try_from_slice(&instruction_data) {
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {},
    }

    // same behaviour as in previous match statement, with small changes:
    // PowerStatus -> SetPowerStatus and initialize -> switch_power
    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {},
    }
    // instruction was not recognized:
    // there is no match in any of the previous match statements
    // (instruction_data doesn't contain PowerStatus and SetPowerStatus data)
    // exit with InvalidInstructionData error
    Err(ProgramError::InvalidInstructionData)
}



pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {
    // assumed that at least 3 accounts are provided, if less accounts will be provided:
    // ProgramError::NotEnoughAccountKeys will be returned by next_account_info and
    // the function will stop execution and return the error using the ? operator
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // calculate the size of power_status bytes vector
    let account_span = (power_status.try_to_vec()?).len();
    // get minimal rent in lamports required to store serialized power_status data of size account_span
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // Invoke a cross-program instruction:
    // first argument: system instruction created using system_instruction::create_account
    // second argument: reference to a slice of AccountInfo values required by system instruction
    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            user.clone(), power.clone(), system_program.clone()
        ]
    )?;

    // serialize power_data
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    // The value () inside the Ok variant is an empty tuple, 
    // which is often used in Rust to indicate a successful operation that doesn't return any meaningful value
    Ok(())
}

pub fn switch_power(
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    // deserialize PowerStatus from acount data
    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    // flip is_on boolean value
    power_status.is_on = !power_status.is_on;
    // serialize back flipped value
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;
    // log message 
    msg!("{} is pulling the power switch!", &name);
    // log current power_status state using match statement
    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}
