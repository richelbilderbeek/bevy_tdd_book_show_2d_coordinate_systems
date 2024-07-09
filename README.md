# bevy_tdd_book_show_2d_coordinat_systems

[![Check build](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_build.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_build.yaml)
[![Check links](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_links.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_links.yaml)
[![Check markdown](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_markdown.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_markdown.yaml)
[![Check Rust style](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_rust_style.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_rust_style.yaml)
[![Check spelling](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_spelling.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/check_spelling.yaml)
[![Measure code coverage](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/measure_codecov.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/actions/workflows/measure_codecov.yaml)
[![codecov](https://codecov.io/gh/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems/graph/badge.svg?token=XAVFZYDQKZ)](https://codecov.io/gh/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems)

'Hello world' code for [https://github.com/richelbilderbeek/bevy_tdd_book](https://github.com/richelbilderbeek/bevy_tdd_book)

First, we follow the steps of the [Bevy setup](https://bevyengine.org/learn/quick-start/getting-started/setup/):

```bash
git clone https://github.com/richelbilderbeek/bevy_tdd_book_show_2d_coordinat_systems
cd bevy_tdd_book_show_2d_coordinat_systems
cargo init
cargo add bevy
cargo add bevy -F dynamic_linking
```

To [Cargo.toml](Cargo.toml) add:

```bash
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

To [.cargo/config.toml](.cargo/config.toml) add:

```bash
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

Code:

```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }
}
```

## Files used by continuous integration scripts

Filename                                  |Descriptions
------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------
[mlc_config.json](mlc_config.json)        |Configuration of the link checker, use `markdown-link-check --config mlc_config.json --quiet docs/**/*.md` to do link checking locally
[.spellcheck.yml](.spellcheck.yml)        |Configuration of the spell checker, use `pyspelling -c .spellcheck.yml` to do spellcheck locally
[.wordlist.txt](.wordlist.txt)            |Whitelisted words for the spell checker, use `pyspelling -c .spellcheck.yml` to do spellcheck locally
[.markdownlint.jsonc](.markdownlint.jsonc)|Configuration of the markdown linter, use `markdownlint "**/*.md"` to do markdown linting locally. The name of this file is a default name.
[.markdownlintignore](.markdownlintignore)|Files ignored by the markdown linter, use `markdownlint "**/*.md"` to do markdown linting locally. The name of this file is a default name.

## References

* [Bevy setup](https://bevyengine.org/learn/quick-start/getting-started/setup/)
