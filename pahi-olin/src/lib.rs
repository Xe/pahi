use std::ffi::c_void;
use wasmer_runtime::{func, imports, Ctx, ImportObject, Memory};

mod abi;
pub mod error;

/// Process is an individual CommonWA process. It is the collection of resources
/// and other macguffins that the child module ends up requiring.
pub struct Process {
    pub name: String,
    pub called_functions: Vec<String>,
}

impl Process {
    pub fn new(host_name: String) -> Self {
        Process {
            name: host_name,
            called_functions: vec![],
        }
    }

    pub fn log_call(&mut self, func_name: String) {
        self.called_functions.push(func_name);
    }

    pub fn get_memory_and_environment(ctx: &mut Ctx, mem_index: u32) -> (&Memory, &mut Process) {
        unsafe { ctx.memory_and_data_mut(mem_index) }
    }
}

pub fn import_object(name: String) -> ImportObject {
    let env_generator = move || {
        let my_name = name.clone();
        fn destructor(data: *mut c_void) {
            unsafe {
                drop(Box::from_raw(data as *mut Process));
            }
        }
        let custom_abi_env = Box::new(Process::new(my_name));
        (
            Box::into_raw(custom_abi_env) as *mut c_void,
            destructor as fn(*mut c_void),
        )
    };

    imports! {
        env_generator,
        "env" => {
            // io
            "io_get_stderr" => func!(abi::io_get_stderr),

            // log
            "log_write" => func!(abi::log::write),

            // resource
            "resource_write" => func!(abi::resource_write),

            // runtime
            "runtime_name" => func!(abi::runtime::name),
            "runtime_exit" => func!(abi::runtime::exit),
            "runtime_spec_major" => func!(abi::runtime::spec_major),
            "runtime_spec_minor" => func!(abi::runtime::spec_minor),
            "runtime_msleep" => func!(abi::runtime::sleep),
        },
    }
}
