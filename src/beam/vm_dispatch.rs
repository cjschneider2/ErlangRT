//! Generated by `codegen/create_vm_dispatch.py`
//! Dispatch for all opcode types.
//! Config used: OTP20
#![allow(dead_code)]

use crate::{
  beam::{disp_result::DispatchResult, gen_op::*, opcodes::*},
  emulator::{code::opcode::RawOpcode, process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};

#[inline]
pub fn dispatch_op_inline(vm: &mut VM, op: RawOpcode, ctx: &mut Context, curr_p: &mut Process) -> RtResult<DispatchResult> {
  match op {
    OPCODE_FUNC_INFO => {
      assert_arity(OPCODE_FUNC_INFO, OpcodeFuncInfo::ARITY);
      return OpcodeFuncInfo::run(vm, ctx, curr_p);
    },

    OPCODE_CALL => {
      assert_arity(OPCODE_CALL, OpcodeCall::ARITY);
      return OpcodeCall::run(vm, ctx, curr_p);
    },

    OPCODE_CALL_LAST => {
      assert_arity(OPCODE_CALL_LAST, OpcodeCallLast::ARITY);
      return OpcodeCallLast::run(vm, ctx, curr_p);
    },

    OPCODE_CALL_ONLY => {
      assert_arity(OPCODE_CALL_ONLY, OpcodeCallOnly::ARITY);
      return OpcodeCallOnly::run(vm, ctx, curr_p);
    },

    OPCODE_CALL_EXT => {
      assert_arity(OPCODE_CALL_EXT, OpcodeCallExt::ARITY);
      return OpcodeCallExt::run(vm, ctx, curr_p);
    },

    OPCODE_CALL_EXT_LAST => {
      assert_arity(OPCODE_CALL_EXT_LAST, OpcodeCallExtLast::ARITY);
      return OpcodeCallExtLast::run(vm, ctx, curr_p);
    },

    OPCODE_BIF0 => {
      assert_arity(OPCODE_BIF0, OpcodeBif0::ARITY);
      return OpcodeBif0::run(vm, ctx, curr_p);
    },

    OPCODE_BIF1 => {
      assert_arity(OPCODE_BIF1, OpcodeBif1::ARITY);
      return OpcodeBif1::run(vm, ctx, curr_p);
    },

    OPCODE_BIF2 => {
      assert_arity(OPCODE_BIF2, OpcodeBif2::ARITY);
      return OpcodeBif2::run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE => {
      assert_arity(OPCODE_ALLOCATE, OpcodeAllocate::ARITY);
      return OpcodeAllocate::run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE_HEAP => {
      assert_arity(OPCODE_ALLOCATE_HEAP, OpcodeAllocateHeap::ARITY);
      return OpcodeAllocateHeap::run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE_ZERO => {
      assert_arity(OPCODE_ALLOCATE_ZERO, OpcodeAllocateZero::ARITY);
      return OpcodeAllocateZero::run(vm, ctx, curr_p);
    },

    OPCODE_ALLOCATE_HEAP_ZERO => {
      assert_arity(OPCODE_ALLOCATE_HEAP_ZERO, OpcodeAllocateHeapZero::ARITY);
      return OpcodeAllocateHeapZero::run(vm, ctx, curr_p);
    },

    OPCODE_TEST_HEAP => {
      assert_arity(OPCODE_TEST_HEAP, OpcodeTestHeap::ARITY);
      return OpcodeTestHeap::run(vm, ctx, curr_p);
    },

    OPCODE_DEALLOCATE => {
      assert_arity(OPCODE_DEALLOCATE, OpcodeDeallocate::ARITY);
      return OpcodeDeallocate::run(vm, ctx, curr_p);
    },

    OPCODE_RETURN => {
      assert_arity(OPCODE_RETURN, OpcodeReturn::ARITY);
      return OpcodeReturn::run(vm, ctx, curr_p);
    },

    OPCODE_SEND => {
      assert_arity(OPCODE_SEND, OpcodeSend::ARITY);
      return OpcodeSend::run(vm, ctx, curr_p);
    },

    OPCODE_REMOVE_MESSAGE => {
      assert_arity(OPCODE_REMOVE_MESSAGE, OpcodeRemoveMessage::ARITY);
      return OpcodeRemoveMessage::run(vm, ctx, curr_p);
    },

    OPCODE_LOOP_REC => {
      assert_arity(OPCODE_LOOP_REC, OpcodeLoopRec::ARITY);
      return OpcodeLoopRec::run(vm, ctx, curr_p);
    },

    OPCODE_LOOP_REC_END => {
      assert_arity(OPCODE_LOOP_REC_END, OpcodeLoopRecEnd::ARITY);
      return OpcodeLoopRecEnd::run(vm, ctx, curr_p);
    },

    OPCODE_IS_LT => {
      assert_arity(OPCODE_IS_LT, OpcodeIsLt::ARITY);
      return OpcodeIsLt::run(vm, ctx, curr_p);
    },

    OPCODE_IS_GE => {
      assert_arity(OPCODE_IS_GE, OpcodeIsGe::ARITY);
      return OpcodeIsGe::run(vm, ctx, curr_p);
    },

    OPCODE_IS_EQ => {
      assert_arity(OPCODE_IS_EQ, OpcodeIsEq::ARITY);
      return OpcodeIsEq::run(vm, ctx, curr_p);
    },

    OPCODE_IS_EQ_EXACT => {
      assert_arity(OPCODE_IS_EQ_EXACT, OpcodeIsEqExact::ARITY);
      return OpcodeIsEqExact::run(vm, ctx, curr_p);
    },

    OPCODE_IS_INTEGER => {
      assert_arity(OPCODE_IS_INTEGER, OpcodeIsInteger::ARITY);
      return OpcodeIsInteger::run(vm, ctx, curr_p);
    },

    OPCODE_IS_ATOM => {
      assert_arity(OPCODE_IS_ATOM, OpcodeIsAtom::ARITY);
      return OpcodeIsAtom::run(vm, ctx, curr_p);
    },

    OPCODE_IS_NIL => {
      assert_arity(OPCODE_IS_NIL, OpcodeIsNil::ARITY);
      return OpcodeIsNil::run(vm, ctx, curr_p);
    },

    OPCODE_IS_NONEMPTY_LIST => {
      assert_arity(OPCODE_IS_NONEMPTY_LIST, OpcodeIsNonemptyList::ARITY);
      return OpcodeIsNonemptyList::run(vm, ctx, curr_p);
    },

    OPCODE_SELECT_VAL => {
      assert_arity(OPCODE_SELECT_VAL, OpcodeSelectVal::ARITY);
      return OpcodeSelectVal::run(vm, ctx, curr_p);
    },

    OPCODE_MOVE => {
      assert_arity(OPCODE_MOVE, OpcodeMove::ARITY);
      return OpcodeMove::run(vm, ctx, curr_p);
    },

    OPCODE_GET_LIST => {
      assert_arity(OPCODE_GET_LIST, OpcodeGetList::ARITY);
      return OpcodeGetList::run(vm, ctx, curr_p);
    },

    OPCODE_PUT_LIST => {
      assert_arity(OPCODE_PUT_LIST, OpcodePutList::ARITY);
      return OpcodePutList::run(vm, ctx, curr_p);
    },

    OPCODE_BADMATCH => {
      assert_arity(OPCODE_BADMATCH, OpcodeBadmatch::ARITY);
      return OpcodeBadmatch::run(vm, ctx, curr_p);
    },

    OPCODE_CALL_FUN => {
      assert_arity(OPCODE_CALL_FUN, OpcodeCallFun::ARITY);
      return OpcodeCallFun::run(vm, ctx, curr_p);
    },

    OPCODE_IS_FUNCTION => {
      assert_arity(OPCODE_IS_FUNCTION, OpcodeIsFunction::ARITY);
      return OpcodeIsFunction::run(vm, ctx, curr_p);
    },

    OPCODE_CALL_EXT_ONLY => {
      assert_arity(OPCODE_CALL_EXT_ONLY, OpcodeCallExtOnly::ARITY);
      return OpcodeCallExtOnly::run(vm, ctx, curr_p);
    },

    OPCODE_MAKE_FUN2 => {
      assert_arity(OPCODE_MAKE_FUN2, OpcodeMakeFun2::ARITY);
      return OpcodeMakeFun2::run(vm, ctx, curr_p);
    },

    OPCODE_IS_FUNCTION2 => {
      assert_arity(OPCODE_IS_FUNCTION2, OpcodeIsFunction2::ARITY);
      return OpcodeIsFunction2::run(vm, ctx, curr_p);
    },

    OPCODE_GC_BIF1 => {
      assert_arity(OPCODE_GC_BIF1, OpcodeGcBif1::ARITY);
      return OpcodeGcBif1::run(vm, ctx, curr_p);
    },

    OPCODE_GC_BIF2 => {
      assert_arity(OPCODE_GC_BIF2, OpcodeGcBif2::ARITY);
      return OpcodeGcBif2::run(vm, ctx, curr_p);
    },

    OPCODE_TRIM => {
      assert_arity(OPCODE_TRIM, OpcodeTrim::ARITY);
      return OpcodeTrim::run(vm, ctx, curr_p);
    },

    OPCODE_GC_BIF3 => {
      assert_arity(OPCODE_GC_BIF3, OpcodeGcBif3::ARITY);
      return OpcodeGcBif3::run(vm, ctx, curr_p);
    },

    other => unknown_opcode(other, ctx),
  }
  Ok(DispatchResult::Yield)
}

