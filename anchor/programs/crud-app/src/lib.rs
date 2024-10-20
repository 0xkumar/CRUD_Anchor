#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8MyKPcdT5TUWNgTiRmtdbiKQrDL1LnoQysKfb8jeZJwQ");

#[program]
pub mod crud {
    use super::*;
    pub fn create_journal(ctx: Context<CreateEntry>,title: String,message: String) -> Result<()>{
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.owner = *ctx.accounts.owner.key;
      journal_entry.title = title;
      journal_entry.message = message;
      Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>,_title:String, message: String) -> Result<()>{
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.message = message;

      Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()>{
      Ok(())
    }
}

//When you are defining your context data structure you need to define all of the accounts
//that are going to be passed through the given instruction that you're writing
#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateEntry<'info>{
  #[account(
    init,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    space = JournalEntryState::INIT_SPACE,
    payer = owner,
  )]
  pub journal_entry: Account<'info,JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info,System>
}

//This implementation can be used only the Owner can change the Journal Entry

// #[derive(Accounts)]
// pub struct UpdateEntry<'info> {
//     #[account(
//         mut,
//         has_one = owner @ ErrorCode::UnauthorizedAccess, //This constraint checks that the Owner field of the 'journal_entry' 
//         //matches the owner account provided in the context.
//     )]
//     pub journal_entry: Account<'info, JournalEntryState>,

//     #[account(mut)]
//     pub owner: Signer<'info>, //Added the 'owner' field as a Signer<'info> to ensure that the person trying to update 
//     //the entry is actually signing the transaction.
// }

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner, //The Extra Lamports will receive to the Owner
    realloc::zero = true, // setting the original calcuation of space back to zero and then recalcualtes again
  )]
  pub journal_entry: Account<'info,JournalEntryState>,

  #[account(mut)]
  pub owner:Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteEntry<'info>{
  #[account(
    mut,
    seeds = [title.as_bytes(),owner.key().as_ref()],
    bump,
    close = owner, //It closes the account only if the public key specified here is equal to the Signer of the instruction
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info,System>,
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState{
  pub owner: Pubkey,
  #[max_len(50)]
  pub title: String,
  #[max_len(1000)]
  pub message: String,
}


// #[error_code]
// pub enum ErrorCode {
//     #[msg("You are not authorized to perform this action")]
//     UnauthorizedAccess,
// }