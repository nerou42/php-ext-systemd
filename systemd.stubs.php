<?php
const STATE_BUSERROR = 'BUSERROR';
const STATE_ERRNO = 'ERRNO';
const STATE_EXTEND_TIMEOUT_USEC = 'EXTEND_TIMEOUT_USEC';
const STATE_FDNAME = 'FDNAME';
const STATE_FDSTORE = 'FDSTORE';
const STATE_FDSTOREREMOVE = 'FDSTOREREMOVE';
const STATE_MAINPID = 'MAINPID';
const STATE_READY = 'READY';
const STATE_RELOADING = 'RELOADING';
const STATE_STATUS = 'STATUS';
const STATE_STOPPING = 'STOPPING';
const STATE_WATCHDOG = 'WATCHDOG';
const STATE_WATCHDOG_USEC = 'WATCHDOG_USEC';

function systemd_notify(bool $unset_environment, string $state, string $value): bool;

function systemd_watchdog_enabled(bool $unset_environment): int;
