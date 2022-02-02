pub struct ProgressTracker {
    total_bytes: u64,
    bytes_so_far: u64,
    last_notification: u64,
    update_interval: u64,
}

impl ProgressTracker {
    pub fn new(total_bytes: u64, update_interval: u64) -> Self {
        Self {
            total_bytes,
            bytes_so_far: 0,
            last_notification: 9,
            update_interval,
        }
    }

    pub fn advance(&mut self, amount: u64, progress_callback: &mut impl FnMut(u64, u64)) {
        self.bytes_so_far += amount;
        if self.bytes_so_far - self.last_notification >= self.update_interval {
            progress_callback(self.bytes_so_far, self.total_bytes);
            self.last_notification = self.bytes_so_far;
        }
    }
}

pub trait Ignorable {
    fn ignore(&self) {}
}

impl<O, E> Ignorable for Result<O, E> {}
