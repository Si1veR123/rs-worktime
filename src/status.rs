use super::long_duration::LongDuration;

#[derive(Debug)]
pub struct Settings {
    pub work_time: LongDuration,
    pub short_break_time: LongDuration,
    pub long_break_time: LongDuration,
    pub long_break_cycles: usize
}

impl Default for Settings {
    fn default() -> Self {
        Self { work_time: LongDuration::new_minutes(25), short_break_time: LongDuration::new_minutes(5), long_break_time: LongDuration::new_minutes(30), long_break_cycles: 4 }
    }
}


#[derive(Debug)]
pub struct Status {
    pub time_completed_in_state: LongDuration,
    pub completed_pomodoros: usize,
    pub paused: bool,
    pub in_break: bool,
    pub hover_on_pause: bool
}

impl Status {
    pub fn initial() -> Self {
        Self { time_completed_in_state: LongDuration::new_seconds(0), completed_pomodoros: 0, paused: false, in_break: false, hover_on_pause: false }
    }

    fn in_long_break(&self, settings: &Settings) -> bool {
        if self.completed_pomodoros == 0 {
            return false
        }

        self.in_break & ((self.completed_pomodoros % settings.long_break_cycles) == 0)
    }

    pub fn completed_work_time(&self, settings: &Settings) -> LongDuration {
        let base = LongDuration::new_seconds(self.completed_pomodoros as u64 * settings.work_time.seconds());

        match self.in_break {
            true => return &base + &settings.work_time,
            false => return &base + &self.time_completed_in_state
        }
    }

    pub fn fraction_of_state(&self, settings: &Settings) -> f32 {
        match self.in_break {
            false => self.time_completed_in_state.seconds() as f32 / settings.work_time.seconds() as f32,
            true => {
                let break_time = match self.in_long_break(settings) {
                    true => settings.long_break_time.seconds() as f32,
                    false => settings.short_break_time.seconds() as f32,
                };

                self.time_completed_in_state.seconds() as f32 / break_time
            }
        }
    }

    pub fn remaining_time_in_state(&self, settings: &Settings) -> LongDuration {
        &match self.in_break {
            false => &settings.work_time - &self.time_completed_in_state,
            true => {
                let break_time = match self.in_long_break(settings) {
                    true => &settings.long_break_time,
                    false => &settings.short_break_time,
                };

                break_time - &self.time_completed_in_state
            }
        } + &LongDuration::new_minutes(1)
    }

    pub fn update(&mut self, settings: &Settings, delta: &LongDuration) {
        if self.paused {
            return;
        }

        self.time_completed_in_state = &self.time_completed_in_state + delta;

        if self.in_break {
            let break_time = match self.in_long_break(settings) {
                true => &settings.long_break_time,
                false => &settings.short_break_time
            };

            if self.time_completed_in_state.seconds() > break_time.seconds() {
                // break finished
                self.in_break = false;
                self.completed_pomodoros += 1;
                self.time_completed_in_state = LongDuration::new_seconds(0);
            }
        } else {
            if self.time_completed_in_state.seconds() > settings.work_time.seconds() {
                // work finished
                self.in_break = true;
                self.time_completed_in_state = LongDuration::new_seconds(0);
            }
        }
    }
}
