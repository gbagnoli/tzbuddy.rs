# Feature Tests
Verify CLI features including sorting, formatting, and time manipulation.

## Sorting
```scrut
$ tzbuddy -z Europe/Rome -z America/New_York -z Asia/Tokyo "2024-01-01 12:00"
 Asia/Tokyo       (JST) Mon 21:00 01/01/2024 ·  16   17   18   19   20  | 21 |  22   23   00+  01+  02+  03+ 
 Europe/Rome      (CET) Mon 13:00 01/01/2024 ·  08   09   10   11   12  | 13 |  14   15   16   17   18   19  
 America/New_York (EST) Mon 07:00 01/01/2024 ·  02   03   04   05   06  | 07 |  08   09   10   11   12   13  
```

```scrut
$ tzbuddy -I -z Europe/Rome -z Asia/Tokyo -z America/New_York "2024-01-01 12:00"
 America/New_York (EST) Mon 07:00 01/01/2024 ·  02   03   04   05   06  | 07 |  08   09   10   11   12   13  
 Europe/Rome      (CET) Mon 13:00 01/01/2024 ·  08   09   10   11   12  | 13 |  14   15   16   17   18   19  
 Asia/Tokyo       (JST) Mon 21:00 01/01/2024 ·  16   17   18   19   20  | 21 |  22   23   00+  01+  02+  03+ 
```

```scrut
$ tzbuddy -z America/New_York -z Asia/Tokyo -z Europe/Rome -O "2024-01-01 12:00"
 America/New_York (EST) Mon 07:00 01/01/2024 ·  02   03   04   05   06  | 07 |  08   09   10   11   12   13  
 Asia/Tokyo       (JST) Mon 21:00 01/01/2024 ·  16   17   18   19   20  | 21 |  22   23   00+  01+  02+  03+ 
 Europe/Rome      (CET) Mon 13:00 01/01/2024 ·  08   09   10   11   12  | 13 |  14   15   16   17   18   19  
```

## Formatting
```scrut
$ tzbuddy -z UTC -H "2024-01-01 12:00"
  07   08   09   10   11  | 12 |  13   14   15   16   17   18  
```

```scrut
$ tzbuddy -z UTC -a -s 1 "2024-01-01 12:00"
 UTC (UTC) Mon 12:00 pm 01/01/2024 ·   1 pm  
```

```scrut
$ tzbuddy -z UTC -s 3 "2024-01-01 12:00"
 UTC (UTC) Mon 12:00 01/01/2024 · | 12 |  13   14  
```

## Functional
```scrut
$ tzbuddy -L
Africa
America
* (glob+)
Asia
* (glob+)
Europe
* (glob+)
UTC
* (glob+)
```

```scrut
$ tzbuddy -z UTC "2024-01-01 12:00"
 UTC (UTC) Mon 12:00 01/01/2024 ·  07   08   09   10   11  | 12 |  13   14   15   16   17   18  
```
