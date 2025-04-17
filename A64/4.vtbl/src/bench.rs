use std::io::Stdout;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Timer {
    time_start: u128,
    loops: usize,
    count: usize,
    info: String,
    stdout: Stdout,
}

impl Iterator for Timer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        print!("\r[{:3}/{:3}] ", self.count, self.loops);
        let _ = self.stdout.flush();

        if self.count < self.loops {
            Some(self.count)
        } else {
            None
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let total = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            - self.time_start;

        println!(
            "{:15}  Total {:>6} ms  Avg {:>6}ms",
            self.info,
            total,
            total / (self.loops as u128)
        );
    }
}

impl Timer {
    pub fn iter(&mut self) -> &mut Timer {
        return IntoIterator::into_iter(self);
    }
}

pub fn init(loops: usize, info: String) -> Timer {
    Timer {
        time_start: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        loops: loops,
        count: 0,
        info: info,
        stdout: std::io::stdout(),
    }
}
