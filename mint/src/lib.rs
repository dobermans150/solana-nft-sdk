use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        native_token::LAMPORTS_PER_SOL,
        program::invoke,
        pubkey::Pubkey,
        system_instruction,
    },
    spl_associated_token_account::instruction as token_account_instruction,
    spl_token::instruction as token_instruction,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let mint = next_account_info(accounts_iter)?; // Token pub address
    let token_account = next_account_info(accounts_iter)?; // token account associated pub key
    let mint_authority = next_account_info(accounts_iter)?; // owner oof the account associated
    let rent = next_account_info(accounts_iter)?; // Rent var
    let system_program = next_account_info(accounts_iter)?; // System program
    let token_program = next_account_info(accounts_iter)?; // Token program var
    let associated_token_program = next_account_info(accounts_iter)?; // Associated token program var

    msg!("Creating token in solana...");
    msg!("Creating solana space of the token...");
    msg!("Mint: {}", mint.key);

    // * Remember, everything is a account in solana, tje Token and nft is a account too.

    invoke(
        &system_instruction::create_account(
            &mint_authority.key,
            &mint.key,
            LAMPORTS_PER_SOL,
            82,
            &token_program.key,
        ),
        &[mint.clone(), mint_authority.clone(), token_program.clone()],
    )?;

    msg!("Initializing mint account...");
    msg!("Creating acount to mint the token");
    msg!("Mint: {}", mint.key);

    invoke(
        &token_instruction::initialize_mint(
            &token_program.key,
            &mint.key,
            &mint_authority.key,
            Some(&mint_authority.key),
            0,
        )?,
        &[
            mint.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    msg!("Creating token account...");
    msg!("Creating account recipe associated of the token...");
    msg!("Token Address: {}", token_account.key);

    invoke(
        &token_account_instruction::create_associated_token_account(
            &mint_authority.key,
            &mint_authority.key,
            &mint.key,
            &token_program.key,
        ),
        &[
            mint.clone(),
            token_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ],
    )?;

    msg!("Mint: {}", mint.key);
    msg!("Token acount Address: {}", token_account.key);

    invoke(
        &token_instruction::mint_to(
            &token_program.key,
            &mint.key,
            &token_account.key,
            &mint_authority.key,
            &[&mint_authority.key],
            1,
        )?,
        &[
            mint.clone(),
            mint_authority.clone(),
            token_account.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    msg!("Token mint process completed successfully.");

    Ok(())
}
