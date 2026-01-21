use anchor_lang::prelude::*;

declare_id!("MwUYTnAbCSUKDvNFBdT3kNv7JpKhauKpRgc2DV2LFKX");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_price: i64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        oracle.owner = ctx.accounts.owner.key();
        oracle.price = initial_price;
        Ok(())
    }

   pub fn update(ctx: Context<Update>, new_price: i64) -> Result<()> {
        instructions::update(ctx, new_price)
    }
}

pub mod instructions;

#[account]
pub struct Oracle {
    pub owner: Pubkey,
    pub price: i64,
}

impl Oracle {
    pub const LEN: usize = 32 + 8; // owner + price
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Oracle::LEN,
    )]
    pub oracle: Account<'info, Oracle>,

    /// The account that will become the oracle owner.
    /// Store its pubkey in the Oracle state.
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub oracle: Account<'info, Oracle>,
    /// The oracle owner authorized to update the price.
    pub owner: Signer<'info>,
}