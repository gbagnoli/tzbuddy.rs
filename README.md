Overview
=========

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

Use cargo for now. Binary distribution will come later. Also, this create
require nightly builds as of 27/10/2020

```
cargo install tzbuddy
```

Usage
=======

See `tzbuddy --help` for all available options. There is no configuration, so
you probably want to create an alias in your shell.

Development
===========

there are git hooks one can use to automatically run checks before commit

```
ln -s $(pwd)/hooks/pre-commit.sh .git/hooks/pre-commit
ln -s $(pwd)/hooks/pre-push.sh .git/hooks/pre-push
```

to release:

* bump the versio in Cargo.toml
* run `cargo release --dry-run -vv`
* run `cargo release`
