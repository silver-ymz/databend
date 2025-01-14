---
title: TO_DATE
---

TO_DATE converts an expression to a date format. 

The function can accept one or two arguments. If given one argument, the function extracts a date from the string. If the argument is an integer, the function interprets the integer as the number of days before (for a negative number) or after (for a positive number) the Unix epoch (midnight on January 1, 1970). 

If given two arguments, the function converts the first string to a date based on the pattern specified in the second string. To format the date and time according to your desired representation, make use of specifiers and refer to the supported specifiers available at https://docs.rs/chrono/latest/chrono/format/strftime/index.html.

## Syntax

```sql
-- Convert a string or integer to a date
TO_DATE(<expr>)

-- Convert a string to a date using the given pattern
TO_DATE(<expr, expr>)
```

## Return Type

Returns a date in the format "YYYY-MM-DD".

## Examples

### Given a String Argument

```sql
SELECT TO_DATE('2022-01-02T01:12:00+07:00');

---
2022-01-02

SELECT TO_DATE('2022-01-02 03:25:02.868894');

---
2022-01-02

SELECT TO_DATE('2022-01-02 02:00:11');

---
2022-01-02

SELECT TO_DATE('2022-01-02T02:00:22');

---
2022-01-02

SELECT TO_DATE('2022-01-02');

---
2022-01-02
```

### Given an Integer Argument

```sql
SELECT TO_DATE(1);

---
1970-01-02

SELECT TO_DATE(-1);

---
1969-12-31
```

:::tip

Please note that a Date value ranges from 1000-01-01 to 9999-12-31. Databend would return an error if you run the following statement:

```sql
SELECT TO_DATE(9999999999999999999);
```
:::

### Given Two Arguments

```sql
SELECT TO_DATE('Month/Day/Year: 12/25/2022','Month/Day/Year: %m/%d/%Y');

---
2022-12-25
```