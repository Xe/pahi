let wasmTypes = ./wasmtypes.dhall

let show
    : wasmTypes → Text
    =   λ(typ : wasmTypes)
      → merge
          { i32 = "i32"
          , i64 = "i64"
          , f32 = "f32"
          , f64 = "f64"
          , void = "void"
          , u32 = "u32"
          , u64 = "u64"
          }
          typ

in  show
