let error = ../types/error.dhall

in  error::{
    , name = "NotFoundError"
    , intVal = 4
    , desc =
        ''
        The requested resource is not found.
        ''
    }
