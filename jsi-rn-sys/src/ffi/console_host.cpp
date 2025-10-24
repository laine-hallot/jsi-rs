#include "rust/cxx.h"
#include <jsi/jsi.h>
#include <react/runtime/TimerManager.h>
#include <PlatformTimerRegistryImpl.h>

void install_timer_globals(facebook::jsi::Runtime &runtime)
{
  auto platformTimers = std::make_unique<facebook::react::PlatformTimerRegistryImpl>();
  auto* platformTimersPtr = platformTimers.get();
  auto timerManager = std::make_shared<facebook::react::TimerManager>(std::move(platformTimers));
  platformTimersPtr->setTimerManager(timerManager);
  timerManager->attachGlobals(runtime);
}
