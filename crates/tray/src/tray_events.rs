pub const DEFAULT_DOUBLE_CLICK_MS: u64 = 500;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayAction {
    OpenWindow,
}

#[derive(Debug, Default)]
pub struct TrayClickTracker {
    last_left_up_at_ms: Option<u64>,
}

impl TrayClickTracker {
    pub fn left_up_at(&mut self, now_ms: u64) -> Option<TrayAction> {
        let action = self
            .last_left_up_at_ms
            .filter(|previous_ms| now_ms.saturating_sub(*previous_ms) <= DEFAULT_DOUBLE_CLICK_MS)
            .map(|_| TrayAction::OpenWindow);

        self.last_left_up_at_ms = if action.is_some() { None } else { Some(now_ms) };
        action
    }
}
