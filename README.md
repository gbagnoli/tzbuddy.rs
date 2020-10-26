Overview
=========

tzbuddy is a simple cli to visualize times in different timezones.
It displays the current hour as well past and future values.

```bash
$ tzbuddy --tz 'US/Pacific' --tz 'Asia/Tokyo' --tz 'Europe/Rome' -s 18
 US/Pacific  (PDT) Mon 12:29 26/10/2020 ·  4    5    6    7    8    9    10   11  | 12 |  13   14   15   16   17   18   19   20   21  
 Asia/Tokyo  (JST) Tue 04:29 27/10/2020 ·  20   21   22   23   0+   1+   2+   3+  | 4+|   5+   6+   7+   8+   9+   10+  11+  12+  13+ 
 Europe/Rome (CET) Mon 20:29 26/10/2020 ·  12   13   14   15   16   17   18   19  | 20 |  21   22   23   0+   1+   2+   3+   4+   5+
```

Install
========

TODO - use cargo though

Usage
=======

See `tzbuddy --help` for all available options. There is no configuration, so you probably want to create an alias.

