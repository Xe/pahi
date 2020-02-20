use crate::*;
use chrono::prelude::*;
use wasmer_runtime::Ctx;

pub fn now(ctx: &mut Ctx) -> Result<i64, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("time_now".to_string());

    Ok(Utc::now().timestamp())
}
