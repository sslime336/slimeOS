use core::arch::asm;

use lazy_static::lazy_static;

use crate::{println, sync::safe_cell::SafeCell};

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0;
const APP_SIZE_LIMIT: usize = 0;

struct AppManager {
    napp: usize, // The number of apps.
    cur_app_id: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.napp {
            panic!("All applications completed!");
        }

        println!("[kernel] Loading app_{}", app_id);

        // clear app area
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);

        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );

        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());

        app_dst.copy_from_slice(app_src);

        // memory fence about fetching the instruction memory
        asm!("fence.i");
    }
}

lazy_static! {
    static ref APP_MANAGER: SafeCell<AppManager> = unsafe {
        SafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let napp = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), napp + 1);
            app_start[..=napp].copy_from_slice(app_start_raw);
            AppManager {
                napp,
                cur_app_id: 0,
                app_start,
            }
        })
    };
}
