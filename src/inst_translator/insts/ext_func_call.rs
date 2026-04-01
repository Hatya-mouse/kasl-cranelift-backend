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
use cranelift::prelude::{AbiParam, InstBuilder};
use cranelift_codegen::ir;
use cranelift_module::{Linkage, Module};
use kasl_ir::IRType;

impl InstTranslator<'_> {
    pub fn call_ext_func_f(
        &mut self,
        f32_name: &str,
        f64_name: &str,
        kasl_args: &[kasl_ir::Value],
        ir_args: &[ir::Value],
    ) -> &[ir::Value] {
        let mut sig = self.module.make_signature();
        let kasl_types: Vec<IRType> = kasl_args
            .iter()
            .map(|kasl_arg| self.func.get_val_type(*kasl_arg))
            .collect();
        let ir_types: Vec<ir::Type> = kasl_types
            .iter()
            .map(|ty| self.type_converter.convert(*ty))
            .collect();

        sig.params.extend_from_slice(
            &ir_types
                .iter()
                .map(|ty| AbiParam::new(*ty))
                .collect::<Vec<_>>(),
        );
        sig.returns.push(AbiParam::new(ir_types[0]));
        let func_id = match kasl_types[0] {
            IRType::F32 => self
                .module
                .declare_function(f32_name, Linkage::Import, &sig)
                .unwrap(),
            IRType::F64 => self
                .module
                .declare_function(f64_name, Linkage::Import, &sig)
                .unwrap(),
            _ => panic!("Non-float type is passed to fbop"),
        };
        let func_ref = self.module.declare_func_in_func(func_id, self.builder.func);
        let call = self.builder.ins().call(func_ref, &ir_args);
        self.builder.inst_results(call)
    }
}
