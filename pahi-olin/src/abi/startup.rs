use crate::*;
use wasmer_runtime::{Array, Ctx, WasmPtr};

pub fn arg_len(ctx: &mut Ctx) -> Result<u32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("startup_arg_len".to_string());

    Ok(env.args.len() as u32)
}

pub fn arg_at(ctx: &mut Ctx, id: u32, ptr: WasmPtr<u8, Array>, len: u32) -> Result<i32, ()> {
    let (memory, env) = Process::get_memory_and_environment(ctx, 0);
    env.log_call("startup_arg_at".to_string());

    if env.args.len() < (id + 1) as usize {
        return Ok(error::Error::InvalidArgument as i32);
    }

    let val = &env.args[id as usize];
    if val.len() < len as usize {
        unsafe {
            let memory_writer = ptr
                .deref_mut(memory, 0, val.len() as u32)
                .expect("pointer deference to work");
            for (i, b) in val.bytes().enumerate() {
                memory_writer[i].set(b);
            }

            assert_eq!(ptr.get_utf8_string(memory, val.len() as u32).unwrap(), val);
        }
    }

    Ok(val.len() as i32)
}
