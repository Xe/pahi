let type = ./type.dhall

let wasmTypes = ./wasmtypes.dhall

let string
    : Text → List type.Type
    =   λ(name : Text)
      → [ type::{
          , name = "${name}_base"
          , cRepr = "int *"
          , lowersTo = wasmTypes.u32
          }
        , type::{
          , name = "${name}_len"
          , cRepr = "size_t"
          , lowersTo = wasmTypes.u32
          }
        ]

let void = type::{ name = "void", cRepr = "void", lowersTo = wasmTypes.void }

let i32
    : Text → type.Type
    =   λ(name : Text)
      → type::{ name = name, cRepr = "int", lowersTo = wasmTypes.i32 }

let u32
    : Text → type.Type
    =   λ(name : Text)
      → type::{ name = name, cRepr = "unsigned int", lowersTo = wasmTypes.u32 }

let i64
    : Text → type.Type
    =   λ(name : Text)
      → type::{ name = name, cRepr = "long", lowersTo = wasmTypes.i64 }

let u64
    : Text → type.Type
    =   λ(name : Text)
      → type::{ name = name, cRepr = "unsigned long", lowersTo = wasmTypes.u64 }

in  { string = string, i32 = i32, u32 = u32, void = void, i64 = i64, u64 = u64 }
