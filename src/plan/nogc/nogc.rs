use std::sync::Mutex;
use ::util::heap::MonotonePageResource;
use ::util::alloc::bumpallocator::BumpAllocator;
use ::util::alloc::allocator::Allocator;
use libc::c_void;
use ::plan::Plan;

lazy_static! {
    pub static ref SPACE: Mutex<MonotonePageResource> = Mutex::new(MonotonePageResource::new());
}
pub type NoGCMutator<'a> = BumpAllocator<'a>;

pub struct NoGC{}

impl Plan for NoGC {
    fn gc_init(heap_size: usize) {
        let mut globl = SPACE.lock().unwrap();
        (*globl).init(heap_size);
    }

    fn bind_mutator(thread_id: usize) -> *mut c_void {
        Box::into_raw(Box::new(NoGCMutator::new(thread_id, &SPACE))) as *mut c_void
    }
}