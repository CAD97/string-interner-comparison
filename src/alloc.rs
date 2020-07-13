use serde::{Deserialize, Serialize};
use std::alloc::{GlobalAlloc, Layout, System};
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Event {
    Alloc { addr: usize, size: usize },
    Freed { addr: usize, size: usize },
    Point {},
}

pub struct Tracing {
    pub inner: System,
    pub active: AtomicBool,
}

impl Tracing {
    pub const fn new() -> Self {
        Self {
            inner: System,
            active: AtomicBool::new(false),
        }
    }

    pub fn set_active(&self, active: bool) {
        self.active.store(active, Ordering::SeqCst);
    }

    pub fn mark_point(&self) {
        self.write_ev(Event::Point {})
    }

    fn write_ev(&self, ev: Event) {
        let mut buf = [0u8; 1024];
        let mut cursor = Cursor::new(&mut buf[..]);
        serde_json::to_writer(&mut cursor, &ev).unwrap();
        let end = cursor.position() as usize;
        self.write(&buf[..end]);
        self.write(b"\n");
    }

    fn write(&self, s: &[u8]) {
        unsafe {
            // Yeah, libc::STDERR_FILENO doesn't exist on Windows, so just guess
            libc::write(2, s.as_ptr() as _, s.len() as _);
        }
    }
}

unsafe impl GlobalAlloc for Tracing {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = self.inner.alloc(layout);
        if self.active.load(Ordering::SeqCst) {
            self.write_ev(Event::Alloc {
                addr: res as _,
                size: layout.size(),
            });
        }
        res
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.active.load(Ordering::SeqCst) {
            self.write_ev(Event::Freed {
                addr: ptr as _,
                size: layout.size(),
            });
        }
        self.inner.dealloc(ptr, layout)
    }
}
