let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let len =
      func::{
      , name = "arg_len"
      , params = [] : List type.Type
      , return = utils.u32 "argc"
      , errors = [] : List err.Type
      , desc =
          "Returns the number of arguments passed to the current application, including the path to application binary itself."
      }

let at =
      func::{
      , name = "arg_at"
      , params = [ utils.u32 "i" ] # utils.mut_string "out"
      , errors = [ ../errors/InvalidArgumentError.dhall ]
      , desc =
          ''
          Writes the argument at position id to out.

          The result is truncated if the argument length is greater than that of out.

          Returns the number of bytes written, or InvalidArgumentError if id is greater than or equal to the total number of arguments.''
      }

in  show
      ns::{
      , name = "startup"
      , desc = "Obtain startup information about the current application."
      , funcs = [ at, len ]
      }
