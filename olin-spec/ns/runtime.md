# runtime

Control and obtain information about the runtime environment.

## exit

**Parameters:**

- `status`: i32 (int)

**Returns:**

- `void`: void (void)

**Effects:** Stops VM execution.

**Semantics:**

Immediately exits the VM and passes the status code passed to this function up the stack.

**Errors:**

None

## msleep

**Parameters:**

- `ms`: u32 (unsigned int)

**Returns:**

- `void`: void (void)

**Effects:** Sleeps for the given ms milliseconds.

**Semantics:**

If the environment does not support sleeping, calling this results in a fatal error.

**Errors:**

None

## name

**Parameters:**

- `out_base (mut)`: u32 (unsigned int *)
- `out_len (mut)`: u32 (size_t)

**Returns:**

- `len`: i32 (int)

**Effects:** None.

**Semantics:**

Writes the name of the current runtime environment to out and returns the number of bytes written.

The name must not be longer than 32 bytes and must be valid UTF-8.

**Errors:**

None

## spec_major

**Parameters:**

None

**Returns:**

- `spec_major_version`: u32 (unsigned int)

**Effects:** None.

**Semantics:**

Returns the major version of the CommonWA spec implemented by the runtime environment.

This value must be equal to the version which the application targets to ensure all features working correctly.

**Errors:**

None

## spec_minor

**Parameters:**

None

**Returns:**

- `spec_minor_version`: u32 (unsigned int)

**Effects:** None.

**Semantics:**

Returns the minor version of the CommonWA spec implemented by the runtime environment.

This value must be equal to the version which the application targets to ensure all features working correctly.

**Errors:**

None

