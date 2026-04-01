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

use crate::{InstTranslator, type_conversion::TypeConverter};
use cranelift::prelude::{AbiParam, Configurable, FunctionBuilder, FunctionBuilderContext};
use cranelift_codegen::{settings, verify_function};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use kasl_ir::Function;

pub struct CraneliftBackend {
    builder_ctx: FunctionBuilderContext,
    ctx: cranelift_codegen::Context,
    module: Option<JITModule>,
}

impl Default for CraneliftBackend {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        flag_builder.set("opt_level", "speed").unwrap();
        flag_builder.set("enable_alias_analysis", "true").unwrap();
        let isa_builder = cranelift_native::builder()
            .unwrap_or_else(|msg| panic!("The host machine is not supported: {}", msg));
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);

        Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module: Some(module),
        }
    }
}

impl Drop for CraneliftBackend {
    fn drop(&mut self) {
        if let Some(module) = self.module.take() {
            unsafe { module.free_memory() };
        }
    }
}

impl CraneliftBackend {
    /// Compiles the program to executable function.
    pub fn compile(&mut self, func: Function) -> Result<*const u8, String> {
        self.translate(func);

        // Verify the function
        let verifier_flags = settings::Flags::new(settings::builder());
        verify_function(&self.ctx.func, &verifier_flags).map_err(|e| e.to_string())?;

        let module = self.module.as_mut().unwrap();

        let id = module
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;
        module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        module.clear_context(&mut self.ctx);
        module.finalize_definitions().unwrap();

        let code = module.get_finalized_function(id);
        Ok(code)
    }

    /// Translates the function to cranelift IR.
    pub fn translate(&mut self, func: Function) {
        let module = self.module.as_mut().unwrap();
        let type_converter = TypeConverter::new(module);

        // Add parameters in the entry block
        let Some(entry_block) = func.entry_block().and_then(|block| func.get_block(&block)) else {
            return;
        };

        // Define the function parameters based on the parameters of the entry block
        for arg in entry_block.get_params() {
            let arg_type = func.get_val_type(*arg);
            let converted_type = type_converter.convert(arg_type);
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(converted_type));
        }

        // Create a function builder
        let builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        // Create a translator and translate the instructions to Cranelift IR
        let mut translator = InstTranslator::new(module, builder, func, type_converter);
        translator.translate();
    }
}
