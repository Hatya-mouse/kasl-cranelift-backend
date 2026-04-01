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
use cranelift::prelude::{FloatCC, InstBuilder, IntCC, types};
use kasl_ir::{FloatUnaryOp, IntUnaryOp, Value};

impl InstTranslator<'_> {
    pub(super) fn translate_iuop(&mut self, op: IntUnaryOp, operand: Value, dst: Value) {
        let ir_operand = self.get_val(&operand);

        let val = match op {
            IntUnaryOp::Abs => self.builder.ins().iabs(ir_operand),
            IntUnaryOp::Sgn => {
                let is_positive =
                    self.builder
                        .ins()
                        .icmp_imm(IntCC::SignedGreaterThan, ir_operand, 0);
                let is_negative = self
                    .builder
                    .ins()
                    .icmp_imm(IntCC::SignedLessThan, ir_operand, 0);

                let one = self.builder.ins().iconst(types::I32, 1);
                let zero = self.builder.ins().iconst(types::I32, 0);
                let minus_one = self.builder.ins().iconst(types::I32, -1);
                let pos_val = self.builder.ins().select(is_positive, one, zero);
                self.builder.ins().select(is_negative, minus_one, pos_val)
            }
            IntUnaryOp::Neg => self.builder.ins().ineg(ir_operand),
            IntUnaryOp::BNot => self.builder.ins().bnot(ir_operand),
        };

        self.vals.insert(dst, val);
    }

    pub(super) fn translate_fuop(&mut self, op: FloatUnaryOp, operand: Value, dst: Value) {
        let ir_operand = self.get_val(&operand);

        let val = match op {
            FloatUnaryOp::Abs => self.builder.ins().fabs(ir_operand),
            FloatUnaryOp::Sgn => {
                let one = self.builder.ins().f32const(1.0);
                let zero = self.builder.ins().f32const(0.0);
                let minus_one = self.builder.ins().f32const(-1.0);

                let is_positive = self
                    .builder
                    .ins()
                    .fcmp(FloatCC::GreaterThan, ir_operand, zero);
                let is_negative = self.builder.ins().fcmp(FloatCC::LessThan, ir_operand, zero);

                let pos_val = self.builder.ins().select(is_positive, one, zero);
                self.builder.ins().select(is_negative, minus_one, pos_val)
            }
            FloatUnaryOp::Neg => self.builder.ins().fneg(ir_operand),
            FloatUnaryOp::Floor => self.builder.ins().floor(ir_operand),
            FloatUnaryOp::Ceil => self.builder.ins().ceil(ir_operand),
            FloatUnaryOp::Round => self.builder.ins().nearest(ir_operand),
            FloatUnaryOp::Sin => {
                self.call_ext_func_f("f32_sin", "f64_sin", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Cos => {
                self.call_ext_func_f("f32_cos", "f64_cos", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Tan => {
                self.call_ext_func_f("f32_tan", "f64_tan", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Asin => {
                self.call_ext_func_f("f32_asin", "f64_asin", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Acos => {
                self.call_ext_func_f("f32_acos", "f64_acos", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Atan => {
                self.call_ext_func_f("f32_atan", "f64_atan", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Exp => {
                self.call_ext_func_f("f32_exp", "f64_exp", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Log10 => {
                self.call_ext_func_f("f32_log10", "f64_log10", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Log2 => {
                self.call_ext_func_f("f32_log2", "f64_log2", &[operand], &[ir_operand])[0]
            }
            FloatUnaryOp::Sqrt => self.builder.ins().sqrt(ir_operand),
        };

        self.vals.insert(dst, val);
    }
}
