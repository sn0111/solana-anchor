use anchor_lang::prelude::*;

declare_id!("2LFoqVKYUZEbiH6uw7egsp14FKevAyZwSyPbAUzi8EeR");

#[program]
pub mod create_note {
    use super::*;

    pub fn initialize(ctx: Context<InitializeNote>,title:String,body:String,date:String) -> Result<()> {
        let user_note = &mut ctx.accounts.note;
        user_note.id=1;
        user_note.title=title;
        user_note.body=body;
        user_note.date=date;
        Ok(())
    }

    pub fn update_note(ctx: Context<UpdateNote>,title:String,body:String,date:String)->Result<()>{
        let note = &mut ctx.accounts.note;
        note.id=1;
        note.title=title;
        note.body=body;
        note.date=date;
        Ok(())
    }

    pub fn note_with_seeds(ctx: Context<NoteWithSeeds>,title:String,body:String,date:String) -> Result<()> {
        let user_note = &mut ctx.accounts.note;
        user_note.id=1;
        user_note.title=title;
        user_note.body=body;
        user_note.date=date;
        Ok(())
    }

    pub fn update_note_with_seeds(ctx: Context<UpdateNoteWithSeeds>,title:String,body:String,date:String) -> Result<()> {
        let user_note = &mut ctx.accounts.note;
        user_note.id=1;
        user_note.title=title;
        user_note.body=body;
        user_note.date=date;
        Ok(())
    }

    pub fn delete_note_seeds(ctx: Context<DeleteNoteWithSeeds>,title:String) -> Result<()> {
        msg!("Seed Note for {} deleted", title);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeNote<'info> {
    #[account(init,payer=user,space=1000)]
    pub note:Account<'info,Note>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>

}

#[derive(Accounts)]
#[instruction(title:String,body:String)]
pub struct NoteWithSeeds<'info>{
    #[account(
        init,
        payer=user,
        seeds=[title.as_bytes(),user.key().as_ref()],
        bump,
        space=2000
    )]
    pub note:Account<'info,Note>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(title:String,body:String)]
pub struct UpdateNoteWithSeeds<'info>{
    #[account(
        mut,
        seeds=[title.as_bytes(),user.key().as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + title.len() + 4 + body.len(),
        realloc::payer=user,
        realloc::zero = true,
    )]
    pub note:Account<'info,Note>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>    
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteNoteWithSeeds<'info>{
    #[account(
        mut,
        seeds=[title.as_bytes(),user.key().as_ref()],
        bump,
        close=user
    )]
    pub note:Account<'info,Note>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>    
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    #[account(mut)]
    pub note:Account<'info,Note>,
    pub user:Signer<'info>,

}

#[account]
pub struct Note{
    pub id:u64,
    pub title:String,
    pub body:String,
    pub date:String
}

impl Note {
    pub fn add(&mut self,note:Note)->Result<()>{
        self.id=note.id;
        self.title=note.title;
        self.body=note.body;
        self.date=note.date;
        Ok(())
    }
}
