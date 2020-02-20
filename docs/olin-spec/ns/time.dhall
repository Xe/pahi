let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let now =
      func::{
      , name = "now"
      , params = [] : List type.Type
      , return = utils.u64 "unix_time"
      , errors = [] : List err.Type
      , effects = "Queries the current time from the system."
      , desc =
          "Returns the number of seconds elapsed since Jan 1, 1970, UTC. This offset will always be in UTC."
      }

in  show
      ns::{
      , name = "time"
      , desc = "Get the current time (offset from Jan 1, 1970) from the host."
      , funcs = [ now ]
      }
