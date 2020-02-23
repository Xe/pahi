# io

File descriptors for standard input and output.

## get_stdout

**Parameters:**

None

**Returns:**

- `fd`: u32 (unsigned int)

**Effects:** Creates a file descriptor in the process environment

**Semantics:**

Returns a read-only file descriptor pointing to the semantic standard output of the WebAssembly VM.

**Errors:**

None

## get_stdin

**Parameters:**

None

**Returns:**

- `fd`: u32 (unsigned int)

**Effects:** Creates a file descriptor in the process environment

**Semantics:**

Returns a read-only file descriptor pointing to the semantic standard input of the WebAssembly VM.

**Errors:**

None

## get_stderr

**Parameters:**

None

**Returns:**

- `fd`: u32 (unsigned int)

**Effects:** Creates a file descriptor in the process environment

**Semantics:**

Returns a read-only file descriptor pointing to the semantic standard error output of the WebAssembly VM.

**Errors:**

None

