use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
    time::Instant,
};

#[derive(Copy, Clone)]
enum Source {
    Statuses,
    Diffs,
    Fills,
}
static FD_CLASS: Lazy<DashMap<c_int, Source>> = Lazy::new(|| DashMap::new());

redhook::hook! {
    unsafe fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: c_int) -> c_int => my_openat {
            
        let fd =  redhook::real!(openat)(dirfd, pathname, flags, mode);
            
        println!("called {}", fd);
        if fd >= 0 && !pathname.is_null() {
            let path = CStr::from_ptr(pathname).to_string_lossy();
            println!("called {}", path.clone());
            if path.contains("node_order_statuses_by_block") { FD_CLASS.insert(fd, Source::Statuses); }
            else if path.contains("node_raw_book_diffs_by_block") { FD_CLASS.insert(fd, Source::Diffs); }
            else if path.contains("node_fills_by_block") { FD_CLASS.insert(fd, Source::Fills); }
        }
        fd
    
    }

}
