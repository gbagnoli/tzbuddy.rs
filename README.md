Overview
=========

![build](https://github.com/gbagnoli/tzbuddy.rs/workflows/build/badge.svg)

tzbuddy is a simple cli to visualize times in different timezones.
It displays the current hour as well past and future values.

```bash
$ tzbuddy --tz Europe/Rome --tz Europe/Dublin --tz "US/Eastern" --tz "US/Pacific" --tz "Asia/Tokyo"
 Asia/Tokyo    (JST) Wed 05:17 28/10/2020 ·  00+  01+  02+  03+  04+ | 05+|  06+  07+  08+  09+  10+  11+
 Europe/Rome   (CET) Tue 21:17 27/10/2020 ·  16   17   18   19   20  | 21 |  22   23   00+  01+  02+  03+
 Europe/Dublin (GMT) Tue 20:17 27/10/2020 ·  15   16   17   18   19  | 20 |  21   22   23   00+  01+  02+
 US/Eastern    (EDT) Tue 16:17 27/10/2020 ·  11   12   13   14   15  | 16 |  17   18   19   20   21   22
 US/Pacific    (PDT) Tue 13:17 27/10/2020 ·  08   09   10   11   12  | 13 |  14   15   16   17   18   19
```

Install
========

Binaries are provided for [each tagged
releases](https://github.com/gbagnoli/tzbuddy.rs/releases).

In alternative one install using cargo to build from source

```
cargo install tzbuddy
```

On macOS, with brew, you can use the [brew tap
repository](https://github.com/gbagnoli/homebrew-tzbuddy)

```
$ brew tap gbagnoli/tzbuddy
$ brew install tzbuddy
```

Usage
=======

See `tzbuddy --help` for all available options.

Prior to version `0.3`, `tzbuddy` did not support configuration files, and you
had to create an alias on the shell.

Since version `0.3` there is support for configuration. Simply pass `--save` and
`tzbuddy` will remember flags when running without. Eventual options on the
commandline takes precedence. `--no-config` will skip loading the config.
Running `--save` without any other option will save an empty config.

Development
===========

there are git hooks one can use to automatically run checks before commit

```
ln -s $(pwd)/hooks/pre-commit.sh .git/hooks/pre-commit
ln -s $(pwd)/hooks/pre-push.sh .git/hooks/pre-push
```

to release:

* install cargo release `cargo install cargo-release`
* bump the version in Cargo.toml
* make sure to build first so that Cargo.lock gets updated (`cargo build`)
* commit the Cargo.toml/Cargo.lock files
* run `cargo release -v`
* run `cargo release -v --execute` to push the release
