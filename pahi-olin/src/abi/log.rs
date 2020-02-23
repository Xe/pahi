use crate::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use wasmer_runtime::{Array, Ctx, WasmPtr};

/// The log levels used by CommonWA. See the log namespace documentation for
/// more information.
#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum LogLevel {
    Error = 1,
    Warning = 3,
    Info = 6,
}

pub fn write(ctx: &mut Ctx, level: u32, ptr: WasmPtr<u8, Array>, len: u32) {
    let (memory, env) = Process::get_memory_and_environment(ctx, 0);
    let string = ptr.get_utf8_string(memory, len).unwrap();
    match FromPrimitive::from_u32(level) {
        Some(LogLevel::Info) => eprintln!("{}: info: {}", env.name, string),
        Some(LogLevel::Warning) => eprintln!("{}: warn: {}", env.name, string),
        Some(LogLevel::Error) => eprintln!("{}: error: {}", env.name, string),
        None => panic!("invalid log level {}", level),
    }

    env.log_call("log_write".to_string());
}
