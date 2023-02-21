use anchor_lang::prelude::*;

declare_id!("qGTXzG5tb3aGmo9yojYY2yzMPd74jaivjWTKvnEDc6J");

#[program]
pub mod anchor_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Anchor Hello Program");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
