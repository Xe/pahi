use crate::{error::Error::NotFound, *};
use wasmer_runtime::{Array, Ctx, WasmPtr};

pub fn get(
    ctx: &mut Ctx,
    key_ptr: WasmPtr<u8, Array>,
    key_len: u32,
    value_ptr: WasmPtr<u8, Array>,
    value_len: u32,
) -> Result<i32, std::option::NoneError> {
    let (memory, env) = Process::get_memory_and_environment(ctx, 0);
    let key = key_ptr.get_utf8_string(memory, key_len)?;

    match env.envvars.get(key) {
        Some(value) => {
            if value_len >= value.len() as u32 {
                unsafe {
                    let memory_writer = value_ptr.deref_mut(memory, 0, value.len() as u32)?;
                    for (i, b) in value.bytes().enumerate() {
                        memory_writer[i].set(b);
                    }

                    assert_eq!(
                        value_ptr
                            .get_utf8_string(memory, value.len() as u32)
                            .unwrap(),
                        value
                    );
                }
            }

            return Ok(value.len() as i32);
        }
        None => {
            return Ok(NotFound as i32);
        }
    }
}
