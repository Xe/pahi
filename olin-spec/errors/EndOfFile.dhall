let error = ../types/error.dhall

in  error::{
    , name = "EndOfFile"
    , intVal = 5
    , desc =
        ''
        The requested resource has no more data to be read.
        ''
    }
