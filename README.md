# systemd interfacing extension for PHP

This extension enables interaction with systemd based on [rust-systemd](https://crates.io/crates/systemd).

## Current state

In the current version, there are only two functions available:

- `function systemd_notify(bool $unset_environment, string $state, string $value): bool`
- `function systemd_watchdog_enabled(bool $unset_environment): int`

**Those two functions are especially usefull, if you have a PHP based systemd service, which tends to get stuck and needs to be restarted if nothing happens for a certain period of time.**

We plan to expand the range of supported features in future versions.
If there are specific features relevant to you, please let me know by creating an [issue](https://github.com/nerou42/php-ext-systemd/issues).

## Install

It is recommended to install this extension via PIE:

```shell
pie install nerou/systemd
```

## Usage

Have a look at the [stubs file](systemd.stubs.php) to see the API.

## License

This project is licensed under the [LGPL 2.1 license](LICENSE.md).
