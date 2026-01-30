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
    require!(symbol.len() <= 16, OracleError::SymbolTooLong);
    require!(initial_price >= 0, OracleError::InvalidPrice);

    let oracle = &mut ctx.accounts.oracle;
    oracle.owner = ctx.accounts.owner.key();
    oracle.symbol = symbol;
    oracle.price = initial_price;
    oracle.expo = expo;
    oracle.confidence = confidence;
    oracle.last_update_slot = Clock::get()?.slot;

    // v4 defaults
    oracle.status = 0; // Active
    oracle.max_staleness_slots = 10_000;
    oracle.max_confidence = confidence; // or use a relaxed default if you prefer

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

    pub fn set_policy(ctx: Context<SetPolicy>, max_staleness_slots: u64, max_confidence: u64) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    oracle.max_staleness_slots = max_staleness_slots;
    oracle.max_confidence = max_confidence;
    Ok(())
}

    pub fn pause(ctx: Context<SetStatus>) -> Result<()> {
        ctx.accounts.oracle.status = 1;
        Ok(())
    }

    pub fn resume(ctx: Context<SetStatus>) -> Result<()> {
        ctx.accounts.oracle.status = 0;
        Ok(())
    }


   pub fn check_price_stored(ctx: Context<CheckPrice>) -> Result<()> {
    let max_staleness_slots = ctx.accounts.oracle.max_staleness_slots;
    let max_confidence = ctx.accounts.oracle.max_confidence;

    instructions::check_price(ctx, max_staleness_slots, max_confidence)
}

}

#[account]
pub struct Oracle {
    pub owner: Pubkey,
    pub symbol: String,
    pub price: i64,
    pub expo: i32,
    pub confidence: u64,
    pub last_update_slot: u64,
    pub status: u8,                // 0=Active, 1=Paused
    pub max_staleness_slots: u64,  // stored policy
    pub max_confidence: u64,       // stored policy
}

impl Oracle {
    pub const LEN: usize = 32
        + (4 + 16)
        + 8
        + 4
        + 8
        + 8
        + 1        // status
        + 8        // max_staleness_slots
        + 8;       // max_confidence
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

    #[msg("Oracle is paused")]
    OraclePaused,

    #[msg("Symbol too long (max 16 bytes)")]
    SymbolTooLong,

    #[msg("Price must be non-negative")]
    InvalidPrice,
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


#[derive(Accounts)]
pub struct SetPolicy<'info> {
    #[account(mut, has_one = owner)]
    pub oracle: Account<'info, Oracle>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetStatus<'info> {
    #[account(mut, has_one = owner)]
    pub oracle: Account<'info, Oracle>,
    pub owner: Signer<'info>,
}
