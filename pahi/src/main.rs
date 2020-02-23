extern crate wasmer_runtime;

use pahi_olin::*;
use std::fs;
use structopt::StructOpt;
use wasmer_runtime::{error, instantiate};

#[macro_use]
extern crate log;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pa'i",
    about = "A WebAssembly runtime in Rust meeting the Olin ABI."
)]
struct Opt {
    /// Backend
    #[structopt(short, long, default_value = "cranelift")]
    backend: String,

    /// Binary to run
    #[structopt()]
    fname: String,

    /// Main function
    #[structopt(short, long, default_value = "_start")]
    entrypoint: String,

    /// Arguments of the wasm child
    #[structopt()]
    args: Vec<String>,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    env_logger::init();
    info!("la pa'i is starting...");
    debug!("args: {:?}", opt.args);

    if opt.backend != "cranelift" {
        return Err(format!(
            "wanted backend to be cranelift, got: {}",
            opt.backend
        ));
    }

    let mut args: Vec<String> = vec![];
    args.push(opt.fname.clone());
    for arg in &opt.args {
        args.push(arg.to_string());
    }

    let filename = opt.fname.clone();
    let imports = import_object(opt.fname, args);
    let mut exit_code = 0;

    debug!("opening {}", filename);

    let data: &[u8] = &fs::read(&filename).expect("wanted file to have data");
    let mut instance = instantiate(data, &imports).expect("wanted imports to work");
    let result = instance
        .func::<(), ()>(&opt.entrypoint)
        .expect("_start not found")
        .call();

    match result {
        Ok(_) => info!("{} exited peacefully", filename),
        Err(why) => match why {
            error::RuntimeError::Error { data } => {
                if let Some(exit) = data.downcast_ref::<ExitCode>() {
                    exit_code = exit.code;
                }
            }
            error::RuntimeError::Trap { ref msg } => {
                error!("{} exited violently: {}", filename, msg);
            }
        },
    }

    let (_, env) = Process::get_memory_and_environment(instance.context_mut(), 0);

    info!("func logs:");

    for (key, val) in env.called_functions.iter() {
        info!("{}: {}", key, val);
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    Ok(())
}
