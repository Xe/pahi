let ns = ./ns.dhall

let func = ./func.dhall

let type = ./type.dhall

let showFunc = ./renderFuncToMD.dhall

let Prelude = ../Prelude.dhall

let toList = Prelude.Text.concatMapSep "\n" func.Type showFunc

let show
    : ns.Type → Text
    =   λ(namespace : ns.Type)
      → ''
        # ${namespace.name}

        ${namespace.desc}

        ${toList namespace.funcs}
        ''

in  show
