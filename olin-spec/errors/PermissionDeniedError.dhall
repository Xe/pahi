let error = ../types/error.dhall

in  error::{
    , name = "PermissionDeniedError"
    , intVal = 3
    , desc =
        ''
        The application is trying to perform an operation that is not allowed by the security policy.
        ''
    }
