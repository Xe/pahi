use crate::env::*;
use log::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::Infallible;
use wasmer_runtime::{Array, Ctx, WasmPtr};

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum LogLevel {
    Error = 1,
    Warning = 3,
    Info = 6,
}

pub fn log_write(ctx: &mut Ctx, level: u32, ptr: WasmPtr<u8, Array>, len: u32) {
    let (memory, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    let string = ptr.get_utf8_string(memory, len).unwrap();
    match FromPrimitive::from_u32(level) {
        Some(LogLevel::Info) => info!("{}: {}", env.host_name, string),
        Some(LogLevel::Warning) => warn!("{}: {}", env.host_name, string),
        Some(LogLevel::Error) => error!("{}: {}", env.host_name, string),
        None => {
            panic!("invalid log level {}", level);
        }
    }

    env.log_call("log_write".to_string());
}

pub fn runtime_exit(
    ctx: &mut Ctx,
    code: u32,
) -> Result<Infallible, wasmer_runtime::error::RuntimeError> {
    let (_, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    env.log_call("runtime_exit".to_string());

    Err(wasmer_runtime::error::RuntimeError::Error {
        data: Box::new(format!("{}: exiting with {}", env.host_name, code)),
    })
}

const RUNTIME_NAME: &'static str = "pa'i";

pub fn runtime_name(
    ctx: &mut Ctx,
    ptr: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, wasmer_runtime::error::RuntimeError> {
    let (memory, _) = OlinEnv::get_memory_and_environment(ctx, 0);
    if len < RUNTIME_NAME.len() as u32 {
        return Ok(RUNTIME_NAME.len() as u32);
    }

    unsafe {
        let memory_writer = ptr
            .deref_mut(memory, 0, RUNTIME_NAME.len() as u32)
            .expect("pointer deference to work");
        for (i, b) in RUNTIME_NAME.bytes().enumerate() {
            memory_writer[i].set(b);
        }

        assert_eq!(
            ptr.get_utf8_string(memory, RUNTIME_NAME.len() as u32)
                .unwrap(),
            RUNTIME_NAME
        );
    }

    Ok(RUNTIME_NAME.len() as u32)
}

const CWA_VERSION_MAJOR: u32 = 0;
const CWA_VERSION_MINOR: u32 = 2;

pub fn runtime_spec_major(_: &mut Ctx) -> u32 {
    CWA_VERSION_MAJOR
}
pub fn runtime_spec_minor(_: &mut Ctx) -> u32 {
    CWA_VERSION_MINOR
}

pub fn io_get_stderr(_: &mut Ctx) -> u32 {
    0
}

pub fn resource_write(
    _: &mut Ctx,
    _fd: u32,
    _base: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, wasmer_runtime::error::Error> {
    Ok(len)
}
