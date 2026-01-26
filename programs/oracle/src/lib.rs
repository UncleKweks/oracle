use anchor_lang::prelude::*;

declare_id!("MwUYTnAbCSUKDvNFBdT3kNv7JpKhauKpRgc2DV2LFKX");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, symbol: String, initial_price: i64, expo: i32, confidence: u64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        oracle.owner = ctx.accounts.owner.key();
        oracle.symbol = symbol;
        oracle.price = initial_price;
        oracle.expo = expo;
        oracle.confidence = confidence;
        oracle.last_update_slot = Clock::get()?.slot;
        Ok(())
    }

   pub fn update(ctx: Context<Update>, new_price: i64, new_confidence: u64) -> Result<()> {
        instructions::update(ctx, new_price, new_confidence)
    }
}

pub mod instructions;

#[account]
pub struct Oracle {
    pub owner: Pubkey,         // same as v1
    pub symbol: String,        // e.g. "SOL/USD"
    pub price: i64,            // raw price value
    pub expo: i32,             // decimals/exponent, like Pyth
    pub confidence: u64,       // +/- range around price
    pub last_update_slot: u64, // for staleness checks
}

impl Oracle {
    pub const LEN: usize = 32       // owner
        + 4 + 16                    // symbol: String (4 bytes length + up to 16 chars)
        + 8                         // price
        + 4                         // expo
        + 8                         // confidence
        + 8;                        // last_update_slot
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