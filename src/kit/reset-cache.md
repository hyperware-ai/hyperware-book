# `kit reset-cache`

The `kit reset-cache` command clears the cache where `kit` stores Kinode core binaries, logs, etc.

## Discussion

In general, `kit reset-cache` should not need to be used.
There are occasionally cases where the `kit` cache gets corrupted.
If seeing confusing and difficult to explain behavior from `kit`, a `kit reset-cache` won't hurt.

## Arguments

```
$ kit reset-cache --help
Reset kit cache (Kinode core binaries, logs, etc.)

Usage: kit reset-cache

Options:
  -h, --help  Print help
```
