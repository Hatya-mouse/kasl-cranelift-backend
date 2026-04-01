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
use cranelift::prelude::{InstBuilder, types};
use cranelift_codegen::ir;
use kasl_ir::Const;

impl InstTranslator<'_> {
    pub(super) fn translate_const(&mut self, value: Const) -> ir::Value {
        match value {
            Const::I8(val) => self.builder.ins().iconst(types::I8, val as i64),
            Const::I16(val) => self.builder.ins().iconst(types::I16, val as i64),
            Const::I32(val) => self.builder.ins().iconst(types::I32, val as i64),
            Const::I64(val) => self.builder.ins().iconst(types::I64, val as i64),
            Const::F32(val) => self.builder.ins().f32const(val),
            Const::F64(val) => self.builder.ins().f64const(val),
            Const::Ptr(val) => self
                .builder
                .ins()
                .iconst(self.type_converter.pointer_type(), val),
        }
    }
}
