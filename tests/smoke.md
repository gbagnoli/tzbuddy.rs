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
$ /bin/bash -c "XDG_CONFIG_HOME=$TESTDIR tzbuddy --tz UTC --save && XDG_CONFIG_HOME=$TESTDIR tzbuddy"
Saving config at * (glob)
*UTC (UTC)* (glob)
*UTC (UTC)* (glob)
```

## No Config
Test that --no-config ignores saved configuration.

```scrut
$ /bin/bash -c "XDG_CONFIG_HOME=$TESTDIR tzbuddy --tz UTC --save && XDG_CONFIG_HOME=$TESTDIR tzbuddy --no-config"
Saving config at * (glob)
*UTC (UTC)* (glob)
No timezones provided.
```
