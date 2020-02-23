use crate::{
    resource::{Resource, Stderr, Stdin, Stdout},
    Process,
};
use log::debug;
use url::Url;
use wasmer_runtime::Ctx;

pub fn stdout(ctx: &mut Ctx) -> Result<u32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    let fd = env.get_fd();
    debug!("stdout: {}", fd);

    env.resources.insert(
        fd,
        Box::new(Stdout::new(Url::parse("pahi:stdout").unwrap())),
    );

    Ok(fd)
}

pub fn stdin(ctx: &mut Ctx) -> Result<u32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    let fd = env.get_fd();
    debug!("stdin: {}", fd);

    env.resources
        .insert(fd, Box::new(Stdin::new(Url::parse("pahi:stdin").unwrap())));

    Ok(fd)
}

pub fn stderr(ctx: &mut Ctx) -> Result<u32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    let fd = env.get_fd();
    debug!("stderr: {}", fd);

    env.resources.insert(
        fd,
        Box::new(Stderr::new(Url::parse("pahi:stderr").unwrap())),
    );

    Ok(fd)
}
