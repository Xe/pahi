let Prelude =
      https://prelude.dhall-lang.org/package.dhall sha256:c1b3fc613aabfb64a9e17f6c0d70fe82016a030beedd79851730993e9083fde2

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
