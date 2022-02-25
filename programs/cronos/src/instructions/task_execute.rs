use crate::state::*;
use crate::errors::CronosError;

use anchor_lang::prelude::*;
use solana_program::sysvar;

#[derive(Accounts)]
#[instruction()]
pub struct TaskExecute<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.daemon.as_ref()
        ],
        bump = fee.bump,
        constraint = fee.daemon == daemon.key(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.int.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = daemon,
        constraint = task.status == TaskStatus::Queued @ CronosError::TaskNotQueued,
        constraint = task.schedule.exec_at <= clock.unix_timestamp @ CronosError::TaskNotDue,
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<TaskExecute>) -> Result<()> {
    let config = &ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let task = &mut ctx.accounts.task;
    let worker = &mut ctx.accounts.worker;

    task.execute(&ctx.remaining_accounts.iter().as_slice(), config, daemon, fee, worker)
}