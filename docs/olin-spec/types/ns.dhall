let func = ./func.dhall

in  { Type = { name : Text, desc : Text, funcs : List func.Type }
    , default =
        { name = "unknown"
        , desc = "please fill in the desc field"
        , funcs = [] : List func.Type
        }
    }
