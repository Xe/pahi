let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let fd
    : Text → Bool → func.Type
    =   λ(name : Text)
      → λ(errSink : Bool)
      → let sinkName = if errSink then "error output" else "${name}put"

        in  func::{
            , name = "get_std${name}"
            , params = [] : List type.Type
            , return = utils.u32 "fd"
            , errors = [] : List err.Type
            , effects = "Creates a file descriptor in the process environment"
            , desc =
                "Returns a read-only file descriptor pointing to the semantic standard ${sinkName} of the WebAssembly VM."
            }

in  show
      ns::{
      , name = "io"
      , desc = "File descriptors for standard input and output."
      , funcs = [ fd "out" False, fd "in" False, fd "err" True ]
      }
