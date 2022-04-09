pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5fkWJPVaiCqtJgYx8MLgCxL3W8F4iYCn68mtdbsA37G1");

#[program]
pub mod cronos_pool {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        config_bump: u8,
        pool_bump: u8,
        registry_bump: u8,
        registry_page_bump: u8,
        snapshot_bump: u8,
        snapshot_page_bump: u8,
    ) -> Result<()> {
        initialize::handler(
            ctx,
            config_bump,
            pool_bump,
            registry_bump,
            registry_page_bump,
            snapshot_bump,
            snapshot_page_bump,
        )
    }
}