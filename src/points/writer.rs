// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Points;

use {
    ListSeparator,
    WriteBuffer,
    WriteOptions,
};

impl WriteBuffer for (f64, f64) {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        self.0.write_buf_opt(opt, buf);

        match opt.list_separator {
            ListSeparator::Space => buf.push(b' '),
            ListSeparator::Comma => buf.push(b','),
            ListSeparator::CommaSpace => buf.extend_from_slice(b", "),
        }

        self.1.write_buf_opt(opt, buf);
    }
}

impl_display!(Points);
impl_debug_from_display!(Points);
