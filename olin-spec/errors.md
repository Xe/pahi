# Errors

This document defines all of the error codes that may be returned by API functions.

## UnknownError

Return value: -1

Represents an error that cannot be represented by any other error code.

Implementations should provide details about this error in logs.

## InvalidArgumentError

Return value: -2

One or more arguments passed to the function is invalid.

Any state of the application must not be changed if this error is returned.

## PermissionDeniedError

Return value: -3

The application is trying to perform an operation that is not allowed by the security policy.

## NotFoundError

Return value: -4

The requested resource is not found.

## EndOfFile

Return value: -5

The requested resource has no more data to be read.

