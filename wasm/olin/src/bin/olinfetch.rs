#![no_main]
#![feature(start)]

extern crate olin;

use anyhow::{anyhow, Result};
use olin::{entrypoint, env, runtime, stdio, time};
use std::io::Write;

entrypoint!();

fn main() -> Result<()> {
    let mut out = stdio::out();
    if let Ok(url) = env::get("GEMINI_URL") {
        write!(out, "20 text/gemini\n# WebAssembly Runtime Information\n")?;
        write!(out, "URL: {}\n", url)?;
        write!(
            out,
            "Server software: {}\n",
            env::get("SERVER_SOFTWARE").unwrap()
        )?;
    }

    let mut rt_name = [0u8; 32];
    let runtime_name = runtime::name_buf(rt_name.as_mut())
        .ok_or_else(|| anyhow!("Runtime name larger than 32 byte limit"))?;

    write!(out, "CPU:     {}\n", "wasm32").expect("write to work");
    write!(
        out,
        "Runtime: {} {}.{}\n",
        runtime_name,
        runtime::spec_major(),
        runtime::spec_minor()
    )?;
    write!(out, "Now:     {}\n", time::now().to_rfc3339())?;
    Ok(())
}
