use anchor_lang::prelude::*;

declare_id!("3R3RyYdY3yZJbSPi4pduJc7wabLEhrPE7Y14JYSusZug");
#[program]
pub mod study_chain {
    use super::*;

    pub fn create_commitment(
        ctx: Context<CreateCommitment>,
        commitment: String,
        deadline: i64,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.student = ctx.accounts.student.key();
        record.commitment = commitment;
        record.deadline = deadline;
        record.completed = false;
        record.failed = false;
        Ok(())
    }

    pub fn complete_commitment(ctx: Context<CompleteCommitment>) -> Result<()> {
        let record = &mut ctx.accounts.record;
        require!(!record.failed, ErrorCode::AlreadyFailed);
        record.completed = true;
        Ok(())
    }

    pub fn fail_commitment(ctx: Context<FailCommitment>) -> Result<()> {
        let record = &mut ctx.accounts.record;
        let clock = Clock::get()?;

        require!(
            clock.unix_timestamp > record.deadline,
            ErrorCode::DeadlineNotPassed
        );
        require!(!record.completed, ErrorCode::AlreadyCompleted);

        record.failed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCommitment<'info> {
    #[account(
        init,
        payer = student,
        space = 8 + 32 + 4 + 200 + 8 + 1 + 1
    )]
    pub record: Account<'info, StudyRecord>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteCommitment<'info> {
    #[account(mut)]
    pub record: Account<'info, StudyRecord>,
    pub student: Signer<'info>,
}

#[derive(Accounts)]
pub struct FailCommitment<'info> {
    #[account(mut)]
    pub record: Account<'info, StudyRecord>,
    pub caller: Signer<'info>,
}

#[account]
pub struct StudyRecord {
    pub student: Pubkey,
    pub commitment: String,
    pub deadline: i64,
    pub completed: bool,
    pub failed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Deadline has not passed yet")]
    DeadlineNotPassed,
    #[msg("Commitment has already been completed")]
    AlreadyCompleted,
    #[msg("Commitment has already failed")]
    AlreadyFailed,
}
