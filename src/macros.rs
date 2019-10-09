// Copyright (c) 2017 Stefan Lankes, RWTH Aachen University
//                    Colin Finck, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

macro_rules! align_down {
	($value:expr, $alignment:expr) => {
		$value & !($alignment - 1)
	};
}

macro_rules! align_up {
	($value:expr, $alignment:expr) => {
		align_down!($value + ($alignment - 1), $alignment)
	};
}

/// Print formatted text to our console.
///
/// From http://blog.phil-opp.com/rust-os/printing-to-screen.html, but tweaked
/// for HermitCore.
macro_rules! print {
	($($arg:tt)+) => ({
		use core::fmt::Write;
		$crate::console::CONSOLE.lock().write_fmt(format_args!($($arg)+)).unwrap();
	});
}

/// Print formatted text to our console, followed by a newline.
macro_rules! println {
	($($arg:tt)+) => (print!("{}\n", format_args!($($arg)+)));
}

macro_rules! isolate_var {
    /* .data */
    (static $name:ident: $var_type:ty, $val:expr) => {
        #[link_section = ".isolated_data"]
        static $name: $var_type = $val;
    };

    (static mut $name:ident: $var_type:ty, $val:expr) => {
        #[link_section = ".isolated_data"]
        static mut $name: $var_type = $val;
    };

    /* .bss */
    (static $name:ident: $var_type:ty) => {
        #[link_section = ".isolated_bss"]
        static $name: $var_type = 0;
    };

    (static mut $name:ident: $var_type:ty) => {
        #[link_section = ".isolated_bss"]
        static mut $name: $var_type = 0;
    };
}

macro_rules! isolate_function_no_ret {
    ($f:ident($($x:tt)*)) => {{
        use x86_64::mm::mpk;
        use mm::SAFE_MEM_REGION;
        mpk::mpk_set_perm(SAFE_MEM_REGION, mpk::MpkPerm::MpkNone);
        $f($($x)*);
        mpk::mpk_set_perm(SAFE_MEM_REGION, mpk::MpkPerm::MpkRw);
    }};
}

macro_rules! isolate_function {
    ($f:ident($($x:tt)*)) => {{
        use x86_64::mm::mpk;
        use mm::SAFE_MEM_REGION;
        mpk::mpk_set_perm(SAFE_MEM_REGION, mpk::MpkPerm::MpkNone);
        let temp_ret = $f($($x)*);
        mpk::mpk_set_perm(SAFE_MEM_REGION, mpk::MpkPerm::MpkRw);
        temp_ret
    }};
}
