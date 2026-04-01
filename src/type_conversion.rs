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

use cranelift::prelude::types;
use cranelift_codegen::ir;
use cranelift_jit::JITModule;
use cranelift_module::Module;
use kasl_ir::IRType;

#[derive(Copy, Clone)]
pub struct TypeConverter {
    pointer_type: ir::Type,
}

impl TypeConverter {
    pub fn new(module: &JITModule) -> Self {
        let pointer_type = module.target_config().pointer_type();
        Self { pointer_type }
    }

    pub fn convert(&self, ty: IRType) -> ir::Type {
        match ty {
            IRType::I8 => types::I8,
            IRType::I16 => types::I16,
            IRType::I32 => types::I32,
            IRType::I64 => types::I64,
            IRType::F32 => types::F32,
            IRType::F64 => types::F64,
            IRType::Void => types::INVALID,
            IRType::Ptr => self.pointer_type,
        }
    }

    pub fn pointer_type(&self) -> ir::Type {
        self.pointer_type
    }
}
