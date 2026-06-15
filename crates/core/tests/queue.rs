use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use dell_controller_core::ddc::{
    CommandQueue, DdcBackend, DdcError, RetryPolicy, VcpCode, VcpFeature,
};
use parking_lot::Mutex as ParkingMutex;

#[derive(Clone, Default)]
struct RecordingBackend {
    calls: Arc<Mutex<Vec<String>>>,
    failures_before_success: Arc<Mutex<usize>>,
}

#[derive(Default)]
struct SharedSerialBackend {
    gate: ParkingMutex<()>,
    active: AtomicBool,
    overlapped: AtomicBool,
}

impl DdcBackend for SharedSerialBackend {
    fn synchronization_lock(&self) -> Option<&ParkingMutex<()>> {
        Some(&self.gate)
    }

    fn get_vcp_feature(&self, code: VcpCode) -> Result<VcpFeature, DdcError> {
        if self.active.swap(true, Ordering::SeqCst) {
            self.overlapped.store(true, Ordering::SeqCst);
        }
        thread::sleep(Duration::from_millis(20));
        self.active.store(false, Ordering::SeqCst);
        Ok(VcpFeature {
            code,
            current: 1,
            maximum: 100,
        })
    }

    fn set_vcp_feature(&self, _code: VcpCode, _value: u32) -> Result<(), DdcError> {
        Ok(())
    }
}

impl RecordingBackend {
    fn new(failures_before_success: usize) -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            failures_before_success: Arc::new(Mutex::new(failures_before_success)),
        }
    }

    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}

impl DdcBackend for RecordingBackend {
    fn get_vcp_feature(&self, code: VcpCode) -> Result<VcpFeature, DdcError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("get:{:02X}", code.get()));
        let mut remaining = self.failures_before_success.lock().unwrap();
        if *remaining > 0 {
            *remaining -= 1;
            return Err(DdcError::Transient("busy".into()));
        }
        Ok(VcpFeature {
            code,
            current: 75,
            maximum: 100,
        })
    }

    fn set_vcp_feature(&self, code: VcpCode, value: u32) -> Result<(), DdcError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("set:{:02X}:{value}", code.get()));
        Ok(())
    }
}

#[test]
fn retries_transient_get_failures_and_preserves_order() {
    let backend = RecordingBackend::new(2);
    let queue = CommandQueue::new(
        backend.clone(),
        RetryPolicy {
            attempts: 3,
            delay_ms: 0,
        },
    );

    let feature = queue.get(VcpCode::new(0x10)).expect("eventual read");
    queue.set(VcpCode::new(0x12), 80).expect("write succeeds");

    assert_eq!(feature.current, 75);
    assert_eq!(
        backend.calls(),
        vec!["get:10", "get:10", "get:10", "set:12:80"]
    );
}

#[test]
fn queues_for_the_same_backend_share_backend_serialization() {
    let backend = Arc::new(SharedSerialBackend::default());
    let first = Arc::new(CommandQueue::new(
        backend.clone(),
        RetryPolicy {
            attempts: 1,
            delay_ms: 0,
        },
    ));
    let second = Arc::new(CommandQueue::new(
        backend.clone(),
        RetryPolicy {
            attempts: 1,
            delay_ms: 0,
        },
    ));

    let first_thread = {
        let first = first.clone();
        thread::spawn(move || first.get(VcpCode::new(0x10)).expect("first read"))
    };
    let second_thread = thread::spawn(move || second.get(VcpCode::new(0x12)).expect("second read"));

    first_thread.join().expect("first thread completes");
    second_thread.join().expect("second thread completes");

    assert!(!backend.overlapped.load(Ordering::SeqCst));
}
