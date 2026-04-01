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

use crate::InstTranslator;
use cranelift::prelude::InstBuilder;
use kasl_ir::{FloatBinOp, IntBinOp, Value};

impl InstTranslator<'_> {
    pub(super) fn translate_ibop(&mut self, op: IntBinOp, lhs: Value, rhs: Value, dst: Value) {
        let ir_lhs = self.get_val(&lhs);
        let ir_rhs = self.get_val(&rhs);

        // Add an instruction based on the op
        let val = match op {
            IntBinOp::Add => self.builder.ins().iadd(ir_lhs, ir_rhs),
            IntBinOp::Sub => self.builder.ins().isub(ir_lhs, ir_rhs),
            IntBinOp::Mul => self.builder.ins().imul(ir_lhs, ir_rhs),
            IntBinOp::Div => self.builder.ins().sdiv(ir_lhs, ir_rhs),
            IntBinOp::SRem => self.builder.ins().srem(ir_lhs, ir_rhs),
            IntBinOp::IShL => self.builder.ins().ishl(ir_lhs, ir_rhs),
            IntBinOp::SShR => self.builder.ins().sshr(ir_lhs, ir_rhs),
            IntBinOp::UShR => self.builder.ins().ushr(ir_lhs, ir_rhs),
            IntBinOp::Min => self.builder.ins().smin(ir_lhs, ir_rhs),
            IntBinOp::Max => self.builder.ins().smax(ir_lhs, ir_rhs),
            IntBinOp::BAnd => self.builder.ins().band(ir_lhs, ir_rhs),
            IntBinOp::BOr => self.builder.ins().bor(ir_lhs, ir_rhs),
            IntBinOp::BXor => self.builder.ins().bxor(ir_lhs, ir_rhs),
            IntBinOp::BNand => self.builder.ins().band_not(ir_lhs, ir_rhs),
            IntBinOp::BNor => self.builder.ins().bor_not(ir_lhs, ir_rhs),
            IntBinOp::BXnor => self.builder.ins().bxor_not(ir_lhs, ir_rhs),
        };

        self.vals.insert(dst, val);
    }

    pub(super) fn translate_fbop(&mut self, op: FloatBinOp, lhs: Value, rhs: Value, dst: Value) {
        let ir_lhs = self.get_val(&lhs);
        let ir_rhs = self.get_val(&rhs);

        // Add an instruction based on the op
        let val = match op {
            FloatBinOp::Add => self.builder.ins().fadd(ir_lhs, ir_rhs),
            FloatBinOp::Sub => self.builder.ins().fsub(ir_lhs, ir_rhs),
            FloatBinOp::Mul => self.builder.ins().fmul(ir_lhs, ir_rhs),
            FloatBinOp::Div => self.builder.ins().fdiv(ir_lhs, ir_rhs),
            FloatBinOp::Rem => {
                let div = self.builder.ins().fdiv(ir_lhs, ir_rhs);
                let div_floor = self.builder.ins().floor(div);
                let floor_mul = self.builder.ins().fmul(ir_rhs, div_floor);
                self.builder.ins().fsub(ir_lhs, floor_mul)
            }
            FloatBinOp::Pow => {
                self.call_ext_func_f("f32_powf", "f64_powf", &[lhs, rhs], &[ir_lhs, ir_rhs])[0]
            }
            FloatBinOp::Atan2 => {
                self.call_ext_func_f("f32_atan2", "f64_atan2", &[lhs, rhs], &[ir_lhs, ir_rhs])[0]
            }
            FloatBinOp::Log => {
                self.call_ext_func_f("f32_log", "f64_log", &[lhs, rhs], &[ir_lhs, ir_rhs])[0]
            }
            FloatBinOp::Min => self.builder.ins().fmin(ir_lhs, ir_rhs),
            FloatBinOp::Max => self.builder.ins().fmax(ir_lhs, ir_rhs),
        };

        self.vals.insert(dst, val);
    }
}
