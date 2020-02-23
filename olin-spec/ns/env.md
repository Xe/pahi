# env

Get configuration information (like database credentials) from the host.

## get

**Parameters:**

- `key_base`: u32 (unsigned int *)
- `key_len`: u32 (size_t)
- `value_base (mut)`: u32 (unsigned int *)
- `value_len (mut)`: u32 (size_t)

**Returns:**

- `result`: i32 (int)

**Effects:** Looks up key in the environment for the process

**Semantics:**

Requests a value from the environment, like environment variables for
common applications. `key` indicate the buffer
containing the key for the information we want to request.
`value` indicate the buffer in which the value
will be written.

Returns:
- `NotFoundError` if the key does not exist
- the number of bytes written to the value buffer if the key exists and the buffer
  is big enough
- the needed size for the value buffer if it was not big enough. No data is written,
  and the caller must check if that return value is larger than `value.len` to
  reallocate and retry.

**Errors:**

- NotFoundError

