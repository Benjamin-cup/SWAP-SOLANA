use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface, Mint }};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token_mint_a:InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b:InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init, payer = maker,  space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE, seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()], bump
    )]
    pub offer: Account<'info, System>,
    pub vault:InterfaceAccount<'info, TokenAccount>, 

}

pub fn send_offered_tokens_to_vault(ctx: Context<MakeOffer>) -> Result<()> {
    msg!("Greetings from: {{:?}}", ctx.program_id);
    Ok(())
}
