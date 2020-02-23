let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let get =
      func::{
      , name = "get"
      , params = utils.string "key" # utils.mut_string "value"
      , return = utils.i32 "result"
      , errors = [ ../errors/NotFoundError.dhall ]
      , effects = "Looks up key in the environment for the process"
      , desc =
          ''
          Requests a value from the environment, like environment variables for
          common applications. `key` indicate the buffer
          containing the key for the information we want to request.
          `value` indicate the buffer in which the value
          will be written.

          Returns:
          - `NotFoundError` if the key does not exist
          - the number of bytes written to the value buffer if the key exists and the buffer
            is big enough
          - the needed size for the value buffer if it was not big enough. No data is written,
            and the caller must check if that return value is larger than `value.len` to
            reallocate and retry.''
      }

in  show
      ns::{
      , name = "env"
      , desc =
          "Get configuration information (like database credentials) from the host."
      , funcs = [ get ]
      }
