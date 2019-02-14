use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{boxed, lterm::LTerm},
};

/// Map builder allocates necessary space on the given heap and allows
/// adding keys and values as necessary.
///
/// 1. Create MapBuilder with the heap where you want to build.
/// 2. Call `add(key, value)`
/// 3. Finalize by requesting the term value of a newly built map.
pub struct MapBuilder {
  // because i can't into lifetimes :( but it lives short anyway
  // heap: *mut Heap,
  p: *mut boxed::Map,
}

impl MapBuilder {
  pub fn new(heap: &mut Heap, size_hint: usize) -> RtResult<Self> {
    let p = boxed::Map::create_into(heap, size_hint)?;
    Ok(Self { p })
  }

  pub unsafe fn add(&mut self, key: LTerm, value: LTerm) -> RtResult<()> {
    boxed::Map::add(self.p, key, value)?;
    Ok(())
  }

  pub fn make_term(&mut self) -> LTerm {
    LTerm::make_boxed(self.p)
  }
}