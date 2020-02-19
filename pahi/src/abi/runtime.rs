use crate::env::*;
use log::info;
use std::{thread, time};
use wasmer_runtime::{Array, Ctx, WasmPtr};

pub fn exit(
    ctx: &mut Ctx,
    code: u32,
) -> Result<(), ()> {
    let (_, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    env.log_call("runtime_exit".to_string());
    info!("{}: exiting with {}", env.host_name, code);
    Err(())
}

pub const NAME: &'static str = "pa'i";

pub fn name(
    ctx: &mut Ctx,
    ptr: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, wasmer_runtime::error::RuntimeError> {
    let (memory, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    if len < NAME.len() as u32 {
        return Ok(NAME.len() as u32);
    }

    unsafe {
        let memory_writer = ptr
            .deref_mut(memory, 0, NAME.len() as u32)
            .expect("pointer deference to work");
        for (i, b) in NAME.bytes().enumerate() {
            memory_writer[i].set(b);
        }

        assert_eq!(
            ptr.get_utf8_string(memory, NAME.len() as u32)
                .unwrap(),
            NAME
        );
    }

    env.log_call("runtime_name".to_string());
    Ok(NAME.len() as u32)
}

pub const CWA_VERSION_MAJOR: u32 = 0;
pub const CWA_VERSION_MINOR: u32 = 2;

pub fn spec_major(ctx: &mut Ctx) -> u32 {
    let (_, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    env.log_call("runtime_spec_major".to_string());

    CWA_VERSION_MAJOR
}
pub fn spec_minor(ctx: &mut Ctx) -> u32 {
    let (_, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    env.log_call("runtime_spec_major".to_string());

    CWA_VERSION_MINOR
}

pub fn sleep(_: &mut Ctx, len: i32) {
    let amt = time::Duration::from_millis(len as u64);
    thread::sleep(amt);
}
