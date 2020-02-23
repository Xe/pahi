let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let open =
      func::{
      , name = "open"
      , params = utils.string "url"
      , return = utils.i32 "fd"
      , errors =
        [ ../errors/PermissionDeniedError.dhall
        , ../errors/NotFoundError.dhall
        , ../errors/InvalidArgumentError.dhall
        ]
      , effects =
          "Depends on the scheme of the url, but generally creates a file descriptor in the process environment."
      , desc =
          ''
          Opens the resource specified by url.

          Returns the resource id on success, or the error code on failure.

          The URL format is as defined [here](../schemes)''
      }

let close =
      func::{
      , name = "close"
      , params = [ utils.i32 "fd" ]
      , return = utils.void
      , errors = [] : List err.Type
      , effects = "Closes the resource at the given descriptor"
      , desc =
          ''
          Closes the resource specified by id.

          Calling this on an invalid resource id is a fatal error.''
      }

let read =
      func::{
      , name = "read"
      , params = [ utils.i32 "fd" ] # utils.string "data"
      , return = utils.i32 "len"
      , errors =
        [ ../errors/InvalidArgumentError.dhall
        , ../errors/UnknownError.dhall
        , ../errors/EndOfFile.dhall
        ]
      , effects =
          "Reads to data (the memory range from data_base to data_base + data_len) from the resource at the given descriptor"
      , desc =
          "Returns the actual bytes written on success, or the error on failure."
      }

let write =
      func::{
      , name = "write"
      , params = [ utils.i32 "fd" ] # utils.string "data"
      , return = utils.i32 "len"
      , errors =
        [ ../errors/InvalidArgumentError.dhall, ../errors/UnknownError.dhall ]
      , effects =
          "Writes data (the memory range from data_base to data_base + data_len) to the resource at the given descriptor"
      , desc =
          "Returns the actual bytes written on success, or the error on failure."
      }

in  show
      ns::{
      , name = "resource"
      , desc = "Operations related to readable/writeable resources."
      , funcs = [ close, open, read, write ]
      }
