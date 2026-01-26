use anchor_lang::prelude::*;

use crate::Update; 
use crate::PriceUpdated;

pub fn update(
    ctx: Context<Update>,
    new_price: i64,
    new_confidence: u64,
) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;

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

