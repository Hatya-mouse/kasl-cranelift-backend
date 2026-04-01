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

use cranelift_jit::JITBuilder;

/// Import the functions for external function call.
pub(super) fn import_symbols(builder: &mut JITBuilder) {
    // Binary operations
    builder.symbol("f32_powf", f32::powf as *const u8);
    builder.symbol("f64_powf", f64::powf as *const u8);

    builder.symbol("f32_atan2", f32::powf as *const u8);
    builder.symbol("f64_atan2", f64::powf as *const u8);

    builder.symbol("f32_log", f32::log as *const u8);
    builder.symbol("f64_log", f64::log as *const u8);

    // Unary operations
    builder.symbol("f32_sin", f32::sin as *const u8);
    builder.symbol("f64_sin", f64::sin as *const u8);

    builder.symbol("f32_cos", f32::cos as *const u8);
    builder.symbol("f64_cos", f64::cos as *const u8);

    builder.symbol("f32_tan", f32::tan as *const u8);
    builder.symbol("f64_tan", f64::tan as *const u8);

    builder.symbol("f32_asin", f32::asin as *const u8);
    builder.symbol("f64_asin", f64::asin as *const u8);

    builder.symbol("f32_acos", f32::acos as *const u8);
    builder.symbol("f64_acos", f64::acos as *const u8);

    builder.symbol("f32_atan", f32::atan as *const u8);
    builder.symbol("f64_atan", f64::atan as *const u8);

    builder.symbol("f32_exp", f32::exp as *const u8);
    builder.symbol("f64_exp", f64::exp as *const u8);

    builder.symbol("f32_log10", f32::log10 as *const u8);
    builder.symbol("f64_log10", f64::log10 as *const u8);

    builder.symbol("f32_log2", f32::log2 as *const u8);
    builder.symbol("f64_log2", f64::log2 as *const u8);
}
