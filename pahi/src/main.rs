extern crate wasmer_runtime;

mod abi;
mod env;

use crate::env::OlinEnv;
use std::ffi::c_void;
use std::fs;
use wasmer_runtime::{error, func, imports, instantiate};

#[macro_use]
extern crate log;

fn main() -> error::Result<()> {
    env_logger::init();
    log::info!("la pa'i is starting...");
    let args: Vec<String> = std::env::args().collect();
    debug!("args: {:?}", args);

    let env_generator = move || {
        fn destructor(data: *mut c_void) {
            unsafe {
                drop(Box::from_raw(data as *mut OlinEnv));
            }
        }
        let custom_abi_env = Box::new(OlinEnv::new("olin".to_string()));
        (
            Box::into_raw(custom_abi_env) as *mut c_void,
            destructor as fn(*mut c_void),
        )
    };

    let import_object = imports! {
        env_generator,
        // Define the "customabi" namespace that was implicitly used
        // by our sample application.
        "env" => {
            // the func! macro autodetects the signature
            "log_write" => func!(abi::log_write),

            // io
            "io_get_stderr" => func!(abi::io_get_stderr),

            // resource
            "resource_write" => func!(abi::resource_write),

            // runtime
            "runtime_name" => func!(abi::runtime_name),
            "runtime_exit" => func!(abi::runtime_exit),
            "runtime_spec_major" => func!(abi::runtime_spec_major),
            "runtime_spec_minor" => func!(abi::runtime_spec_minor),
        },
    };

    if args.len() != 2 {
        panic!("wanted args.len() == 2, got: {}", args.len());
    }

    let filename = &args[1];
    debug!("opening {}", filename);

    let data: &[u8] = &fs::read(filename).expect("wanted file to have data");
    let mut instance = instantiate(data, &import_object)?;

    if let Err(why) = instance.call("_start", &[]) {
        error!("runtime error: {}", why);
    }

    let (_, env) = OlinEnv::get_memory_and_environment(instance.context_mut(), 0);

    info!("Here are the logged calls: {:?}", env.called_functions);
    Ok(())
}
