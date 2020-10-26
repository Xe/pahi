#![feature(try_trait)]

use rand::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::ffi::c_void;
use wasmer_runtime::{func, imports, Ctx, ImportObject, Memory};

pub mod abi;
pub mod error;
pub mod exec;
pub mod resource;
pub mod scheme;

/// Exit code for child processes
#[derive(Debug)]
pub struct ExitCode {
    pub code: i32,
}

/// Process is an individual CommonWA process. It is the collection of resources
/// and other macguffins that the child module ends up requiring.
pub struct Process {
    pub name: String,
    pub args: Vec<String>,
    pub called_functions: Box<HashMap<String, u32>>,
    pub resource_urls: Vec<String>,
    pub envvars: BTreeMap<String, String>,
    pub resources: Box<HashMap<u32, Box<dyn resource::Resource>>>,
}

impl Process {
    pub fn new(host_name: String, args: Vec<String>, envvars: BTreeMap<String, String>) -> Self {
        openssl::init();

        Process {
            name: host_name,
            args: args,
            called_functions: Box::new(HashMap::new()),
            resource_urls: vec![],
            envvars: envvars,
            resources: Box::new(HashMap::new()),
        }
    }

    pub fn log_call(&mut self, func_name: String) {
        let func_log = &mut self.called_functions.as_mut();

        if func_log.contains_key(&func_name) {
            func_log.insert(func_name.clone(), {
                let amt = func_log[&func_name] + 1;
                amt
            });
        } else {
            func_log.insert(func_name, 1);
        }
    }

    pub fn log_url(&mut self, url: String) {
        self.resource_urls.push(url.clone());
    }

    pub fn get_memory_and_environment(ctx: &mut Ctx, mem_index: u32) -> (&Memory, &mut Process) {
        unsafe { ctx.memory_and_data_mut(mem_index) }
    }

    pub fn get_fd(&self) -> u32 {
        let mut rng = thread_rng();
        let mut result = rng.next_u32() % (65536 / 2);

        while self.resources.contains_key(&result) {
            result = rng.next_u32() % (65536 / 2);
        }

        result
    }
}

pub fn import_object(
    name: String,
    args: Vec<String>,
    envvars: BTreeMap<String, String>,
) -> ImportObject {
    let env_generator = move || {
        let my_name = name.clone();
        fn destructor(data: *mut c_void) {
            unsafe {
                drop(Box::from_raw(data as *mut Process));
            }
        }
        let custom_abi_env = Box::new(Process::new(my_name, args.clone(), envvars.clone()));
        (
            Box::into_raw(custom_abi_env) as *mut c_void,
            destructor as fn(*mut c_void),
        )
    };

    imports! {
        env_generator,
        "cwa" => {
            // env
            "env_get" => func!(abi::env::get),

            // io
            "io_get_stderr" => func!(abi::io::stderr),
            "io_get_stdout" => func!(abi::io::stdout),
            "io_get_stdin" => func!(abi::io::stdin),

            // log
            "log_write" => func!(abi::log::write),

            // random
            "random_i32" => func!(abi::random::rand_i32),
            "random_u32" => func!(abi::random::rand_u32),
            "random_i64" => func!(abi::random::rand_i64),
            "random_u64" => func!(abi::random::rand_u64),

            // resource
            "resource_open" => func!(abi::resource::open),
            "resource_close" => func!(abi::resource::close),
            "resource_read" => func!(abi::resource::read),
            "resource_write" => func!(abi::resource::write),
            "resource_flush" => func!(abi::resource::flush),

            // runtime
            "runtime_name" => func!(abi::runtime::name),
            "runtime_exit" => func!(abi::runtime::exit),
            "runtime_spec_major" => func!(abi::runtime::spec_major),
            "runtime_spec_minor" => func!(abi::runtime::spec_minor),
            "runtime_msleep" => func!(abi::runtime::sleep),

            // startup
            "startup_arg_len" => func!(abi::startup::arg_len),
            "startup_arg_at" => func!(abi::startup::arg_at),

            // time
            "time_now" => func!(abi::time::now),
        },
        "env" => {
            // env
            "env_get" => func!(abi::env::get),

            // io
            "io_get_stderr" => func!(abi::io::stderr),
            "io_get_stdout" => func!(abi::io::stdout),
            "io_get_stdin" => func!(abi::io::stdin),

            // log
            "log_write" => func!(abi::log::write),

            // random
            "random_i32" => func!(abi::random::rand_i32),
            "random_u32" => func!(abi::random::rand_u32),
            "random_i64" => func!(abi::random::rand_i64),
            "random_u64" => func!(abi::random::rand_u64),

            // resource
            "resource_open" => func!(abi::resource::open),
            "resource_close" => func!(abi::resource::close),
            "resource_read" => func!(abi::resource::read),
            "resource_write" => func!(abi::resource::write),
            "resource_flush" => func!(abi::resource::flush),

            // runtime
            "runtime_name" => func!(abi::runtime::name),
            "runtime_exit" => func!(abi::runtime::exit),
            "runtime_spec_major" => func!(abi::runtime::spec_major),
            "runtime_spec_minor" => func!(abi::runtime::spec_minor),
            "runtime_msleep" => func!(abi::runtime::sleep),

            // startup
            "startup_arg_len" => func!(abi::startup::arg_len),
            "startup_arg_at" => func!(abi::startup::arg_at),

            // time
            "time_now" => func!(abi::time::now),
        },
    }
}
