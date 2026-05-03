pub fn wheel_feature_delta(delta: i32) -> Option<i32> {
    match delta.cmp(&0) {
        std::cmp::Ordering::Greater => Some(1),
        std::cmp::Ordering::Less => Some(-1),
        std::cmp::Ordering::Equal => None,
    }
}

pub fn key_feature_delta(key: u32) -> Option<i32> {
    match key {
        0x26 => Some(1),
        0x28 => Some(-1),
        _ => None,
    }
}

pub fn adjusted_feature_value(current: u32, maximum: u32, delta: i32) -> u32 {
    (current as i32 + delta).clamp(0, maximum as i32) as u32
}

pub fn parse_feature_value(input: &str, maximum: u32) -> Option<u32> {
    let value = input.trim().parse::<u32>().ok()?;
    Some(value.min(maximum))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WriteThrottle {
    interval_ms: u64,
    last_sent_at_ms: Option<u64>,
    pending_value: Option<u32>,
}

impl WriteThrottle {
    pub const fn new(interval_ms: u64) -> Self {
        Self {
            interval_ms,
            last_sent_at_ms: None,
            pending_value: None,
        }
    }

    pub fn schedule(&mut self, now_ms: u64, value: u32) -> Option<u32> {
        match self.last_sent_at_ms {
            None => {
                self.last_sent_at_ms = Some(now_ms);
                self.pending_value = None;
                Some(value)
            }
            Some(last_sent_at_ms) if now_ms.saturating_sub(last_sent_at_ms) >= self.interval_ms => {
                self.last_sent_at_ms = Some(now_ms);
                self.pending_value = None;
                Some(value)
            }
            Some(_) => {
                self.pending_value = Some(value);
                None
            }
        }
    }

    pub fn tick(&mut self, now_ms: u64) -> Option<u32> {
        let pending_value = self.pending_value?;
        let last_sent_at_ms = self.last_sent_at_ms.unwrap_or(now_ms);
        if now_ms.saturating_sub(last_sent_at_ms) < self.interval_ms {
            return None;
        }

        self.pending_value = None;
        self.last_sent_at_ms = Some(now_ms);
        Some(pending_value)
    }

    pub fn force(&mut self, now_ms: u64, value: u32) -> u32 {
        self.pending_value = None;
        self.last_sent_at_ms = Some(now_ms);
        value
    }

    pub fn has_pending(&self) -> bool {
        self.pending_value.is_some()
    }
}

impl Default for WriteThrottle {
    fn default() -> Self {
        Self::new(140)
    }
}
