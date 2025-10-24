#include "rust/cxx.h"
#include <jsi/jsi.h>
#include <react/runtime/TimerManager.h>
#include <PlatformTimerRegistryImpl.h>

void install_timer_globals(facebook::jsi::Runtime &runtime);
