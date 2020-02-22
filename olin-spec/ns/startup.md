# startup

Obtain startup information about the current application.

## arg_at

**Parameters:**

- `i`: u32 (unsigned int)
- `out_base (mut)`: u32 (unsigned int *)
- `out_len (mut)`: u32 (size_t)

**Returns:**

- `void`: void (void)

**Effects:** None.

**Semantics:**

Writes the argument at position id to out.

The result is truncated if the argument length is greater than that of out.

Returns the number of bytes written, or InvalidArgumentError if id is greater than or equal to the total number of arguments.

**Errors:**

InvalidArgumentError

## arg_len

**Parameters:**

None

**Returns:**

- `argc`: u32 (unsigned int)

**Effects:** None.

**Semantics:**

Returns the number of arguments passed to the current application, including the path to application binary itself.

**Errors:**

None

