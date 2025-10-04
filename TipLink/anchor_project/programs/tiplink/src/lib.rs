use anchor_lang::prelude::*;
pub mod states;
pub mod instructions;
pub mod errors;

use instructions::send_tip::*;

declare_id!("G85GYMTsHW7k4HpvMqFzp2wBotpGM7ZK4n2beivLNK4e");

#[program]
pub mod tiplink {
    use super::*;

    pub fn send_tip(ctx: Context<SendTip>, amount: u64, token_mint: Pubkey) -> Result<()> {
        instructions::send_tip::send_tip(ctx, amount, token_mint)
    }
}