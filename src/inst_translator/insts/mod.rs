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

mod const_val;

use crate::InstTranslator;
use cranelift::prelude::{InstBuilder, MemFlags, StackSlotData, StackSlotKind};
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
            _ => (),
        }
    }
}
