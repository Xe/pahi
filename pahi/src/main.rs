use color_eyre::eyre::{Result};
use pahi_olin::*;
use std::fs;
use structopt::StructOpt;

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

    /// Print syscalls on exit
    #[structopt(short, long)]
    function_log: bool,

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

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    env_logger::init();
    info!("la pa'i is starting...");
    debug!("args: {:?}", opt.args);

    let mut args: Vec<String> = vec![];
    args.push(opt.fname.clone());
    for arg in &opt.args {
        args.push(arg.to_string());
    }

    debug!("opening {}", opt.fname);
    let data: &[u8] = &fs::read(&opt.fname)?;

    let exec_opt = exec::Opt::new(opt.fname)
        .system_env()
        .cache_prefix("pahi".into())
        .entrypoint(opt.entrypoint)
        .args(args);

    match exec::run(exec_opt, data) {
        Ok(status) => {
            if opt.function_log {
                info!("dumping called syscalls");
                for (key, val) in status.called_functions.iter() {
                    info!("{}: {}", key, val);
                }

                info!("dumping resources");
                for url in status.resource_urls.iter() {
                    info!("{}", url);
                }
            }

            if status.exit_code != 0 {
                std::process::exit(status.exit_code);
            }
        }
        Err(why) => {
            error!("runtime error: {}", why);
        }
    }

    Ok(())
}
