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
use cranelift::prelude::Variable;
use cranelift_codegen::ir::{self, BlockArg};

impl InstTranslator<'_> {
    /// Returns a Cranelift value from KASL-IR value.
    pub(super) fn get_block(&self, kasl_block: &kasl_ir::Block) -> ir::Block {
        self.blocks
            .iter()
            .find(|block| &block.0 == kasl_block)
            .unwrap()
            .1
    }

    /// Returns a Cranelift value from KASL-IR value.
    pub(super) fn get_val(&self, kasl_val: &kasl_ir::Value) -> ir::Value {
        self.vals[kasl_val]
    }

    /// Returns a Cranelift variable from KASL-IR variable.
    pub(super) fn get_var(&self, kasl_var: &kasl_ir::Variable) -> Variable {
        self.vars[kasl_var]
    }

    /// Converts an slice of KASL-IR values into vector of Cranelift BlockArg.
    pub(super) fn convert_args(&self, args: &[kasl_ir::Value]) -> Vec<BlockArg> {
        args.iter()
            .map(|arg| BlockArg::Value(self.get_val(arg)))
            .collect()
    }

    /// Converts a KASL-IR offset into raw offset integer.
    pub(super) fn convert_offset(&self, kasl_offset: kasl_ir::Offset) -> u32 {
        match kasl_offset {
            kasl_ir::Offset::Immediate(offset) => offset,
            kasl_ir::Offset::PointerScaled(scale) => {
                self.type_converter.pointer_type().bytes() * scale
            }
        }
    }
}
