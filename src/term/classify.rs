//! Term ordering and classification.

use rt_defs::*;
use term::boxed;
use term::lterm::*;


fn module() -> &'static str { "classify: " }

/// Enum defines term classification for order comparisons. Use `cmp` on this
/// enum to get relative order for two terms. Enum values are listed in
/// comparison order according to
/// [Term Comparisons](http://erlang.org/doc/reference_manual/expressions.html)
///
/// The order from the documentation is:
///
/// * `number` is less than
/// * `atom` is less than
/// * `reference` is less than
/// * `fun` is less than
/// * `port` is less than
/// * `pid` is less than
/// * `tuple` is less than
/// * `map` is less than
/// * `nil` is less than
/// * `list` is less than
/// * `bit string` (`binary`).
///
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum TermClass {
  Number,
  Atom,
  Ref,
  Fun,
  Port,
  Pid,
  Tuple,
  Map,
  List,
  Binary,
  // Means the value should not be used in comparisons but here it is anyway
  Special_,
}


pub fn classify_term(t: LTerm) -> TermClass {
  let v = t.raw();
  match t.get_term_tag() {
    TermTag::Boxed => unsafe { classify_boxed(t) },
    TermTag::CP => return TermClass::Special_,
    TermTag::Header => TermClass::Special_, // won't look into the header
    TermTag::Small => TermClass::Number,
    TermTag::Atom => TermClass::Atom,
    TermTag::LocalPid => TermClass::Pid,
    TermTag::LocalPort => TermClass::Port,
    TermTag::Special => classify_special(t),
    _ => panic!("{}Invalid primary tag", module())
  }
}


fn classify_special(val: LTerm) -> TermClass {
  match val.get_special_tag() {
    SpecialTag::EmptyList => TermClass::List,
    SpecialTag::EmptyTuple => TermClass::Tuple,
    SpecialTag::EmptyBinary => TermClass::Binary,
    SpecialTag::RegX |
    SpecialTag::RegY |
    SpecialTag::RegFP => TermClass::Special_,
  }
}


/// Given term's raw value `v` and the term itself, try and figure out the
/// classification value for this term.
unsafe fn classify_boxed(val: LTerm) -> TermClass {
  let val_box_ptr = val.get_box_ptr();
  let box_tag = val_box_ptr.get_tag();
  match box_tag {
    boxed::BoxTypeTag::Tuple => TermClass::Tuple,
    boxed::BoxTypeTag::Binary => TermClass::Binary,
    boxed::BoxTypeTag::Tuple => TermClass::Tuple,
    boxed::BoxTypeTag::ExternalPid => TermClass::Pid,
    boxed::BoxTypeTag::ExternalRef => TermClass::Ref,
    boxed::BoxTypeTag::Closure => TermClass::Fun,
    boxed::BoxTypeTag::Float => TermClass::Number,
    _ => panic!("classify: Unexpected boxed_tag={} raw={}",
                box_tag, val.raw())
  }
}


///// Given term's raw value `v` and the term itself, parse its immediate subtags
///// and figure out its classification value.
//#[inline]
//fn classify_immed(v: Word, t: LTerm) -> TermClass {
//  match immediate::get_imm1_tag(v) {
//    immediate::TAG_IMM1_SMALL => TermClass::Number,
//    immediate::TAG_IMM1_PID => TermClass::Pid,
//    immediate::TAG_IMM1_PORT => TermClass::Port,
//    immediate::TAG_IMM1_IMM2 => {
//      match immediate::get_imm2_tag(v) {
//        immediate::TAG_IMM2_CATCH |
//        immediate::TAG_IMM2_IMM3 => TermClass::Special_,
//        immediate::TAG_IMM2_SPECIAL => {
//          if t == LTerm::nil() {
//            TermClass::Nil
//          } else if t == LTerm::empty_tuple() {
//            TermClass::Tuple
//          } else if t == LTerm::empty_binary() {
//            TermClass::Binary
//          } else {
//            TermClass::Special_
//          }
//        },
//        immediate::TAG_IMM2_ATOM => TermClass::Atom,
//        _ => panic!("{}Invalid primary tag", module())
//      } // end match imm2
//    },
//    _ => panic!("{}Invalid primary tag", module())
//  } // end match imm1
//}
