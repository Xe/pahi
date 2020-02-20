let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let showWasmType = ../types/showWasmType.dhall

let rand_for
    : type.Type → func.Type
    =   λ(typ : type.Type)
      → func::{
        , name = "${showWasmType typ.lowersTo}"
        , params = [] : List type.Type
        , return = typ
        , errors = [] : List err.Type
        , effects =
            "Consumes cryptographic entropy, might block while that is refilling."
        , desc = "A sufficiently random ${typ.name} value is returned."
        }

in  show
      ns::{
      , name = "random"
      , desc =
          "Non-cryptographic randomness, but it's fed with cryptographic randomness anyways."
      , funcs =
        [ rand_for (utils.i32 "result")
        , rand_for (utils.u32 "result")
        , rand_for (utils.i64 "result")
        , rand_for (utils.u64 "result")
        ]
      }
