# log

Logging facilities for applications

## write

**Parameters:**

- `level`: i32 (int)
- `text_base`: u32 (int *)
- `text_len`: u32 (size_t)

**Returns:**

- `void`: void (void)

**Effects:** Writes text (the memory range from text_base to text_base + text_len) to the environment-provided logger.

**Semantics:**

`level` can be one of:

- 1: Error
- 3: Warning
- 6: Info

If `level` is not one of these, the result is implementation-defined.

The text must be valid UTF-8, otherwise the behavior is implementation-defined.

This function MUST always succeed.

**Errors:**

None

