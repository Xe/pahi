use crate::*;
use rand::prelude::*;
use wasmer_runtime::{Array, Ctx, WasmPtr};

pub fn rand_i32(ctx: &mut Ctx) -> Result<i32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("random_i32".to_string());

    Ok(rand::thread_rng().next_u32() as i32)
}

pub fn rand_u32(ctx: &mut Ctx) -> Result<u32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("random_u32".to_string());

    Ok(rand::thread_rng().next_u32())
}

pub fn rand_i64(ctx: &mut Ctx) -> Result<i64, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("random_i64".to_string());

    Ok(rand::thread_rng().next_u64() as i64)
}

pub fn rand_u64(ctx: &mut Ctx) -> Result<u64, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("random_u64".to_string());

    Ok(rand::thread_rng().next_u64())
}
