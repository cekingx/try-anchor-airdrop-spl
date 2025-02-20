use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

declare_id!("3cTCQwuRXwQ5oVSivWU3tidjKrMTvfxANWM3KtTSvvqa");

#[program]
pub mod try_anchor_send_spl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let airdrop = &mut ctx.accounts.airdrop_account;
        airdrop.bump = ctx.bumps.airdrop_account;
        airdrop.authority = ctx.accounts.signer.key();
        airdrop.airdroped_token = ctx.accounts.airdroped_token.key();

        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.signer_token_account.to_account_info(),
                    to: ctx.accounts.airdroped_token.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                }
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.airdroped_token.to_account_info(),
                    to: ctx.accounts.claimer_token_account.to_account_info(),
                    authority: ctx.accounts.airdrop_account.to_account_info(),
                },
                &[&[&[ctx.accounts.airdrop_account.bump]]],
            ),
            100000
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = signer_token_account.mint == mint.key() && signer_token_account.owner == signer.key()
    )]
    signer_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        space = Airdrop::LEN,
        seeds = [],
        bump
    )]
    pub airdrop_account: Account<'info, Airdrop>,
    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = airdrop_account,
    )]
    airdroped_token: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Claim<'info> {
    pub claimer: Signer<'info>,
    #[account(
        mut,
        seeds = [],
        bump = airdrop_account.bump,
    )]
    pub airdrop_account: Account<'info, Airdrop>,

    #[account(
        mut,
        constraint = airdroped_token.key() == airdrop_account.airdroped_token
    )]
    pub airdroped_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = claimer_token_account.owner == claimer.key()
    )]
    pub claimer_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Airdrop {
    authority: Pubkey,
    bump: u8,
    airdroped_token: Pubkey,
}

impl Airdrop {
    pub const LEN: usize = 8 + 32 + 1 + 32;
}
