let type = ./type.dhall

let wasmTypes = ./wasmtypes.dhall

in  { i32 = type::{ name = "i32", cRepr = "int", lowersTo = wasmTypes.i32 }
    , i64 = type::{ name = "i64", cRepr = "long", lowersTo = wasmTypes.i64 }
    , f32 = type::{ name = "f32", cRepr = "float", lowersTo = wasmTypes.f32 }
    , f64 = type::{ name = "f64", cRepr = "double", lowersTo = wasmTypes.f64 }
    , ptr = type::{ name = "pointer", cRepr = "*int", lowersTo = wasmTypes.i32 }
    , usize = type::{
      , name = "usize"
      , cRepr = "len_t"
      , lowersTo = wasmTypes.i32
      }
    }
