use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint,TokenInterface};

use crate::state::marketplace::Marketplace;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_bytes()],
        bump,
        space = Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
    // PDA used as the SOL vault for marketplace fees.
    // Not initialized here — it will be implicitly created
    // the first time SOL is transferred into it.
    //just derived the address here
    seeds = [b"treasury", marketplace.key().as_ref()],
    bump,
    )]
    pub treasury: SystemAccount<'info>,


    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.reward_mint,
            name,
        });

        Ok(())
    }
}
