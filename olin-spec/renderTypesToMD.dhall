let utils = ./types/utils.dhall

let type = ./types/type.dhall

let showWasmType = ./types/showWasmType.dhall

let show
    : type.Type → Text
    =   λ(typ : type.Type)
      → ''
        ## ${typ.name}

        **WASM Type**: ${showWasmType typ.lowersTo}

        **C representation**: ${typ.cRepr}

        TODO: details here
        ''

let Prelude = ./Prelude.dhall

let toList = Prelude.Text.concatMapSep "\n" type.Type show

let types =
      [ utils.ptr "pointer"
      , utils.i32 "i32"
      , utils.u32 "u32"
      , utils.i64 "i64"
      , utils.u64 "u64"
      , utils.void
      ]

in  ''
    # Types

    This is an exhaustive list of all of the types that the Olin ABI uses.

    ${toList types}
    ''
