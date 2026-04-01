//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

mod bin_op;
mod cmp;
mod cmp_imm;
mod const_val;
mod ext_func_call;
mod resize;
mod unary_op;

use crate::InstTranslator;
use cranelift::prelude::{InstBuilder, MemFlags, StackSlotData, StackSlotKind};
use cranelift_codegen::ir;
use cranelift_module::Module;
use kasl_ir::Inst;

impl InstTranslator<'_> {
    pub(super) fn translate_inst(&mut self, inst: Inst) {
        match inst {
            Inst::Alloc { size, align, dst } => {
                // Create a slot and allocate the slot
                let slot_data = StackSlotData::new(StackSlotKind::ExplicitSlot, size, align as u8);
                let slot = self.builder.create_sized_stack_slot(slot_data);
                let addr =
                    self.builder
                        .ins()
                        .stack_addr(self.type_converter.pointer_type(), slot, 0);
                self.vals.insert(dst, addr);
            }
            Inst::Load {
                ty,
                src_ptr,
                src_offset,
                dst,
            } => {
                // Convert the type of the value to load and load the value at the address
                let ir_type = self.type_converter.convert(ty);
                let ir_src_ptr = self.get_val(&src_ptr);
                let ir_src_offset = self.convert_offset(src_offset);
                let loaded_val = self.builder.ins().load(
                    ir_type,
                    MemFlags::new(),
                    ir_src_ptr,
                    ir_src_offset as i32,
                );
                self.vals.insert(dst, loaded_val);
            }
            Inst::Store {
                src,
                dst_ptr,
                dst_offset,
            } => {
                // Store the value to the destination pointer
                let ir_src = self.get_val(&src);
                let ir_dst_ptr = self.get_val(&dst_ptr);
                let ir_dst_offset = self.convert_offset(dst_offset);
                self.builder
                    .ins()
                    .store(MemFlags::new(), ir_src, ir_dst_ptr, ir_dst_offset as i32);
            }
            Inst::Memcpy {
                size,
                src_ptr,
                dst_ptr,
            } => {
                let ir_src_ptr = self.get_val(&src_ptr);
                let ir_dst_ptr = self.get_val(&dst_ptr);
                self.builder.emit_small_memory_copy(
                    self.module.target_config(),
                    ir_dst_ptr,
                    ir_src_ptr,
                    size as u64,
                    0,
                    0,
                    true,
                    MemFlags::new(),
                );
            }
            Inst::Memset {
                size,
                value,
                dst_ptr,
            } => {
                let ir_dst_ptr = self.get_val(&dst_ptr);
                self.builder.emit_small_memset(
                    self.module.target_config(),
                    ir_dst_ptr,
                    value,
                    size as u64,
                    0,
                    MemFlags::new(),
                );
            }
            Inst::Const { value, dst } => {
                let val = self.translate_const(value);
                self.vals.insert(dst, val);
            }
            Inst::Assign { var, src } => {
                let ir_var = self.get_var(&var);
                let ir_src = self.get_val(&src);
                self.builder.def_var(ir_var, ir_src);
            }
            Inst::LoadVar { var, dst } => {
                let ir_var = self.get_var(&var);
                let val = self.builder.use_var(ir_var);
                self.vals.insert(dst, val);
            }
            Inst::Jump { block, args } => {
                let ir_block = self.get_block(&block);
                let ir_args = self.convert_args(&args);
                self.builder.ins().jump(ir_block, &ir_args);
            }
            Inst::Brif {
                cond,
                then_block,
                then_args,
                else_block,
                else_args,
            } => {
                let ir_cond = self.get_val(&cond);
                let ir_then_block = self.get_block(&then_block);
                let ir_then_args = self.convert_args(&then_args);
                let ir_else_block = self.get_block(&else_block);
                let ir_else_args = self.convert_args(&else_args);
                self.builder.ins().brif(
                    ir_cond,
                    ir_then_block,
                    &ir_then_args,
                    ir_else_block,
                    &ir_else_args,
                );
            }
            Inst::Return { vals } => {
                let ir_vals: Vec<ir::Value> = vals.iter().map(|val| self.get_val(val)).collect();
                self.builder.ins().return_(&ir_vals);
            }
            Inst::Select {
                cond,
                then_val,
                else_val,
                dst,
            } => {
                let ir_cond = self.get_val(&cond);
                let ir_then_val = self.get_val(&then_val);
                let ir_else_val = self.get_val(&else_val);
                let val = self.builder.ins().select(ir_cond, ir_then_val, ir_else_val);
                self.vals.insert(dst, val);
            }
            Inst::IResize { src, dst_ty, dst } => {
                self.translate_iresize(src, dst_ty, dst);
            }
            Inst::FResize { src, dst_ty, dst } => {
                self.translate_fresize(src, dst_ty, dst);
            }
            Inst::IToF { src, dst_ty, dst } => {
                let ir_src = self.get_val(&src);
                let ir_dst_ty = self.type_converter.convert(dst_ty);
                let val = self.builder.ins().fcvt_from_sint(ir_dst_ty, ir_src);
                self.vals.insert(dst, val);
            }
            Inst::FToI { src, dst_ty, dst } => {
                let ir_src = self.get_val(&src);
                let ir_dst_ty = self.type_converter.convert(dst_ty);
                let val = self.builder.ins().fcvt_to_sint(ir_dst_ty, ir_src);
                self.vals.insert(dst, val);
            }
            Inst::PtrAdd { ptr, offset, dst } => {
                let ir_ptr = self.get_val(&ptr);
                let ir_offset = self.convert_offset(offset);
                let val = self.builder.ins().iadd_imm(ir_ptr, ir_offset as i64);
                self.vals.insert(dst, val);
            }
            Inst::IBinOp { op, lhs, rhs, dst } => {
                self.translate_ibop(op, lhs, rhs, dst);
            }
            Inst::FBinOp { op, lhs, rhs, dst } => {
                self.translate_fbop(op, lhs, rhs, dst);
            }
            Inst::IUnaryOp { op, operand, dst } => {
                self.translate_iuop(op, operand, dst);
            }
            Inst::FUnaryOp { op, operand, dst } => {
                self.translate_fuop(op, operand, dst);
            }
            Inst::ICmp { cmp, lhs, rhs, dst } => {
                self.translate_icmp(cmp, lhs, rhs, dst);
            }
            Inst::FCmp { cmp, lhs, rhs, dst } => {
                self.translate_fcmp(cmp, lhs, rhs, dst);
            }
            Inst::ICmpImm { cmp, lhs, rhs, dst } => {
                self.translate_icmp_imm(cmp, lhs, rhs, dst);
            }
        }
    }
}
