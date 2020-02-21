let error = ../types/error.dhall

in  error::{
    , name = "InvalidArgumentError"
    , intVal = 2
    , desc =
        ''
        One or more arguments passed to the function is invalid.

        Any state of the application must not be changed if this error is returned.
        ''
    }
