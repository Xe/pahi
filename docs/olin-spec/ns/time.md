# time

Get the current time (offset from Jan 1, 1970) from the host.

## now

**Parameters:**

None

**Returns:**

- `unix_time`: u64 (unsigned long)

**Effects:** Queries the current time from the system.

**Semantics:**

Returns the number of seconds elapsed since Jan 1, 1970, UTC. This offset will always be in UTC.

**Errors:**

None

