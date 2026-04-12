# Smoke Test
Verify basic CLI functionality.

## Version
```scrut
$ tzbuddy --version
tzbuddy * (glob)
```

## Configuration Persistence
Test that saving a configuration actually works and is loaded in subsequent calls.

```scrut
$ /bin/bash -c 'XDG_CONFIG_HOME=$TESTDIR tzbuddy --tz UTC --save "2024-01-01 12:00" && XDG_CONFIG_HOME=$TESTDIR tzbuddy "2024-01-01 12:00"'
Saving config at * (glob)
 UTC (UTC) Mon 12:00 01/01/2024 ·  07   08   09   10   11  | 12 |  13   14   15   16   17   18  
 UTC (UTC) Mon 12:00 01/01/2024 ·  07   08   09   10   11  | 12 |  13   14   15   16   17   18  
```

## No Config
Test that --no-config ignores saved configuration.

```scrut
$ /bin/bash -c 'XDG_CONFIG_HOME=$TESTDIR tzbuddy --tz UTC --save "2024-01-01 12:00" && XDG_CONFIG_HOME=$TESTDIR tzbuddy --no-config "2024-01-01 12:00"'
Saving config at * (glob)
 UTC (UTC) Mon 12:00 01/01/2024 ·  07   08   09   10   11  | 12 |  13   14   15   16   17   18  
No timezones provided.
```

## No duplicates
Test that there are no duplicates in the output even the same timezone is present on the commandline or config

```scrut
$ tzbuddy -z Europe/Rome -z Europe/Rome "2024-01-01 12:00"
 Europe/Rome (CET) Mon 13:00 01/01/2024 ·  08   09   10   11   12  | 13 |  14   15   16   17   18   19  
```

## Test DST

```scrut
$ tzbuddy -z Europe/Rome "2026-03-29 01:00"
 Europe/Rome (CEST) Sun 03:00 29/03/2026 ·  21-  22-  23-  00   01  | 03 |  04   05   06   07   08   09  
```

```scrut
$ tzbuddy -z Europe/Rome "2026-03-29 00:00"
 Europe/Rome (CET) Sun 01:00 29/03/2026 ·  20-  21-  22-  23-  00  | 01 |  03   04   05   06   07   08  
```
