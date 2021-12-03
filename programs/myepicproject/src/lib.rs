use anchor_lang::prelude::*;

declare_id!("B3kzHWVNrsM7R5TetVRpYCuTcuvQP36VL7CAiFmamzGg");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // get reference to account
        let base_account = &mut ctx.accounts.base_account;

        // init total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            likes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn increment_likes(ctx: Context<IncrementLikes>, gif_link: String) -> ProgramResult {
        // get base_account ref
        let base_account = &mut ctx.accounts.base_account;

        // get the idx of the gif in gif_list<Vec> by matching on gif_link
        let index = base_account
            .gif_list
            .iter()
            .position(|g| g.gif_link == gif_link)
            .unwrap();

        // using index of gif, find gif and increment likes by 1
        base_account.gif_list[index].likes += 1;

        Ok(())
    }
    pub fn decrement_likes(ctx: Context<DecrementLikes>, gif_link: String) -> ProgramResult {
        // get base_account ref
        let base_account = &mut ctx.accounts.base_account;

        // get the idx of the gif in gif_list<Vec> by matching on gif_link
        let index = base_account
            .gif_list
            .iter()
            .position(|g| g.gif_link == gif_link)
            .unwrap();

        // using index of gif, find gif and increment likes by 1
        base_account.gif_list[index].likes -= 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementLikes<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DecrementLikes<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub likes: i32,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
