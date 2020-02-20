let type = ./type.dhall

let err = ./error.dhall

let func = ./func.dhall

let showType = ./renderTypeToMD.dhall

let Prelude = ../Prelude.dhall

let eq = Prelude.Natural.equal

let toList = Prelude.Text.concatMapSep "\n" type.Type showType

let showErrorInList
    : err.Type → Text
    = λ(er : err.Type) → "${er.name}"

let toErrList = Prelude.Text.concatMapSep "\n" err.Type showErrorInList

let render
    : func.Type → Text
    =   λ(fn : func.Type)
      → let paramLen = List/length type.Type fn.params

        let paramList = if eq paramLen 0 then "None" else toList fn.params

        let errLen = List/length err.Type fn.errors

        let errList = if eq errLen 0 then "None" else toErrList fn.errors

        in  ''
            ## ${fn.name}

            **Parameters:**

            ${paramList}

            **Returns:**

            ${showType fn.return}

            **Effects:** ${fn.effects}

            **Semantics:**

            ${fn.desc}

            **Errors:**

            ${errList}
            ''

in  render
