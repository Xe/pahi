extern crate wasmer_runtime;

use pahi_olin::*;
use std::fs;
use wasmer_runtime::{error, instantiate};

#[macro_use]
extern crate log;

fn main() -> error::Result<()> {
    env_logger::init();
    info!("la pa'i is starting...");
    let args: Vec<String> = std::env::args().collect();
    debug!("args: {:?}", args);

    let imports = import_object("olin".to_string());

    if args.len() != 2 {
        panic!("wanted args.len() == 2, got: {}", args.len());
    }

    let filename = &args[1];
    debug!("opening {}", filename);

    let data: &[u8] = &fs::read(filename).expect("wanted file to have data");
    let mut instance = instantiate(data, &imports)?;
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
