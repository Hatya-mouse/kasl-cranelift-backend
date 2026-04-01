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
use cranelift::prelude::{InstBuilder, IntCC};
use kasl_ir::{IntCmp, Value};

impl InstTranslator<'_> {
    pub(super) fn translate_icmp_imm(&mut self, cmp: IntCmp, lhs: Value, rhs: i64, dst: Value) {
        let ir_lhs = self.get_val(&lhs);

        let val = match cmp {
            IntCmp::Eq => self.builder.ins().icmp_imm(IntCC::Equal, ir_lhs, rhs),
            IntCmp::Ne => self.builder.ins().icmp_imm(IntCC::NotEqual, ir_lhs, rhs),
            IntCmp::Sgt => self
                .builder
                .ins()
                .icmp_imm(IntCC::SignedGreaterThan, ir_lhs, rhs),
            IntCmp::Sge => {
                self.builder
                    .ins()
                    .icmp_imm(IntCC::SignedGreaterThanOrEqual, ir_lhs, rhs)
            }
            IntCmp::Slt => self
                .builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, ir_lhs, rhs),
            IntCmp::Sle => self
                .builder
                .ins()
                .icmp_imm(IntCC::SignedLessThanOrEqual, ir_lhs, rhs),
            IntCmp::Ugt => self
                .builder
                .ins()
                .icmp_imm(IntCC::UnsignedGreaterThan, ir_lhs, rhs),
            IntCmp::Uge => {
                self.builder
                    .ins()
                    .icmp_imm(IntCC::UnsignedGreaterThanOrEqual, ir_lhs, rhs)
            }
            IntCmp::Ult => self
                .builder
                .ins()
                .icmp_imm(IntCC::UnsignedLessThan, ir_lhs, rhs),
            IntCmp::Ule => self
                .builder
                .ins()
                .icmp_imm(IntCC::UnsignedLessThanOrEqual, ir_lhs, rhs),
        };

        self.vals.insert(dst, val);
    }
}
