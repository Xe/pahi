let inter = < none | pahi | cwa >

let test =
      { Type = { interpreter : inter, exitsWith : Natural, fname : Text }
      , default = { interpreter = inter.none, exitsWith = 1, fname = "invalid" }
      }

in  { cases =
      [ test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "allyourlogs.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "allyourargs.wasm"
        }
      , test::{
        , interpreter = inter.pahi
        , exitsWith = 0
        , fname = "allyourargs.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "allyourlogs.wasm"
        }
      , test::{ interpreter = inter.cwa, exitsWith = 0, fname = "shaman.wasm" }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "olinfetch.wasm"
        }
      , test::{
        , interpreter = inter.cwa
        , exitsWith = 0
        , fname = "olin-tests.wasm"
        }
      ]
    , env = [ "MAGIC_CONCH=yes" ]
    }
