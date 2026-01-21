use anchor_lang::prelude::*;

use crate::Update;

pub fn update(ctx: Context<Update>, new_price: i64) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    oracle.price = new_price;
    Ok(())
}



