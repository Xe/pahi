let type = ./type.dhall

let showWasmType = ./showWasmType.dhall

let Prelude = ../Prelude.dhall

let render
    : type.Type → Text
    =   λ(typ : type.Type)
      → "- `${typ.name}`: ${showWasmType typ.lowersTo} (${typ.cRepr})"

in  render
