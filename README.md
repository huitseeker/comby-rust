# comby-rust

Rewrite rules for [Rust](https://github.com/rust-lang/rust) code using the
[comby](https://comby.dev/) refactoring tool.

## Overview

This structural find-and-replace tool allows implementing rules with different
purposes:

- `templates.toml`:
  refactorings that are non-controversial and will require
  minimal manual supervision a posteriori. They should not change code
  semantics.
- `experimental-templates.toml`:
  refactorings that are a matter or style, or
  will require modifications in a large number of cases, but may still be
  useful.
- `nopanic.toml`:
  refactorings that remove panics (and therefore change
  semantics).

## Requirements and installation

  Please refer to the [comby installation and usage instructions](https://comby.dev/docs/get-started).

## Testing

See the `test/run.sh` script for templates and expected results.


## Licence

Copyright 2021 François Garillot

This software is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.

## Code of Conduct

Please note that this project is released with a [Contributor Code of
Conduct][coc]. By participating in this project you agree to abide by its
terms.

[coc]: https://github.com/huitseeker/comby-rust/blob/master/CODE_OF_CONDUCT.md
