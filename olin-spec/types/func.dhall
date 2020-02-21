let type = ./type.dhall

let err = ./error.dhall

let wasmTypes = ./wasmtypes.dhall

let utils = ./utils.dhall

in  { Type =
        { name : Text
        , desc : Text
        , params : List type.Type
        , return : type.Type
        , errors : List err.Type
        , effects : Text
        }
    , default =
        { params = [] : List type.Type
        , return = utils.void
        , errors = [] : List err.Type
        , effects = "None."
        }
    }
