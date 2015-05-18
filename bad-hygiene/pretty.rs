#![feature(core)]
#![feature(no_std)]
#![feature(print)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
fn main() {
    let fib =
        {
            use std::ops::Index;
            const MEM_SIZE: usize = 1 + 1;
            struct Recurrence {
                mem: [u64; MEM_SIZE],
                pos: usize,
            }
            struct IndexOffset<'a> {
                slice: &'a [u64; MEM_SIZE],
                offset: usize,
            }
            impl <'a> Index<usize> for IndexOffset<'a> {
                type
                Output
                =
                u64;
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b u64 {
                    use std::num::Wrapping;
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
            impl Iterator for Recurrence {
                type
                Item
                =
                u64;
                #[inline]
                fn next(&mut self) -> Option<u64> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val =
                            {
                                let n = self.pos;
                                let a =
                                    IndexOffset{slice: &self.mem, offset: n,};
                                a[n - 1] + a[n - 2]
                            };
                        {
                            use std::mem::swap;
                            let mut swap_tmp = next_val;
                            {
                                let result =
                                    match ::std::iter::IntoIterator::into_iter((0..MEM_SIZE).rev())
                                        {
                                        mut iter =>
                                        loop  {
                                            match ::std::iter::Iterator::next(&mut iter)
                                                {
                                                ::std::option::Option::Some(i)
                                                => {
                                                    swap(&mut swap_tmp,
                                                         &mut self.mem[i]);
                                                }
                                                ::std::option::Option::None =>
                                                break ,
                                            }
                                        },
                                    };
                                result
                            }
                        }
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
            Recurrence{mem: [1, 1], pos: 0,}
        };
    {
        let result =
            match ::std::iter::IntoIterator::into_iter(fib.take(10)) {
                mut iter =>
                loop  {
                    match ::std::iter::Iterator::next(&mut iter) {
                        ::std::option::Option::Some(e) => {
                            ::std::io::_print(::std::fmt::Arguments::new_v1({
                                                                                static __STATIC_FMTSTR:
                                                                                       &'static [&'static str]
                                                                                       =
                                                                                    &["",
                                                                                      "\n"];
                                                                                __STATIC_FMTSTR
                                                                            },
                                                                            &match (&e,)
                                                                                 {
                                                                                 (__arg0,)
                                                                                 =>
                                                                                 [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                              ::std::fmt::Display::fmt)],
                                                                             }))
                        }
                        ::std::option::Option::None => break ,
                    }
                },
            };
        result
    }
}
