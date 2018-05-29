use std::time;

pub struct Stopwatch {
    created_at: time::SystemTime,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            created_at: time::SystemTime::now(),
        }
    }

    pub fn elapsed_time(&self) -> Result<time::Duration, time::SystemTimeError> {
        self.created_at.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn basic_behavior() {
        let timer = Stopwatch::new();
        let delay = time::Duration::from_millis(100);
        thread::sleep(delay);
        assert!(timer.elapsed_time().unwrap() >= delay);
    }
}
