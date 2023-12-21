use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("JCAKiGxPrPMpZARbSswpK8NpbeUC7ZJCenmw2hvVhXp9");

#[program]
pub mod post_app_anchor {
    use super::*;

    pub fn create_post(ctx: Context<CreatePost>,title:String, text: String, image_url: String,) -> ProgramResult {
        let post = &mut ctx.accounts.post;
        post.title = title;
        post.body = text;
        post.image_url = image_url;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreatePost<'info,> {
    #[account(init, payer=user, space= 8 + (4 + 50) + (4+ 100) + (4+500))]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
    
}
#[account]
pub struct Post{
    pub title: String,
    pub image_url: String,
    pub body: String,
}
