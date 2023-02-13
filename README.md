# HeadlessUI for Sycamore

This is a WIP port of HeadlessUI for Sycamore. It's very WIP with only a subset of components
currently implemented, with very little testing done aside from usage in my own web app. Now that
`Attributes` has been merged it works with the Sycamore master branch, but a crates.io release for
the feature isn't available yet.

# Progress

All "official" components have been implemented.
Testing is WIP, so far `Combobox`, `Listbox`, `Disclosure` and `Dialog` are getting automatically tested.

`Dialog` is known broken due to [a bug in sycamore](https://github.com/sycamore-rs/sycamore/issues/572).
A fix is waiting to be merged.