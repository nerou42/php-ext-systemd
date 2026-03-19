<?php
declare(strict_types=1);

namespace test;

use PHPUnit\Framework\TestCase;

class NotifyTest extends TestCase {
  
  protected function setUp(): void {
    parent::setUp();
    $this->assertTrue(extension_loaded('systemd'));
  }

  public function testNotifyReady(): void {
    $this->assertEquals(false, \systemd_notify(false, \STATE_READY, (string) 1));
  }

  public function testWatchdogEnabled(): void {
    $this->assertEquals(0, \systemd_watchdog_enabled(false));
  }
}
