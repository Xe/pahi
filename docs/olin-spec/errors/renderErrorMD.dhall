let Prelude = https://prelude.dhall-lang.org/package.dhall

let error = ../types/error.dhall

let errors = ./package.dhall

let renderErrorMD
    : error.Type → Text
    =   λ(err : error.Type)
      → ''
        ## ${err.name}

        Return value: -${Prelude.Natural.show err.intVal}

        ${err.desc}''

let toList = Prelude.Text.concatMapSep "\n" error.Type renderErrorMD

let errList = toList errors

in  ''
    # Errors

    This document defines all of the error codes that may be returned by API functions.

    ${errList}
    ''
