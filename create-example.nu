def main [name: string] {
    cp examples/.base $"examples/($name)" -r

    open $"examples/($name)/Cargo.toml"
    | upsert package.name $"example-($name)"
    | save $"examples/($name)/Cargo.toml"

    open Cargo.toml
    | upsert workspace.members {|row index| $row.workspace.members | append $"examples/($name)"}
    | save Cargo.toml
}