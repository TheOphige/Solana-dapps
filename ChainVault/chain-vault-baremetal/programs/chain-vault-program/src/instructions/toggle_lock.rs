//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
/// 
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    // TODO: Add required accounts and constraints
    // Only the vault authority should be able to toggle the lock
    #[account(mut, has_one = vault_authority)]
    pub vault: Account<'info, Vault>,
    #[account(mut, signer)]
    pub vault_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    // TODO: Implement toggle lock functionality
    let vault = &mut ctx.accounts.vault;
    // Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
    vault.locked = !vault.locked;

    // Emit a toggle lock event after successful state change
    emit!(ToggleLockEvent {
        vault: vault.key(),
        vault_authority: vault.vault_authority,
        locked: vault.locked,
    });

    Ok(())
}