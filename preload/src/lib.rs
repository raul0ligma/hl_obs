use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_void},
    sync::atomic::{AtomicBool, AtomicPtr, Ordering},
};

#[derive(Copy, Clone, Debug)]
enum Source {
    Statuses,
    Diffs,
    Fills,
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

type OpenFn = unsafe extern "C" fn(*const c_char, c_int, ...) -> c_int;

type OpenatFn = unsafe extern "C" fn(c_int, *const c_char, c_int, c_int) -> c_int;
type WriteFn = unsafe extern "C" fn(c_int, *const c_void, usize) -> isize;
type CloseFn = unsafe extern "C" fn(c_int) -> c_int;

static REAL_OPEN64: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

static REAL_OPENAT: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
static REAL_WRITE: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
static REAL_CLOSE: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

unsafe fn log_stderr(msg: &str) {
    // assuming we have it inited
    let real_fn: *mut () = REAL_WRITE.load(Ordering::Relaxed);
    if !real_fn.is_null() {
        let real_write: WriteFn = std::mem::transmute(real_fn);
        real_write(2, msg.as_ptr() as *const c_void, msg.len());
    }
}

#[ctor::ctor]
fn init() {
    unsafe {
        let real_openat = libc::dlsym(libc::RTLD_NEXT, b"openat\0".as_ptr() as *const _);
        REAL_OPENAT.store(real_openat as *mut (), Ordering::SeqCst);

        let real64 = libc::dlsym(libc::RTLD_NEXT, b"open64\0".as_ptr() as *const _);
        REAL_OPEN64.store(real64 as *mut (), Ordering::SeqCst);

        let real_write = libc::dlsym(libc::RTLD_NEXT, b"write\0".as_ptr() as *const _);
        REAL_WRITE.store(real_write as *mut (), Ordering::SeqCst);

        let real_close = libc::dlsym(libc::RTLD_NEXT, b"close\0".as_ptr() as *const _);
        REAL_CLOSE.store(real_close as *mut (), Ordering::SeqCst);

        INITIALIZED.store(true, Ordering::SeqCst);

        log_stderr("[preload] hooks ready\n");
    }
}

unsafe fn classify_fd(fd: c_int, pathname: *const c_char) {
    if !INITIALIZED.load(Ordering::SeqCst) || fd < 0 || pathname.is_null() {
        return;
    }

    let path = CStr::from_ptr(pathname).to_string_lossy();

    if path.contains("node_order_statuses_by_block") {
    } else if path.contains("node_raw_book_diffs_by_block") {
    } else if path.contains("node_fills_by_block") {
    }
}

#[unsafe(no_mangle)]
#[unsafe(export_name = "openat")]
pub unsafe extern "C" fn my_openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: c_int) -> c_int {
    let real_fn = REAL_OPENAT.load(Ordering::SeqCst);
    if real_fn.is_null() {
        return -1;
    }
    let real_openat: OpenatFn = std::mem::transmute(real_fn);
    let fd = real_openat(dirfd, pathname, flags, mode);

    if fd >= 0 {
        classify_fd(fd, pathname);
    }

    log_stderr("[openat] done\n");
    fd
}

#[unsafe(no_mangle)]
#[unsafe(export_name = "open64")]
pub unsafe extern "C" fn my_open64(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int {
    let real_fn = REAL_OPEN64.load(Ordering::SeqCst);
    if real_fn.is_null() {
        return -1;
    }
    let real_open: OpenFn = std::mem::transmute(real_fn);
    let fd = real_open(pathname, flags, mode);
    if fd >= 0 {
        classify_fd(fd, pathname);
    }
    log_stderr("[openat64] done\n");
    fd
}
#[unsafe(no_mangle)]
#[unsafe(export_name = "write")]
pub unsafe extern "C" fn my_write(fd: c_int, buf: *const c_void, count: usize) -> isize {
    let real_fn = REAL_WRITE.load(Ordering::SeqCst);
    if real_fn.is_null() {
        return -1;
    }
    log_stderr("[write] classified\n");

    let real_write: WriteFn = std::mem::transmute(real_fn);

    log_stderr("[write] impl called\n");
    real_write(fd, buf, count)
}

#[unsafe(no_mangle)]
#[unsafe(export_name = "close")]
pub unsafe extern "C" fn my_close(fd: c_int) -> c_int {
    if INITIALIZED.load(Ordering::SeqCst) {}

    let real_fn = REAL_CLOSE.load(Ordering::SeqCst);
    if real_fn.is_null() {
        return -1;
    }

    let real_close: CloseFn = std::mem::transmute(real_fn);
    real_close(fd)
}
