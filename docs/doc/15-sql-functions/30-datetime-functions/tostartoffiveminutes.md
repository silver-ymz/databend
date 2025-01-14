---
title: TO_START_OF_FIVE_MINUTES
---

Rounds down a date with time (timestamp/datetime) to the start of the five-minute interval.
## Syntax

```sql
TO_START_OF_FIVE_MINUTES(<expr>)
```

## Arguments

| Arguments | Description |
|-----------|-------------|
| `<expr>`  | timestamp   |

## Return Type

`TIMESTAMP`, returns date in “YYYY-MM-DD hh:mm:ss.ffffff” format.

## Examples

```sql
SELECT to_start_of_five_minutes(now());
+---------------------------------+
| to_start_of_five_minutes(now()) |
+---------------------------------+
| 2022-10-15 03:15:00.000000      |
+---------------------------------+

SELECT to_start_of_five_minutes(to_timestamp(1630812366));
+----------------------------------------------------+
| to_start_of_five_minutes(to_timestamp(1630812366)) |
+----------------------------------------------------+
| 2021-09-05 03:25:00.000000                         |
+----------------------------------------------------+
```
