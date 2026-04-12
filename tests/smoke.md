# Smoke Test
Verify basic CLI functionality.

## Version
```scrut
$ tzbuddy --version
tzbuddy 0.5.2
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
