pub mod copy_term;
pub mod dump;
pub mod iter;

use crate::{
  defs::{Word, WordSize},
  term::{boxed, lterm::*},
};

use crate::fail::{Error, RtResult};
use core::fmt;

/// Default heap size for constants (literals) when loading a module.
pub const DEFAULT_LIT_HEAP: usize = 8192;

/// Default heap size when spawning a process. (default: 300)
pub const DEFAULT_PROC_HEAP: usize = 16384;

/// A heap structure which grows upwards with allocations. Cannot expand
/// implicitly and will return error when capacity is exceeded. Organize a
/// garbage collect call to get more memory TODO: gc on heap
pub struct Heap {
  data: Vec<Word>,
  /// Heap top, begins at 0 and grows up towards the stack top `stop`.
  htop: Word,
  /// Stack top, begins at the end of capacity and grows down.
  stop: Word,
  /// Stack end, marks end of heap also.
  send: Word,
}

impl Heap {}

impl fmt::Debug for Heap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Heap{{ cap: {}, used: {} }}",
      self.heap_capacity(),
      self.htop()
    )
  }
}

impl Heap {
  pub fn new(capacity: Word) -> Heap {
    assert!(capacity > 0);
    let mut h = Heap {
      data: Vec::with_capacity(capacity),
      htop: 0,
      stop: capacity,
      send: capacity,
    };
    unsafe { h.data.set_len(capacity) };
    h
  }

  /// How many words do we have before it will require GC/growth.
  fn heap_capacity(&self) -> usize {
    self.data.capacity()
  }

  /// How many words are used.
  fn htop(&self) -> usize {
    self.htop
  }

  /// This is used by heap walkers such as "dump.rs"
  fn heap_begin(&self) -> *const Word {
    &self.data[0] as *const Word
  }

  fn heap_begin_mut(&mut self) -> *mut Word {
    &mut self.data[0] as *mut Word
  }

  /// This is used by heap walkers such as "dump.rs"
  unsafe fn heap_end(&self) -> *const Word {
    let p = self.heap_begin();
    p.add(self.htop)
  }

  pub fn alloc<T>(&mut self, n: WordSize, init_nil: bool) -> RtResult<*mut T> {
    let pos = self.htop;
    let n_words = n.words();
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n_words >= self.stop {
      return Err(Error::HeapIsFull);
    }

    // Assume we can grow the data without reallocating
    let raw_nil = LTerm::nil().raw();
    let new_chunk = unsafe { self.heap_begin_mut().add(self.htop) as *mut Word };

    if init_nil {
      unsafe {
        for i in 0..n_words {
          core::ptr::write(new_chunk.add(i), raw_nil)
        }
      }
    }

    self.htop += n_words;

    Ok(new_chunk as *mut T)
  }

  //  /// Allocate words on heap enough to store bignum digits and copy the given
  //  /// bignum to memory, return the pointer.
  //  pub fn allocate_big(&mut self, big: &num::BigInt) -> Hopefully<BignumPtr> {
  //    match self.allocate(BignumPtr::storage_size(big)) {
  //      Ok(p) => unsafe { Ok(BignumPtr::create_at(p, big)) },
  //      Err(e) => Err(e) // repack inner Err into outer Err
  //    }
  //  }

  #[inline]
  pub fn have(&self, need: Word) -> bool {
    self.htop + need <= self.stop
  }
}

/// Create a constant iterator for walking the heap.
/// This is used by heap walkers such as "dump.rs"
pub unsafe fn heap_iter(hp: &Heap) -> iter::HeapIterator {
  let last = hp.htop as isize;
  let begin = hp.heap_begin() as *const LTerm;
  iter::HeapIterator::new(begin, begin.offset(last))
}

impl Heap {
  #[inline]
  pub fn stack_have(&self, need: Word) -> bool {
    self.htop + need <= self.stop
  }

  //  pub fn stack_alloc(&mut self, need: Word) -> Hopefully<()> {
  //    // Check if heap top is too close to stack top, then fail
  //    if !self.stack_have(need) {
  //      return Err(Error::HeapIsFull)
  //    }
  //    self.stack_alloc_unchecked(need);
  //    Ok(())
  //  }

  /// Allocate stack cells without checking. Call `stack_have(n)` beforehand.
  pub fn stack_alloc_unchecked(&mut self, need: Word, fill_nil: bool) {
    self.stop -= need;

    // Clear the new cells
    let raw_nil = LTerm::nil().raw();
    unsafe {
      let p = self.heap_begin_mut().add(self.stop);

      if fill_nil {
        for y in 0..need {
          core::ptr::write(p.add(y), raw_nil)
        }
      }
    }
  }

  // TODO: Add unsafe push without range checks (batch check+multiple push)
  //  pub fn stack_push(&mut self, val: Word) -> Hopefully<()> {
  //    if !self.stack_have(1) {
  //      return Err(Error::HeapIsFull)
  //    }
  //    self.stack_push_unchecked(val);
  //    Ok(())
  //  }

  #[allow(dead_code)]
  pub fn stack_info(&self) {
    println!("Stack (s_top {}, s_end {})", self.stop, self.send)
  }

  /// Push a value to stack without checking. Call `stack_have(1)` beforehand.
  #[inline]
  pub fn stack_push_unchecked(&mut self, val: Word) {
    self.stop -= 1;
    self.data[self.stop] = val;
  }

  /// Push a LTerm to stack without checking. Call `stack_have(1)` beforehand.
  #[inline]
  pub fn stack_push_lterm_unchecked(&mut self, val: LTerm) {
    self.stop -= 1;
    self.data[self.stop] = val.raw();
  }

  /// Check whether `y+1`-th element can be found in stack
  #[inline]
  pub fn stack_have_y(&self, y: Word) -> bool {
    self.send - self.stop >= y + 1
  }

  /// Set stack value (`index`th from stack top) to `val`.
  pub fn set_y(&mut self, index: Word, val: LTerm) -> RtResult<()> {
    debug_assert!(val.is_value(), "Should never set y[] to a NON_VALUE");
    if !self.stack_have_y(index) {
      return Err(Error::StackIndexRange(index));
    }
    if cfg!(feature = "trace_register_changes") {
      println!("set y{} = {}", index, val);
    }
    self.data[index + self.stop + 1] = val.raw();
    Ok(())
  }

  pub fn stack_get_y(&self, index: Word) -> RtResult<LTerm> {
    if !self.stack_have_y(index) {
      return Err(Error::StackIndexRange(index));
    }
    let pos = index + self.stop + 1;
    let result = LTerm::from_raw(self.data[pos]);
    debug_assert!(result.is_value(), "Should never get a NON_VALUE from y[]");
    Ok(result)
  }

  pub fn stack_depth(&self) -> Word {
    self.send - self.stop
  }

  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  pub fn stack_deallocate(&mut self, n: Word) -> LTerm {
    assert!(
      self.stop + n < self.send,
      "Failed to dealloc {}+1 words (s_top {}, s_end {})",
      n,
      self.stop,
      self.send
    );
    let cp = LTerm::from_raw(self.data[self.stop]);
    assert!(cp.is_cp());
    self.stop += n + 1;
    cp
  }
}

/// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
#[inline]
pub fn allocate_cons(hp: &mut Heap) -> RtResult<*mut boxed::Cons> {
  hp.alloc::<boxed::Cons>(WordSize::new(2), false)
}

// / Expand heap to host `n` words of data
// fn alloc_words<ResultType>(hp: &mut Heap, n: Word, init_nil: bool)
//                           -> Result<*mut ResultType, HeapError> {
//  let result = hp.heap_alloc(n, init_nil)? as *mut ResultType;
//  Ok(result)
//}
