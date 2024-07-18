use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod constant;
pub mod state;
pub mod util;
use constant::*;
use error::*;
use instructions::*;
use state::*;
use util::*;

declare_id!("556qRmYYuXwFVXYVztx3HvMzePDBchuXMDLBWVzzaG4M");

#[program]
pub mod genius {
    use super::*;
        
    //  called by contract deployer only 1 time to initialize global values
    //  send SOL to global_account, vault, ata_vault to initialize accounts
    pub fn initialize(
        ctx: Context<Initialize>
    ) -> Result<()> {
        Initialize::process_instruction(ctx)
    }


    //  Admin can hand over admin role
    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        change_admin::process_change_admin(ctx, new_admin)
    }

    //  Admin can add new orchestrators
    //  Made a pda for each orchestrator to remove the limit
    pub fn add_orchestrator(
        ctx: Context<AddOrchestrator>
    ) -> Result<()> {
        AddOrchestrator::process_instruction(ctx)
    }

    //  doesn't close account for now
    pub fn remove_orchestrator(
        ctx: Context<RemoveOrchestrator>
    ) -> Result<()> {
        RemoveOrchestrator::process_instruction(ctx)
    }

    //  admin can update threshold amount
    pub fn update_threshold(
        ctx: Context<UpdateThreshold>,
        threshold: u16
    ) -> Result<()> {
        UpdateThreshold::process_instruction(ctx, threshold)
    }

    //  orchestrator can add bridge liquidity
    pub fn add_bridge_liquidity(
        ctx: Context<AddBridgeLiquidity>,
        amount: u64
    ) -> Result<()> {
        AddBridgeLiquidity::process_instruction(ctx, amount)
    }

    //  orchestrator can remove bridge liquidity
    pub fn remove_bridge_liquidity(
        ctx: Context<RemoveBridgeLiquidity>,
        amount: u64
    ) -> Result<()> {
        RemoveBridgeLiquidity::process_instruction(ctx, amount)
    }

    //  user can deposit any token to the vault
    //  uses jupiter cpi to swap token to USDC
    pub fn swap_deposit(
        ctx: Context<SwapDeposit>,
        amount: u64,
        data: Vec<u8>
    ) -> Result<()> {
        SwapDeposit::process_instruction(ctx, amount, data)
    }
    
    //  user can withdraw any token to the vault
    //  orchestrator swap token and send it to user
    pub fn swap_withdraw(
        ctx: Context<SwapWithdraw>,
        data: Vec<u8>
    ) -> Result<()> {
        SwapWithdraw::process_instruction(ctx, data)
    }

    //  user can withdraw USDC
    //  orchestrator sends it to user address
    //  need this cuz many accounts are differenct than swap_withdraw
    pub fn withdraw_stable_coin(
        ctx: Context<WithdrawStableCoin>,
        amount: u64
    ) -> Result<()> {
        WithdrawStableCoin::process_instruction(ctx, amount)
    }
}


