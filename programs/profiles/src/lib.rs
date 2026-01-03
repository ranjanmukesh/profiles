use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");
#[program]
pub mod profiles{
    use super::*;
    
    pub fn create_profile(
        ctx: Context<CreateProfile>,
        name: String,
        headline: String,
        bio: String,
        ) -> Result<()>{

        let profile = &mut ctx.accounts.profile;
        require_keys_eq!(profile.owner,ctx.accounts.user.key(),ProfileError::Unauthorized);
        profile.name = name;
        profile.headline = headline;
        profile.bio = bio;
        Ok(())
    }   
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        name: String,
        headline: String,
        bio: String,
        ) -> Result<()>{
        let profile = &mut ctx.accounts.profile;
        require_keys_eq!(profile.owner,ctx.accounts.user.key(),ProfileError::Unauthorized);
        profile.name = name;
        profile.headline = headline;
        profile.bio = bio;
        Ok(())
    }

    #[derive(Accounts)]
    #[instruction()]
    pub struct CreateProfile<'info>{ 
        #[account(
            init,
            payer = user,
            space = 8 + Profile::INIT_SPACE,
            seeds = [b"profile",user.key().as_ref()],bump)]
            pub profile: Account<'info,Profile>,
            #[account(mut)]
            pub user: Signer<'info>,
            pub system_program: Program<'info,System>,
            }
    #[derive(Accounts)]
    pub struct UpdateProfile<'info>{
        #[account(
            mut,
            seeds = [b"profile",user.key().as_ref()],bump = profile.bump, has_one = owner)]
            pub profile: Account<'info, Profile>,
            pub user:Signer<'info>,
            #[account(address = profile.owner)]
            pub owner: SystemAccount<'info>,
            }
        #[account]
        pub struct Profile{
            pub owner: Pubkey,
            pub name: String,
            pub headline: String,
            pub bio: String,
            pub bump: u8,
        }
        impl Profile{
            pub const INIT_SPACE: usize = 32 + // owner Pubkey
                                           4 + 100 + // name(4 bytes len + data)
                                           4 + 200 + //headline
                                           4 + 1000 + //bio
                                            1; //bump
        }
        #[error_code]
        pub enum ProfileError{
            #[msg("You are not authorized to update thsi profile")]
            Unauthorized,
        }
            
    }
