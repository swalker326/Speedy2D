/*
 *  Copyright 2021 QuantumBadger
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use crate::error::{BacktraceError, ErrorMessage};
#[cfg(target_arch = "wasm32")]
use crate::web::{WebPerformance, WebWindow};

pub struct Timer
{
    clock: TimeClock,
    start: TimeInstant
}

impl Timer
{
    #[inline]
    pub fn new() -> Result<Self, BacktraceError<ErrorMessage>>
    {
        let clock = TimeClock::new()?;
        let start = clock.now();

        Ok(Self { clock, start })
    }

    #[inline]
    pub fn secs_elapsed(&self) -> f64
    {
        self.clock.secs_elapsed_since(&self.start)
    }
}

#[derive(Clone)]
struct TimeClock
{
    #[cfg(target_arch = "wasm32")]
    performance: WebPerformance
}

impl TimeClock
{
    pub fn new() -> Result<Self, BacktraceError<ErrorMessage>>
    {
        #[cfg(target_arch = "wasm32")]
        return Ok(Self {
            performance: WebWindow::new()?.performance()?
        });

        #[cfg(not(target_arch = "wasm32"))]
        return Ok(Self {});
    }

    #[inline]
    pub fn now(&self) -> TimeInstant
    {
        #[cfg(target_arch = "wasm32")]
        return TimeInstant {
            value: self.performance.now()
        };

        #[cfg(not(target_arch = "wasm32"))]
        return TimeInstant {
            value: Instant::now()
        };
    }

    #[inline]
    pub fn secs_elapsed_since(&self, start: &TimeInstant) -> f64
    {
        #[cfg(target_arch = "wasm32")]
        return (self.now().value - start.value) / 1000.0;

        #[cfg(not(target_arch = "wasm32"))]
        return start.value.elapsed().as_secs_f64();
    }
}

struct TimeInstant
{
    #[cfg(target_arch = "wasm32")]
    value: f64,

    #[cfg(not(target_arch = "wasm32"))]
    value: Instant
}
