#[warn(unused_imports)]
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    sysvar::Sysvar,
};

#[warn(unused_imports)]
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    system_instruction,
};


entrypoint!(process_instruction);
const program_id: &str =  "6sbzC1eH4FTujJXWj51eQe25cYvr4xfXbJ1vAj7j2k5J";
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the instruction data to get the action (0 for SOL distribution)
    let action = instruction_data[0];

    match action {
        // Distribute SOL to the recipient account
        0 => {
            // Ensure that the first account is the recipient of SOL
            let recipient_account_info = &accounts[0];

            // Transfer SOL from the faucet account to the recipient
            let faucet_account_info = &accounts[1];
            let system_program_info = &accounts[2];

            // this checks if the recipient has already some sol for fairer distribution 
            //if recipient_account_info.lamports() > &0 {
            //    return Err(ProgramError::AccountAlreadyInitialized);
            //}

            // Calculate the amount of SOL to distribute (you can adjust this as needed)
            let sol_amount = 100_000_000; // 100,000,000 SOL (0.1 SOL)

            // Transfer SOL from the faucet account to the recipient
            let transfer_instruction = system_instruction::transfer(
                &faucet_account_info.key,
                &recipient_account_info.key,
                sol_amount,
            );

            // Send the transfer instruction
            solana_program::program::invoke_signed(
                &transfer_instruction,
                &[faucet_account_info.clone(), recipient_account_info.clone(), system_program_info.clone()],
                &[],
            )?;
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
} 
