use anchor_lang::prelude::*;

use crate::{Update, CheckPrice, PriceUpdated, OracleError};


pub fn update(
    ctx: Context<Update>,
    new_price: i64,
    new_confidence: u64,
) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;

    // v4 safety guards
    require!(oracle.status == 0, OracleError::OraclePaused);
    require!(new_price >= 0, OracleError::InvalidPrice);

    oracle.price = new_price;
    oracle.confidence = new_confidence;
    oracle.last_update_slot = Clock::get()?.slot;

    emit!(PriceUpdated {
        oracle: oracle.key(),
        symbol: oracle.symbol.clone(),
        price: oracle.price,
        expo: oracle.expo,
        confidence: oracle.confidence,
        slot: oracle.last_update_slot,
    });

    Ok(())
}

pub fn check_price(
    ctx: Context<CheckPrice>,
    max_staleness_slots: u64,
    max_confidence: u64,
) -> Result<()> {
    let oracle = &ctx.accounts.oracle;

    // v4 safety guard
    require!(oracle.status == 0, OracleError::OraclePaused);

    let current_slot = Clock::get()?.slot;

    let age = current_slot
        .checked_sub(oracle.last_update_slot)
        .unwrap_or(u64::MAX);

    require!(age <= max_staleness_slots, OracleError::StalePrice);
    require!(oracle.confidence <= max_confidence, OracleError::ConfidenceTooHigh);

    Ok(())
}

