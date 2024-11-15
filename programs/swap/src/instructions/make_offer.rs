use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface, Mint }};

#[derive(Accounts)]
#[instruction(id:u64)]
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
    pub offer: Account<'info, Offer>,
    #[account(
        init, payer = maker,
         associated_token::mint = token_mint_a,
          associated_token::authority = offer, 
          associated_token::token_program = token_program,
            // space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE, seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()], bump
    )]
    pub vault:InterfaceAccount<'info, TokenAccount>, 
    pub system_program:Program<'info, System>,
    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn send_offered_tokens_to_vault(context: &Context<MakeOffer>, token_a_offered_amount:u64) -> Result<()> {
    transfer_tokens(
        
    )
    Ok(())
}
