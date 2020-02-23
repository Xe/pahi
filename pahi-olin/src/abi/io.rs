use crate::{
    resource::{Resource, Stderr, Stdin, Stdout},
    Process,
};
use wasmer_runtime::Ctx;

pub fn stdout(ctx: &mut Ctx) -> Result<i32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    let fd = env.get_fd();

    env.resources
        .insert(fd, Box::new(Stdout::new("".to_string())));

    Ok(0)
}

pub fn stdin(ctx: &mut Ctx) -> Result<i32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    let fd = env.get_fd();

    env.resources
        .insert(fd, Box::new(Stdin::new("".to_string())));

    Ok(0)
}

pub fn stderr(ctx: &mut Ctx) -> Result<i32, ()> {
    let (_, env) = Process::get_memory_and_environment(ctx, 0);
    let fd = env.get_fd();

    env.resources
        .insert(fd, Box::new(Stderr::new("".to_string())));

    Ok(0)
}
