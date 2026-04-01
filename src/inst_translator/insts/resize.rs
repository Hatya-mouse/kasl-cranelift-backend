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
use kasl_ir::IRType;

impl InstTranslator<'_> {
    pub(super) fn translate_iresize(
        &mut self,
        src: kasl_ir::Value,
        dst_ty: IRType,
        dst: kasl_ir::Value,
    ) {
        let ir_src = self.get_val(&src);
        let src_ty = self.func.get_val_type(src);
        let ir_dst_ty = self.type_converter.convert(dst_ty);

        // Get the size of the types and resize depending on the comparison result
        let src_size = self.type_converter.get_size(src_ty);
        let dst_size = self.type_converter.get_size(dst_ty);
        let val = if dst_size > src_size {
            self.builder.ins().sextend(ir_dst_ty, ir_src)
        } else if dst_size < src_size {
            self.builder.ins().ireduce(ir_dst_ty, ir_src)
        } else {
            ir_src
        };

        self.vals.insert(dst, val);
    }

    pub(super) fn translate_fresize(
        &mut self,
        src: kasl_ir::Value,
        dst_ty: IRType,
        dst: kasl_ir::Value,
    ) {
        let ir_src = self.get_val(&src);
        let src_ty = self.func.get_val_type(src);
        let ir_dst_ty = self.type_converter.convert(dst_ty);

        // Get the size of the types and resize depending on the comparison result
        let src_size = self.type_converter.get_size(src_ty);
        let dst_size = self.type_converter.get_size(dst_ty);
        let val = if dst_size > src_size {
            self.builder.ins().fpromote(ir_dst_ty, ir_src)
        } else if dst_size < src_size {
            self.builder.ins().fdemote(ir_dst_ty, ir_src)
        } else {
            ir_src
        };

        self.vals.insert(dst, val);
    }
}
