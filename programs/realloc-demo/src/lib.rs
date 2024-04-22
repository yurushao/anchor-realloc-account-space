use anchor_lang::prelude::*;

declare_id!("FXFWZVRVm76Dw59UtcEGVWqno5C4U379sdo1PF423Z8A");

#[program]
pub mod realloc_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
