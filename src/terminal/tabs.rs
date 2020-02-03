// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// This file is part of cancer.
//
// cancer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cancer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cancer.  If not, see <http://www.gnu.org/licenses/>.

use bit_vec::BitVec;

#[derive(Debug)]
pub struct Tabs {
    cols: u32,
    rows: u32,

    inner: BitVec,
}

impl Tabs {
    pub fn new(cols: u32, rows: u32) -> Self {
        Tabs {
            cols: cols,
            rows: rows,

            inner: BitVec::from_fn(cols as usize, |i| i % 8 == 0),
        }
    }

    pub fn resize(&mut self, cols: u32, rows: u32) {
        self.inner.grow(cols as usize, false);

        if cols > self.cols {
            for i in (self.cols..cols).filter(|i| i % 8 == 0) {
                self.inner.set(i as usize, true);
            }
        }

        self.cols = cols;
        self.rows = rows;
    }

    pub fn set(&mut self, x: u32, value: bool) {
        self.inner.set(x as usize, value);
    }

    pub fn get(&self, x: u32) -> bool {
        self.inner.get(x as usize).unwrap_or(false)
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn next(&self, n: i32, start: u32) -> u32 {
        let mut end = start;

        if n > 0 {
            while end < self.cols {
                end += 1;

                if self.get(end) {
                    break;
                }
            }
        } else {
            while end != 0 {
                end -= 1;

                if self.get(end) {
                    break;
                }
            }
        }

        if end == self.cols && !self.get(end) {
            start
        } else {
            end
        }
    }
}
