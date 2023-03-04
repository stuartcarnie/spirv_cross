extern crate bindgen;
extern crate convert_case;
extern crate regex;

use std::{env, fmt};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use bindgen::callbacks::{EnumVariantCustomBehavior, EnumVariantValue, ParseCallbacks};
use convert_case::{Case, Casing};
use regex::{Captures, Regex};

struct RenameEnums {
    re: Regex,
}

impl RenameEnums {
    fn new() -> Self {
        Self {
            re: Regex::new(r"\d+(?:F|I|UI|U)").unwrap(),
        }
    }
}

impl Debug for RenameEnums {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt("RenameEnums", f)
    }
}

impl ParseCallbacks for RenameEnums {
    fn enum_variant_name(&self, enum_name: Option<&str>, original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<String> {
        let Some(enum_name) = enum_name else { return None; };

        let converted = enum_name.to_case(Case::Pascal);
        let original = original_variant_name.to_case(Case::Pascal);

        if let Some(val) = original.strip_prefix(&converted) {
            if val.is_empty() || val.as_bytes()[0].is_ascii_digit() {
                None
            } else {
                let val = self.re.replace_all(val, &|caps: &Captures| {
                    caps[0].to_ascii_lowercase()
                }).to_string();
                Some(val)
            }
        } else {
            None
        }
    }

    fn enum_variant_behavior(&self, _enum_name: Option<&str>, original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<EnumVariantCustomBehavior> {
        if original_variant_name.to_ascii_lowercase().ends_with("max") {
            match original_variant_name {
                "BuiltInMax" => None,
                _ => Some(EnumVariantCustomBehavior::Hide),
            }
        } else {
            None
        }
    }
}

fn main() {
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let rename_enums = Box::new(RenameEnums::new());
    // For native targets, include all types and functions
    bindgen::Builder::default()
        .header(
            out_path
                .join("../spirv_cross/src/wrapper.hpp")
                .to_str()
                .unwrap(),
        )
        .prepend_enum_name(false)
        .parse_callbacks(rename_enums)
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .allowlist_function("sc_internal.*")
        .allowlist_type("spv::.*")
        .allowlist_type("Sc.*")
        .bitfield_enum(".*(Mask|Flags)")
        .rustified_enum("spv::BuiltIn")
        .rustified_enum("spv::Decoration")
        .rustified_enum("spv::SourceLanguage")
        .rustified_enum("spv::ExecutionModel")
        .rustified_enum("spv::AddressingModel")
        .rustified_enum("spv::MemoryModel")
        .rustified_enum("spv::ExecutionMode")
        .rustified_enum("spv::StorageClass")
        .rustified_enum("spv::Dim")
        .rustified_enum("spv::SamplerAddressingMode")
        .rustified_enum("spv::SamplerFilterMode")
        .rustified_enum("spv::ImageFormat")
        .rustified_enum("spv::ImageChannelOrder")
        .rustified_enum("spv::ImageChannelDataType")
        .rustified_enum("spv::ImageOperandsShift")
        .rustified_enum("spv::FPFastMathModeShift")
        .rustified_enum("spv::FPFastMathModeMask")
        .rustified_enum("spv::FPRoundingMode")
        .rustified_enum("spv::LinkageType")
        .rustified_enum("spv::AccessQualifier")
        .rustified_enum("spv::FunctionParameterAttribute")
        .rustified_enum("spv::Decoration")
        .rustified_enum("spv::BuiltIn")
        .rustified_enum("spv::SelectionControlShift")
        .rustified_enum("spv::LoopControlShift")
        .rustified_enum("spv::FunctionControlShift")
        .rustified_enum("spv::MemorySemanticsShift")
        .rustified_enum("spv::MemoryAccessShift")
        .rustified_enum("spv::Scope")
        .rustified_enum("spv::GroupOperation")
        .rustified_enum("spv::KernelEnqueueFlags")
        .rustified_enum("spv::KernelProfilingInfoShift")
        .rustified_enum("spv::Capability")
        .rustified_enum("spv::RayFlagsShift")
        .rustified_enum("spv::RayQueryIntersection")
        .rustified_enum("spv::RayQueryCommittedIntersectionType")
        .rustified_enum("spv::RayQueryCandidateIntersectionType")
        .rustified_enum("spv::FragmentShadingRateShift")
        .rustified_enum("spv::FPDenormMode")
        .rustified_enum("spv::FPOperationMode")
        .rustified_enum("spv::QuantizationModes")
        .rustified_enum("spv::OverflowModes")
        .rustified_enum("spv::PackedVectorFormat")
        .rustified_enum("spv::Op")
        .rustified_enum("spv::StorageClass")
        .rustified_enum("ScInternalResult")
        .rustified_enum("spirv_cross::SPIRType_BaseType")
        .rustified_enum("spirv_cross::MSLVertexFormat")
        .rustified_enum("spirv_cross::MSLShaderVariableFormat")
        .rustified_enum("spirv_cross::MSLShaderVariableRate")
        .rustified_enum("spirv_cross::MSLSamplerCoord")
        .rustified_enum("spirv_cross::MSLSamplerFilter")
        .rustified_enum("spirv_cross::MSLSamplerMipFilter")
        .rustified_enum("spirv_cross::MSLSamplerAddress")
        .rustified_enum("spirv_cross::MSLSamplerCompareFunc")
        .rustified_enum("spirv_cross::MSLSamplerBorderColor")
        .rustified_enum("spirv_cross::MSLFormatResolution")
        .rustified_enum("spirv_cross::MSLChromaLocation")
        .rustified_enum("spirv_cross::MSLComponentSwizzle")
        .rustified_enum("spirv_cross::MSLSamplerYCbCrModelConversion")
        .rustified_enum("spirv_cross::MSLSamplerYCbCrRange")
        .opaque_type("std::.*")
        .clang_args(vec![
            "-DSPIRV_CROSS_WRAPPER_GLSL",
            "-DSPIRV_CROSS_WRAPPER_MSL",
            "-DSPIRV_CROSS_WRAPPER_HLSL",
        ])
        .derive_eq(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        .derive_ord(true)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("../spirv_cross/src/bindings_native.rs"))
        .expect("Couldn't write bindings!");
    // For wasm targets, include all types, functions will be implemented manually
    bindgen::Builder::default()
        .header(
            out_path
                .join("../spirv_cross/src/wrapper.hpp")
                .to_str()
                .unwrap(),
        )
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .allowlist_type("spv::.*")
        .allowlist_type("Sc.*")
        .bitfield_enum(".*(Mask|Flags)")
        .rustified_enum("spv::BuiltIn")
        .rustified_enum("spv::Decoration")
        .rustified_enum("spv::ExecutionModel")
        .rustified_enum("spv::ImageFormat")
        .rustified_enum("spv::Dim")
        .rustified_enum("ScInternalResult")
        .rustified_enum("spirv_cross::SPIRType_BaseType")
        .rustified_enum("spirv_cross::MSLVertexFormat")
        .opaque_type("std::.*")
        .clang_args(vec![
            "-DSPIRV_CROSS_WRAPPER_GLSL",
            "-DSPIRV_CROSS_WRAPPER_MSL",
            "-DSPIRV_CROSS_WRAPPER_HLSL",
        ])
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("../spirv_cross/src/bindings_wasm.rs"))
        .expect("Couldn't write bindings!");
}
