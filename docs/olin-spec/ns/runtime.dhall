let ns = ../types/ns.dhall

let func = ../types/func.dhall

let type = ../types/type.dhall

let err = ../types/error.dhall

let wasmTypes = ../types/wasmtypes.dhall

let utils = ../types/utils.dhall

let show = ../types/renderNSToMD.dhall

let exit =
      func::{
      , name = "exit"
      , params = [ utils.i32 "status" ]
      , effects = "Stops VM execution."
      , desc =
          "Immediately exits the VM and passes the status code passed to this function up the stack."
      }

let msleep =
      func::{
      , name = "msleep"
      , params = [ utils.u32 "ms" ]
      , effects = "Sleeps for the given ms milliseconds."
      , desc =
          "If the environment does not support sleeping, calling this results in a fatal error."
      }

let name =
      func::{
      , name = "name"
      , params = utils.mut_string "out"
      , return = utils.i32 "len"
      , errors = [] : List err.Type
      , desc =
          ''
          Writes the name of the current runtime environment to out and returns the number of bytes written.

          The name must not be longer than 32 bytes and must be valid UTF-8.''
      }

let spec_major =
      func::{
      , name = "spec_major"
      , params = [] : List type.Type
      , return = utils.u32 "spec_major_version"
      , errors = [] : List err.Type
      , desc =
          ''
          Returns the major version of the CommonWA spec implemented by the runtime environment.

          This value must be equal to the version which the application targets to ensure all features working correctly.''
      }

let spec_minor =
      func::{
      , name = "spec_minor"
      , params = [] : List type.Type
      , return = utils.u32 "spec_minor_version"
      , errors = [] : List err.Type
      , desc =
          ''
          Returns the minor version of the CommonWA spec implemented by the runtime environment.

          This value must be equal to the version which the application targets to ensure all features working correctly.''
      }

in  show
      ns::{
      , name = "runtime"
      , desc = "Control and obtain information about the runtime environment."
      , funcs = [ exit, msleep, name, spec_major, spec_minor ]
      }
