extern crate wasmer_runtime;

use cachedir::CacheDirConfig;
use pahi_olin::*;
use std::fs;
use structopt::StructOpt;
use wasmer_runtime::{cache::*, compile, error::RuntimeError, Func, Module};

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

    /// Do not cache compiled code?
    #[structopt(short, long)]
    no_cache: bool,

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
    let module: Module;

    if !opt.no_cache {
        let cache_dir = CacheDirConfig::new("pahi")
            .user_cache(true)
            .get_cache_dir()
            .unwrap()
            .into_path_buf();

        let mut fs_cache =
            unsafe { FileSystemCache::new(cache_dir).expect("wanted to create FS cache") };
        let key = WasmHash::generate(&data);

        match fs_cache.load(key) {
            Ok(modu) => {
                module = modu;
            }
            _ => {
                module = compile(data).expect("module to compile");
                fs_cache
                    .store(key, module.clone())
                    .expect("cache storage to work");
            }
        }
    } else {
        module = compile(data).expect("module to compile");
    }

    let mut instance = module.instantiate(&imports).expect("instantiation to work");
    let result = instance
        .exports
        .get::<Func<(), ()>>(&opt.entrypoint)
        .expect("_start not found")
        .call();

    if let Err(RuntimeError::User(why)) = result {
        match why.downcast_ref::<ExitCode>() {
            Some(exit) => {
                exit_code = exit.code;
            }
            _ => {
                error!("{} exited violently: {:?}", filename, why);
            }
        }
    } else {
        info!("{} exited peacefully", filename);
    }

    let (_, env) = Process::get_memory_and_environment(instance.context_mut(), 0);

    if opt.function_log {
        info!("func logs:");

        for (key, val) in env.called_functions.iter() {
            info!("{}: {}", key, val);
        }
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    Ok(())
}
