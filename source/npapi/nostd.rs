// Copyright 2014 Karl Cheng, other contributors and third parties.
// Licensed under the GNU General Public License, version 3 or later.
// See the COPYRIGHT.md file at the top-level directory of this
// distribution, or go to http://www.gnu.org/licenses/.

// Copyright (c) 2014 Karl Cheng and other contributors.
// Notices for third-party components in this file are available in the
// COPYRIGHT.md file at the top-level directory of this distribution.
//
// Licensed under the GNU General Public License, version 3 or later.
// See the LICENCE-GPLv3.md file at the top-level directory of this
// distribution, or go to <http://www.gnu.org/licenses/>.

/**************************************************************************************************/
/* These three lang functions are required as they are usually provided by the libstd runtime,    */
/* but we can't use them in a dynamic library.                                                    */
/**************************************************************************************************/

#![feature(lang_items)]

extern crate core;
use core::intrinsics::abort;

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

// not sure what needs to be put here, if anything
#[lang="eh_personality"]
extern fn eh_personality() {}

// this crate's definition of failure 
// must be defined by consumers of libcore (core library declares failure, but doesn't define it)
// must be guaranteed to never return
#[lang="begin_unwind"]
extern fn begin_unwind(_args: &core::fmt::Arguments, _file: &str, _line: uint) -> ! {
  // is it really good for an NPAPI library to be doing this?
  // not sure what is the correct way of doing this
  loop {}
}