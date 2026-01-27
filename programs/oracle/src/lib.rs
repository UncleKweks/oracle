use anchor_lang::prelude::*;

declare_id!("MwUYTnAbCSUKDvNFBdT3kNv7JpKhauKpRgc2DV2LFKX");

pub mod instructions;

#[program]
pub mod oracle {
    use super::*;
    use crate::instructions;

    pub fn initialize(
        ctx: Context<Initialize>,
        symbol: String,
        initial_price: i64,
        expo: i32,
        confidence: u64,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        oracle.owner = ctx.accounts.owner.key();
        oracle.symbol = symbol;
        oracle.price = initial_price;
        oracle.expo = expo;
        oracle.confidence = confidence;
        oracle.last_update_slot = Clock::get()?.slot;
        Ok(())
    }

    pub fn update(
        ctx: Context<Update>,
        new_price: i64,
        new_confidence: u64,
    ) -> Result<()> {
        instructions::update(ctx, new_price, new_confidence)
    }

    pub fn check_price(
        ctx: Context<CheckPrice>,
        max_staleness_slots: u64,
        max_confidence: u64,
    ) -> Result<()> {
        instructions::check_price(ctx, max_staleness_slots, max_confidence)
    }
}

// keep all this as you already had it:

#[account]
pub struct Oracle {
    pub owner: Pubkey,
    pub symbol: String,
    pub price: i64,
    pub expo: i32,
    pub confidence: u64,
    pub last_update_slot: u64,
}

impl Oracle {
    pub const LEN: usize = 32
        + 4 + 16
        + 8
        + 4
        + 8
        + 8;
}

#[event]
pub struct PriceUpdated {
    pub oracle: Pubkey,
    pub symbol: String,
    pub price: i64,
    pub expo: i32,
    pub confidence: u64,
    pub slot: u64,
}

#[error_code]
pub enum OracleError {
    #[msg("Oracle price is too stale")]
    StalePrice,

    #[msg("Oracle confidence interval is too wide")]
    ConfidenceTooHigh,
}

#[derive(Accounts)]
#[instruction(symbol: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Oracle::LEN,
        seeds = [b"oracle", owner.key().as_ref(), symbol.as_bytes()],
        bump
    )]
    pub oracle: Account<'info, Oracle>,

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

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CheckPrice<'info> {
    pub oracle: Account<'info, Oracle>,
}
