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
use cranelift::prelude::{FloatCC, InstBuilder, IntCC};
use kasl_ir::{FloatCmp, IntCmp, Value};

impl InstTranslator<'_> {
    pub(super) fn translate_icmp(&mut self, cmp: IntCmp, lhs: Value, rhs: Value, dst: Value) {
        let ir_lhs = self.get_val(&lhs);
        let ir_rhs = self.get_val(&rhs);

        let val = match cmp {
            IntCmp::Eq => self.builder.ins().icmp(IntCC::Equal, ir_lhs, ir_rhs),
            IntCmp::Ne => self.builder.ins().icmp(IntCC::NotEqual, ir_lhs, ir_rhs),
            IntCmp::Sgt => self
                .builder
                .ins()
                .icmp(IntCC::SignedGreaterThan, ir_lhs, ir_rhs),
            IntCmp::Sge => self
                .builder
                .ins()
                .icmp(IntCC::SignedGreaterThanOrEqual, ir_lhs, ir_rhs),
            IntCmp::Slt => self
                .builder
                .ins()
                .icmp(IntCC::SignedLessThan, ir_lhs, ir_rhs),
            IntCmp::Sle => self
                .builder
                .ins()
                .icmp(IntCC::SignedLessThanOrEqual, ir_lhs, ir_rhs),
            IntCmp::Ugt => self
                .builder
                .ins()
                .icmp(IntCC::UnsignedGreaterThan, ir_lhs, ir_rhs),
            IntCmp::Uge => {
                self.builder
                    .ins()
                    .icmp(IntCC::UnsignedGreaterThanOrEqual, ir_lhs, ir_rhs)
            }
            IntCmp::Ult => self
                .builder
                .ins()
                .icmp(IntCC::UnsignedLessThan, ir_lhs, ir_rhs),
            IntCmp::Ule => self
                .builder
                .ins()
                .icmp(IntCC::UnsignedLessThanOrEqual, ir_lhs, ir_rhs),
        };

        self.vals.insert(dst, val);
    }

    pub(super) fn translate_fcmp(&mut self, cmp: FloatCmp, lhs: Value, rhs: Value, dst: Value) {
        let ir_lhs = self.get_val(&lhs);
        let ir_rhs = self.get_val(&rhs);

        let val = match cmp {
            FloatCmp::Eq => self.builder.ins().fcmp(FloatCC::Equal, ir_lhs, ir_rhs),
            FloatCmp::Ne => self.builder.ins().fcmp(FloatCC::NotEqual, ir_lhs, ir_rhs),
            FloatCmp::Gt => self
                .builder
                .ins()
                .fcmp(FloatCC::GreaterThan, ir_lhs, ir_rhs),
            FloatCmp::Ge => self
                .builder
                .ins()
                .fcmp(FloatCC::GreaterThanOrEqual, ir_lhs, ir_rhs),
            FloatCmp::Lt => self.builder.ins().fcmp(FloatCC::LessThan, ir_lhs, ir_rhs),
            FloatCmp::Le => self
                .builder
                .ins()
                .fcmp(FloatCC::LessThanOrEqual, ir_lhs, ir_rhs),
        };

        self.vals.insert(dst, val);
    }
}
