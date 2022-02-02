use anchor_lang::prelude::*;

declare_id!("EHCVqbuK5NhFbULkRfvg11txdYbNbaDvEuKSNvCTnNJf");

#[program]
pub mod gif_portal_backend {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    //get accounts
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    //create gif item
    let item = GifItem{
      link: gif_link,
      user_address: *user.to_account_info().key,
      upvotes: 0,
    };

    //push to array
    base_account.total_gifs += 1;
    base_account.gif_list.push(item);
    Ok(())
  }

  pub fn increment_gif_upvotes(ctx: Context<IncrementGifUpvotes>, gif_index: String) -> ProgramResult {
    //get account
    let base_account = &mut ctx.accounts.base_account;

    let index: usize = match gif_index.parse() {
      Ok(i) => i,
      Err(_) => return Err(ProgramError::InvalidArgument),
    };

    //get gif from index
    let gif = &mut base_account.gif_list[index];

    //increment upvotes
    gif.upvotes += 1;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementGifUpvotes<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GifItem {
  pub link: String,
  pub user_address: Pubkey,
  pub upvotes: u64,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<GifItem>,
}