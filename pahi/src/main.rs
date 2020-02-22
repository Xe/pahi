extern crate wasmer_runtime;

use pahi_olin::*;
use std::fs;
use structopt::StructOpt;
use wasmer_runtime::instantiate;

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

    debug!("opening {}", filename);

    let data: &[u8] = &fs::read(&filename).expect("wanted file to have data");
    let mut instance = instantiate(data, &imports).expect("wanted imports to work");
    let result = instance
        .func::<(), ()>("_start")
        .expect("_start not found")
        .call();

    match result {
        Ok(_) => info!("{} exited peacefully", filename),
        Err(why) => error!("{} exited violently: {}", filename, why),
    }

    let (_, env) = Process::get_memory_and_environment(instance.context_mut(), 0);

    info!("Here are the logged calls: {:?}", env.called_functions);
    Ok(())
}
