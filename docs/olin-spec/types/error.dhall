{ Type = { name : Text, intVal : Natural, desc : Text }
, default =
    { name = "UnknownError"
    , intVal = 1
    , desc =
        ''
        Represents an error that cannot be represented by any other error code.

        Implementations should provide details about this error in logs.
        ''
    }
}
