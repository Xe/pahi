# resource

Operations related to readable/writeable resources.

## close

**Parameters:**

- `fd`: i32 (int)

**Returns:**

- `void`: void (void)

**Effects:** Closes the resource at the given descriptor

**Semantics:**

Closes the resource specified by id.

Calling this on an invalid resource id is a fatal error.

**Errors:**

None

## open

**Parameters:**

- `url_base`: u32 (unsigned int *)
- `url_len`: u32 (size_t)

**Returns:**

- `fd`: i32 (int)

**Effects:** Depends on the scheme of the url, but generally creates a file descriptor in the process environment.

**Semantics:**

Opens the resource specified by url.

Returns the resource id on success, or the error code on failure.

The URL format is as defined [here](../schemes)

**Errors:**

- PermissionDeniedError
- NotFoundError
- InvalidArgumentError

## read

**Parameters:**

- `fd`: i32 (int)
- `data_base`: u32 (unsigned int *)
- `data_len`: u32 (size_t)

**Returns:**

- `len`: i32 (int)

**Effects:** Reads to data (the memory range from data_base to data_base + data_len) from the resource at the given descriptor

**Semantics:**

Returns the actual bytes written on success, or the error on failure.

**Errors:**

- InvalidArgumentError
- UnknownError
- EndOfFile

## write

**Parameters:**

- `fd`: i32 (int)
- `data_base`: u32 (unsigned int *)
- `data_len`: u32 (size_t)

**Returns:**

- `len`: i32 (int)

**Effects:** Writes data (the memory range from data_base to data_base + data_len) to the resource at the given descriptor

**Semantics:**

Returns the actual bytes written on success, or the error on failure.

**Errors:**

- InvalidArgumentError
- UnknownError

