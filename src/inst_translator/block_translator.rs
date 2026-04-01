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
use cranelift_codegen::ir;

impl InstTranslator<'_> {
    pub(super) fn translate_block(&mut self, kasl_block: kasl_ir::Block, ir_block: ir::Block) {
        // Switch to the ir block
        self.builder.switch_to_block(ir_block);

        // Get the block data
        if let Some(block_data) = self.func.get_block(&kasl_block) {
            // Get the block parameters and register them as a value
            let params = self.builder.block_params(ir_block);
            for (kasl_param, ir_param) in block_data.get_params().iter().zip(params.iter()) {
                self.vals.insert(*kasl_param, *ir_param);
            }

            // Translate the instructions
            for inst in block_data.get_insts().iter().cloned().collect::<Vec<_>>() {
                self.translate_inst(inst);
            }
        }
    }
}
