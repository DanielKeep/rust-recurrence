//! This crate defines a macro for creating iterators which implement
//! recurrence relations.

/*
This is an implementation detail, but it *also* needs to be visible at the
*expansion* site of the `recurrence` macro (*i.e.* in another crate).  Thus,
we need to publically export this macro.  That said, we don't want it showing
up in the documentation; hence the use of `#[doc(hidden)]` and a prefixed
name.
*/
#[doc(hidden)]
#[macro_export]
macro_rules! _recurrence_count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + _recurrence_count_exprs!($($tail),*));
}

/// Expands to an expression implementing the `Iterator` trait, which yields
/// successive elements of the given recurrence relationship.
///
/// For example, you can define a Fibonacci sequence iterator like so:
/// 
/// ```
/// # // Keep in mind that this code block will be run as a test!  The lines
/// # // beginning with `# ` will be stripped from the generated documentation,
/// # // but included in the test.
/// # #[macro_use] extern crate recurrence;
/// # fn main() {
/// #     let _ =
/// recurrence![ fib[n]: f64 = 0.0, 1.0 ... fib[n-1] + fib[n-2] ]
/// #     ;
/// # }
/// ```
#[macro_export]
macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
        {
            use std::ops::Index;
            
            const MEM_SIZE: usize = _recurrence_count_exprs!($($inits),+);
    
            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }
    
            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }
    
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;
    
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;
                    
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);
                    
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
    
            impl Iterator for Recurrence {
                type Item = $sty;
    
                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur
                        };
    
                        {
                            use std::mem::swap;
    
                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
    
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
    
            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}