# io

File descriptors for standard input and output.

## get_out

**Parameters:**

None

**Returns:**

- `fd`: u32 (unsigned int)

**Effects:** Creates a file descriptor in the process environment

**Semantics:**

Returns a read-only file descriptor pointing to the semantic standard output of the WebAssembly VM.

**Errors:**

None

## get_in

**Parameters:**

None

**Returns:**

- `fd`: u32 (unsigned int)

**Effects:** Creates a file descriptor in the process environment

**Semantics:**

Returns a read-only file descriptor pointing to the semantic standard input of the WebAssembly VM.

**Errors:**

None

## get_err

**Parameters:**

None

**Returns:**

- `fd`: u32 (unsigned int)

**Effects:** Creates a file descriptor in the process environment

**Semantics:**

Returns a read-only file descriptor pointing to the semantic standard error output of the WebAssembly VM.

**Errors:**

None

