let wasmTypes = ./wasmtypes.dhall

in  { Type = { name : Text, cRepr : Text, lowersTo : wasmTypes }
    , default = {=}
    }
