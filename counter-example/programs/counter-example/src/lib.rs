use anchor_lang::prelude::*;

declare_id!("4qiCyVbvqgb3xz4X2SFuAUAQ7iSwrGpQsDgSyjy13vgS");

#[program]
pub mod counter_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter =&mut ctx.accounts.counter;
        counter.count=0;
        msg!("Counter Account is created.");
        msg!("Counter is initialized with count: {:}",counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) ->Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous Counter: {}",counter.count);
        counter.count +=1;
        msg!("After increment Counter: {}",counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) ->Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous Counter: {}",counter.count);
        counter.count -=1;
        msg!("After decrement Counter: {}",counter.count);
        Ok(())
    }


    pub fn set_count(ctx: Context<Update>,count:u64) ->Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous Counter: {}",counter.count);
        counter.count =count;
        msg!("After set Counter: {}",counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=8+8)]
    pub counter:Account<'info,Counter>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter{
    pub count:u64
}
