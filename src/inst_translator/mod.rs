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

mod block_translator;
mod insts;
mod utils;

use crate::type_conversion::TypeConverter;
use cranelift::prelude::{FunctionBuilder, Variable};
use cranelift_codegen::ir;
use cranelift_jit::JITModule;
use kasl_ir::Function;
use std::collections::HashMap;

/// A struct to translate the instructions to cranelift IR.
pub(crate) struct InstTranslator<'a> {
    module: &'a mut JITModule,
    pub(super) builder: FunctionBuilder<'a>,
    func: Function,
    type_converter: TypeConverter,

    blocks: Vec<(kasl_ir::Block, ir::Block)>,
    vals: HashMap<kasl_ir::Value, ir::Value>,
    vars: HashMap<kasl_ir::Variable, Variable>,
}

impl<'a> InstTranslator<'a> {
    /// Creates a new InstTranslator instance.
    pub fn new(
        module: &'a mut JITModule,
        builder: FunctionBuilder<'a>,
        func: Function,
        type_converter: TypeConverter,
    ) -> Self {
        Self {
            module,
            builder,
            func,
            type_converter,
            blocks: Vec::new(),
            vals: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    /// Translates the function to cranelift IR.
    pub fn translate(&mut self) {
        // Create cranelift blocks for every blocks
        let blocks = self.func.sorted_blocks();
        for block in blocks {
            let ir_block = self.builder.create_block();
            self.blocks.push((block, ir_block));
        }

        // Declare all variables
        for kasl_var in self.func.get_vars() {
            let var_ty = self.func.get_var_type(kasl_var);
            let ir_ty = self.type_converter.convert(var_ty);
            let ir_var = self.builder.declare_var(ir_ty);
            self.vars.insert(kasl_var, ir_var);
        }

        // Translate the blocks
        for (kasl_block, ir_block) in self.blocks.clone() {
            self.translate_block(kasl_block, ir_block);
        }

        // Seal all blocks
        self.builder.seal_all_blocks();
    }
}
