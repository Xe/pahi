let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let wasmTypes = ../types/wasmtypes.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let write =
      func::{
      , name = "write"
      , params = [ utils.i32 "level" ] # utils.string "text"
      , return = utils.void
      , errors = [] : List err.Type
      , effects =
          "Writes text (the memory range from text_base to text_base + text_len) to the environment-provided logger."
      , desc =
          ''
          `level` can be one of:

          - 1: Error
          - 3: Warning
          - 6: Info

          If `level` is not one of these, the result is implementation-defined.

          The text must be valid UTF-8, otherwise the behavior is implementation-defined.

          This function MUST always succeed.''
      }

in  show
      ns::{
      , name = "log"
      , desc = "Logging facilities for applications"
      , funcs = [ write ]
      }
