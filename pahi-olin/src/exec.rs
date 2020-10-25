use cachedir::CacheDirConfig;
use std::collections::{BTreeMap, HashMap};
use wasmer_runtime::{cache::*, compile, error::RuntimeError, Func, Module};

pub struct Opt {
    name: String,
    backend: String,
    cache_prefix: Option<String>,
    entrypoint: String,
    args: Vec<String>,
    env: BTreeMap<String, String>,
}

impl Opt {
    pub fn new(name: String) -> Self {
        Opt {
            name: name,
            backend: "cranelift".into(),
            cache_prefix: None,
            entrypoint: "_start".into(),
            args: vec![],
            env: BTreeMap::<String, String>::new(),
        }
    }

    // builder methods

    /// Sets a custom wasmer backend for this program.
    pub fn backend(mut self, backend: String) -> Self {
        self.backend = backend;
        self
    }

    pub fn cache_prefix(mut self, prefix: String) -> Self {
        self.cache_prefix = Some(prefix);
        self
    }

    pub fn entrypoint(mut self, entrypoint: String) -> Self {
        self.entrypoint = entrypoint;
        self
    }

    pub fn args(mut self, argv: Vec<String>) -> Self {
        self.args = argv;
        self
    }

    pub fn env(mut self, env: BTreeMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub fn system_env(mut self) -> Self {
        for (key, value) in std::env::vars() {
            self.env.insert(key, value);
        }

        self
    }

    pub fn setenv<T, U>(mut self, key: T, val: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.env.insert(key.into(), val.into());
        self
    }
}

pub fn run(opt: Opt, wasm_bytes: &[u8]) -> Result<Status, Error> {
    if opt.backend != "cranelift" {
        Err(Error::BadBackend(opt.backend))?;
    }

    let module: Module = match opt.cache_prefix {
        None => compile(wasm_bytes)?,
        Some(pfx) => {
            let cache_dir = CacheDirConfig::new(&pfx)
                .user_cache(true)
                .get_cache_dir()
                .unwrap()
                .into_path_buf();

            let mut fs_cache = unsafe { FileSystemCache::new(cache_dir)? };
            let key = WasmHash::generate(&wasm_bytes);

            match fs_cache.load(key) {
                Ok(modu) => modu,
                _ => {
                    let modu = compile(wasm_bytes).expect("module to compile");
                    fs_cache
                        .store(key, modu.clone())
                        .expect("cache storage to work");
                    modu
                }
            }
        }
    };

    let entrypoint = opt.entrypoint.clone();

    let imports = crate::import_object(opt.name.clone(), opt.args, opt.env);
    let mut instance = module.instantiate(&imports)?;
    let result = instance
        .exports
        .get::<Func<(), ()>>(&entrypoint)
        .map_err(|_| Error::CantFindEntrypoint(entrypoint.clone()))?
        .call();

    let mut status = Status::default();

    status.exit_code = if let Err(RuntimeError::User(why)) = result {
        match why.downcast_ref::<crate::ExitCode>() {
            Some(exit) => exit.code,
            _ => {
                log::error!("{} exited violently: {:?}", opt.name, why);
                1
            }
        }
    } else {
        0
    };

    let (_, env) = crate::Process::get_memory_and_environment(instance.context_mut(), 0);
    status.resource_urls = env.resource_urls.clone();
    status.called_functions = env.called_functions.clone();

    Ok(status)
}

#[derive(Default, Debug)]
pub struct Status {
    pub exit_code: i32,
    pub resource_urls: Vec<String>,
    pub called_functions: Box<HashMap<String, u32>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("CWA runtime error: {0}")]
    CWAError(#[from] crate::error::Error),

    #[error("bad backend, wanted cranelift and not {0}")]
    BadBackend(String),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("can't find entrypoint {0}")]
    CantFindEntrypoint(String),

    #[error("Compilation error: {0}")]
    CompilationError(#[from] wasmer_runtime::error::CompileError),

    #[error("runtime error: {0}")]
    RuntimeError(#[from] wasmer_runtime::error::Error),
}
