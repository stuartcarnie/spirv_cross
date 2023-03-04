//! Raw compiler bindings for SPIRV-Cross.
use crate::{bindings as br, spirv::ImageType};
use crate::ptr_util::{read_from_ptr, read_into_vec_from_ptr, read_string_from_ptr};
use crate::spirv::{self, Decoration, Type};
use crate::ErrorCode;
use std::collections::HashSet;
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem::MaybeUninit, ptr};

impl spirv::ImageType {
    pub(crate) fn from_raw(ty: br::spirv_cross::SPIRType_ImageType) -> ImageType {
        ImageType {
            type_id: ty.type_,
            dim: ty.dim,
            depth: ty.depth,
            arrayed: ty.arrayed,
            ms: ty.ms,
            sampled: ty.sampled,
            format: ty.format,
        }
    }
}

impl spirv::Type {
    pub(crate) fn from_raw(
        ty: br::spirv_cross::SPIRType_BaseType,
        vecsize: u32,
        columns: u32,
        member_types: Vec<u32>,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
        image: br::spirv_cross::SPIRType_ImageType,
    ) -> Self {
        use crate::bindings::root::spirv_cross::SPIRType_BaseType as B;
        use crate::spirv::Type::*;
        match ty {
            B::Unknown => Unknown,
            B::Void => Void,
            B::Boolean => Boolean {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Char => Char { array, array_size_literal },
            B::Int => Int {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::UInt => UInt {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Int64 => Int64 { vecsize, array, array_size_literal },
            B::UInt64 => UInt64 { vecsize, array, array_size_literal },
            B::AtomicCounter => AtomicCounter { array, array_size_literal },
            B::Half => Half {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Float => Float {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Double => Double {
                vecsize,
                columns,
                array,
                array_size_literal,
            },
            B::Struct => Struct {
                member_types,
                array,
                array_size_literal,
            },
            B::Image => Image { array, array_size_literal, image: ImageType::from_raw(image) },
            B::SampledImage => SampledImage { array, array_size_literal, image: ImageType::from_raw(image) },
            B::Sampler => Sampler { array, array_size_literal },
            B::SByte => SByte { vecsize, array, array_size_literal },
            B::UByte => UByte { vecsize, array, array_size_literal },
            B::Short => Short { vecsize, array, array_size_literal },
            B::UShort => UShort { vecsize, array, array_size_literal },
            B::ControlPointArray => ControlPointArray,
            B::AccelerationStructure => AccelerationStructure,
            B::RayQuery => RayQuery,
            B::Interpolant => Interpolant,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compiler<TTargetData> {
    pub(crate) sc_compiler: *mut br::ScInternalCompilerBase,
    pub(crate) target_data: TTargetData,
    pub(crate) has_been_compiled: bool,
}

impl<TTargetData> Compiler<TTargetData> {
    #[cfg(any(feature = "glsl", feature = "hlsl"))]
    pub fn compile(&mut self) -> Result<String, ErrorCode> {
        unsafe {
            let mut shader_ptr = ptr::null();
            check!(br::sc_internal_compiler_compile(
                self.sc_compiler,
                &mut shader_ptr,
            ));
            let shader = read_string_from_ptr(shader_ptr)?;
            check!(br::sc_internal_free_pointer(shader_ptr as *mut c_void));
            Ok(shader)
        }
    }

    pub fn get_decoration(&self, id: u32, decoration: spirv::Decoration) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_decoration(
                self.sc_compiler,
                &mut result,
                id,
                decoration,
            ));
        }
        Ok(result)
    }

    pub fn get_name(&mut self, id: u32) -> Result<String, ErrorCode> {
        unsafe {
            let mut name_ptr = ptr::null();
            check!(br::sc_internal_compiler_get_name(
                self.sc_compiler,
                id,
                &mut name_ptr,
            ));
            let name = read_string_from_ptr(name_ptr)?;
            check!(br::sc_internal_free_pointer(name_ptr as *mut c_void));
            Ok(name)
        }
    }

    pub fn set_name(&mut self, id: u32, name: &str) -> Result<(), ErrorCode> {
        let name = CString::new(name);
        unsafe {
            match name {
                Ok(name) => {
                    check!(br::sc_internal_compiler_set_name(
                        self.sc_compiler,
                        id,
                        name.as_ptr(),
                    ));
                }
                _ => return Err(ErrorCode::Unhandled),
            }
        }
        Ok(())
    }

    pub fn set_member_name(&mut self, id: u32, index: u32, name: &str) -> Result<(), ErrorCode> {
        let name = CString::new(name);
        unsafe {
            match name {
                Ok(name) => {
                    check!(br::sc_internal_compiler_set_member_name(
                        self.sc_compiler,
                        id,
                        index,
                        name.as_ptr(),
                    ));
                }
                _ => return Err(ErrorCode::Unhandled),
            }
        }
        Ok(())
    }

    pub fn unset_decoration(
        &mut self,
        id: u32,
        decoration: spirv::Decoration,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_unset_decoration(
                self.sc_compiler,
                id,
                decoration,
            ));
        }

        Ok(())
    }

    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: spirv::Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_set_decoration(
                self.sc_compiler,
                id,
                decoration,
                argument,
            ));
        }

        Ok(())
    }

    pub fn get_entry_points(&self) -> Result<Vec<spirv::EntryPoint>, ErrorCode> {
        let mut entry_points_raw = ptr::null_mut();
        let mut entry_points_raw_length = 0;

        unsafe {
            check!(br::sc_internal_compiler_get_entry_points(
                self.sc_compiler,
                &mut entry_points_raw,
                &mut entry_points_raw_length,
            ));

            let entry_points = (0..entry_points_raw_length)
                .map(|offset| {
                    let entry_point_raw_ptr = entry_points_raw.add(offset);
                    let entry_point_raw = read_from_ptr::<br::ScEntryPoint>(entry_point_raw_ptr);
                    let name = read_string_from_ptr(entry_point_raw.name)?;
                    let entry_point = spirv::EntryPoint {
                        name,
                        execution_model: entry_point_raw.execution_model,
                        work_group_size: spirv::WorkGroupSize {
                            x: entry_point_raw.work_group_size_x,
                            y: entry_point_raw.work_group_size_y,
                            z: entry_point_raw.work_group_size_z,
                        },
                    };

                    check!(br::sc_internal_free_pointer(
                        entry_point_raw.name as *mut c_void,
                    ));

                    Ok(entry_point)
                })
                .collect::<Result<Vec<_>, _>>();

            check!(br::sc_internal_free_pointer(
                entry_points_raw as *mut c_void,
            ));

            entry_points
        }
    }

    pub fn get_active_buffer_ranges(&self, id: u32) -> Result<Vec<spirv::BufferRange>, ErrorCode> {
        let mut active_buffer_ranges_raw = ptr::null_mut();
        let mut active_buffer_ranges_raw_length = 0;

        unsafe {
            check!(br::sc_internal_compiler_get_active_buffer_ranges(
                self.sc_compiler,
                id,
                &mut active_buffer_ranges_raw,
                &mut active_buffer_ranges_raw_length,
            ));

            let active_buffer_ranges = (0..active_buffer_ranges_raw_length)
                .map(|offset| {
                    let active_buffer_range_raw_ptr = active_buffer_ranges_raw.add(offset);
                    let active_buffer_range_raw =
                        read_from_ptr::<br::ScBufferRange>(active_buffer_range_raw_ptr);
                    spirv::BufferRange {
                        index: active_buffer_range_raw.index,
                        offset: active_buffer_range_raw.offset,
                        range: active_buffer_range_raw.range,
                    }
                })
                .collect::<Vec<_>>();

            check!(br::sc_internal_free_pointer(
                active_buffer_ranges_raw as *mut c_void
            ));

            Ok(active_buffer_ranges)
        }
    }

    pub fn get_cleansed_entry_point_name(
        &self,
        entry_point_name: &str,
        execution_model: spirv::ExecutionModel,
    ) -> Result<String, ErrorCode> {
        let mut cleansed_ptr = ptr::null();
        let entry_point = CString::new(entry_point_name);
        match entry_point {
            Ok(ep) => unsafe {
                check!(br::sc_internal_compiler_get_cleansed_entry_point_name(
                    self.sc_compiler,
                    ep.as_ptr(),
                    execution_model,
                    &mut cleansed_ptr
                ));
                let cleansed = read_string_from_ptr(cleansed_ptr)?;
                check!(br::sc_internal_free_pointer(cleansed_ptr as *mut c_void));
                Ok(cleansed)
            },
            _ => Err(ErrorCode::Unhandled),
        }
    }

    pub fn get_specialization_constants(
        &self,
    ) -> Result<Vec<spirv::SpecializationConstant>, ErrorCode> {
        let mut constants_raw = ptr::null_mut();
        let mut constants_raw_length = 0;

        unsafe {
            check!(br::sc_internal_compiler_get_specialization_constants(
                self.sc_compiler,
                &mut constants_raw,
                &mut constants_raw_length,
            ));

            let constants = (0..constants_raw_length)
                .map(|offset| {
                    let constant_raw_ptr = constants_raw.add(offset);
                    let constant_raw =
                        read_from_ptr::<br::ScSpecializationConstant>(constant_raw_ptr);

                    let constant = spirv::SpecializationConstant {
                        id: constant_raw.id,
                        constant_id: constant_raw.constant_id,
                    };

                    Ok(constant)
                })
                .collect::<Result<Vec<_>, _>>();

            check!(br::sc_internal_free_pointer(constants_raw as *mut c_void));

            constants
        }
    }

    pub fn set_scalar_constant(&self, id: u32, value: u64) -> Result<(), ErrorCode> {
        let high_bits = (value >> 32) as u32;
        let low_bits = value as u32;
        unsafe {
            check!(br::sc_internal_compiler_set_scalar_constant(
                self.sc_compiler,
                id,
                high_bits,
                low_bits,
            ));
        }

        Ok(())
    }

    pub fn get_type(&self, id: u32) -> Result<spirv::Type, ErrorCode> {
        unsafe {
            let mut type_ptr = std::mem::zeroed();

            check!(br::sc_internal_compiler_get_type(
                self.sc_compiler,
                id,
                &mut type_ptr,
            ));

            let raw = read_from_ptr::<br::ScType>(type_ptr);
            let member_types = read_into_vec_from_ptr(raw.member_types, raw.member_types_size);
            let array = read_into_vec_from_ptr(raw.array, raw.array_size);
            let array_size_literal = read_into_vec_from_ptr(raw.array_size_literal, raw.array_size);
            let image = raw.image;
            let result = Type::from_raw(raw.type_, raw.vecsize, raw.columns, member_types, array, array_size_literal, image);

            if raw.member_types_size > 0 {
                check!(br::sc_internal_free_pointer(
                    raw.member_types as *mut c_void
                ));
            }
            if raw.array_size > 0 {
                check!(br::sc_internal_free_pointer(raw.array as *mut c_void));
            }
            check!(br::sc_internal_free_pointer(type_ptr as *mut c_void));

            Ok(result)
        }
    }

    pub fn get_member_name(&self, id: u32, index: u32) -> Result<String, ErrorCode> {
        unsafe {
            let mut name_ptr = ptr::null();
            check!(br::sc_internal_compiler_get_member_name(
                self.sc_compiler,
                id,
                index,
                &mut name_ptr,
            ));
            let name = read_string_from_ptr(name_ptr)?;
            check!(br::sc_internal_free_pointer(name_ptr as *mut c_void));
            Ok(name)
        }
    }

    pub fn get_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
    ) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_member_decoration(
                self.sc_compiler,
                id,
                index,
                decoration,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn set_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_set_member_decoration(
                self.sc_compiler,
                id,
                index,
                decoration,
                argument,
            ));
        }

        Ok(())
    }

    pub fn get_declared_struct_size(&self, id: u32) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_declared_struct_size(
                self.sc_compiler,
                id,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn get_declared_struct_member_size(&self, id: u32, index: u32) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(br::sc_internal_compiler_get_declared_struct_member_size(
                self.sc_compiler,
                id,
                index,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn get_shader_resources(&self) -> Result<spirv::ShaderResources, ErrorCode> {
        unsafe {
            let mut shader_resources_raw = MaybeUninit::uninit();
            check!(br::sc_internal_compiler_get_shader_resources(
                self.sc_compiler,
                shader_resources_raw.as_mut_ptr(),
            ));
            let shader_resources_raw = shader_resources_raw.assume_init();

            let fill_resources = |array_raw: &br::ScResourceArray| {
                let resources = (0..array_raw.num)
                    .map(|i| {
                        let resource_raw = read_from_ptr::<br::ScResource>(array_raw.data.add(i));
                        let name = read_string_from_ptr(resource_raw.name)?;
                        check!(br::sc_internal_free_pointer(
                            resource_raw.name as *mut c_void,
                        ));

                        Ok(spirv::Resource {
                            id: resource_raw.id,
                            type_id: resource_raw.type_id,
                            base_type_id: resource_raw.base_type_id,
                            name,
                        })
                    })
                    .collect::<Result<Vec<_>, ErrorCode>>();

                check!(br::sc_internal_free_pointer(array_raw.data as *mut c_void));

                resources
            };

            let fill_builtin_resources = |array_raw: &br::ScBuiltInResourceArray| {
                let resources = (0..array_raw.num)
                    .map(|i| {
                        let resource_raw = read_from_ptr::<br::ScBuiltInResource>(array_raw.data.add(i));
                        let name = read_string_from_ptr(resource_raw.resource.name)?;
                        check!(br::sc_internal_free_pointer(
                            resource_raw.resource.name as *mut c_void,
                        ));

                        Ok(spirv::BuiltInResource {
                            builtin: spirv::BuiltIn::from_raw(resource_raw.builtin),
                            value_type_id: resource_raw.value_type_id,
                            resource: spirv::Resource {
                                id: resource_raw.resource.id,
                                type_id: resource_raw.resource.type_id,
                                base_type_id: resource_raw.resource.base_type_id,
                                name,
                            },
                        })
                    })
                    .collect::<Result<Vec<_>, ErrorCode>>();

                check!(br::sc_internal_free_pointer(array_raw.data as *mut c_void));

                resources
            };

            let uniform_buffers = fill_resources(&shader_resources_raw.uniform_buffers)?;
            let storage_buffers = fill_resources(&shader_resources_raw.storage_buffers)?;
            let stage_inputs = fill_resources(&shader_resources_raw.stage_inputs)?;
            let stage_outputs = fill_resources(&shader_resources_raw.stage_outputs)?;
            let subpass_inputs = fill_resources(&shader_resources_raw.subpass_inputs)?;
            let storage_images = fill_resources(&shader_resources_raw.storage_images)?;
            let sampled_images = fill_resources(&shader_resources_raw.sampled_images)?;
            let atomic_counters = fill_resources(&shader_resources_raw.atomic_counters)?;
            let acceleration_structures = fill_resources(&shader_resources_raw.acceleration_structures)?;
            let push_constant_buffers =
                fill_resources(&shader_resources_raw.push_constant_buffers)?;
            let shader_record_buffers = fill_resources(&shader_resources_raw.shader_record_buffers)?;
            let separate_images = fill_resources(&shader_resources_raw.separate_images)?;
            let separate_samplers = fill_resources(&shader_resources_raw.separate_samplers)?;
            let builtin_inputs = fill_builtin_resources(&shader_resources_raw.builtin_inputs)?;
            let builtin_outputs = fill_builtin_resources(&shader_resources_raw.builtin_outputs)?;

            Ok(spirv::ShaderResources {
                uniform_buffers,
                storage_buffers,
                stage_inputs,
                stage_outputs,
                subpass_inputs,
                storage_images,
                sampled_images,
                atomic_counters,
                acceleration_structures,
                push_constant_buffers,
                shader_record_buffers,
                separate_images,
                separate_samplers,
                builtin_inputs,
                builtin_outputs,
            })
        }
    }

    pub fn get_active_interface_variables(&self) -> Result<HashSet<u32>, ErrorCode> {
        unsafe {
            let mut ids: *mut u32 = ptr::null_mut();
            let mut size: usize = 0;
            check!(br::sc_internal_compiler_get_active_interface_variables(
                self.sc_compiler,
                &mut ids,
                &mut size
            ));
            let result: HashSet<u32> = std::slice::from_raw_parts(ids, size)
                .iter()
                .cloned()
                .collect();
            check!(br::sc_internal_free_pointer(ids as *mut c_void));
            Ok(result)
        }
    }

    pub fn rename_interface_variable(
        &self,
        resources: &[spirv::Resource],
        location: u32,
        new_name: &str,
    ) -> Result<(), ErrorCode> {
        unsafe {
            let mut resources_names = Vec::new();
            for resource in resources.iter() {
                match CString::new(&*resource.name) {
                    Ok(rn) => resources_names.push(rn),
                    Err(_) => return Err(ErrorCode::Unhandled),
                }
            }

            let new_name = CString::new(new_name).map_err(|_| ErrorCode::Unhandled)?;
            let new_name_ptr = new_name.as_ptr();
            let resources = resources
                .iter()
                .enumerate()
                .map(|(i, r)| br::ScResource {
                    id: r.id,
                    type_id: r.type_id,
                    base_type_id: r.base_type_id,
                    name: resources_names[i].as_ptr() as _,
                })
                .collect::<Vec<_>>();
            let resources_ptr = resources.as_ptr();

            check!(br::sc_internal_compiler_rename_interface_variable(
                self.sc_compiler,
                resources_ptr,
                resources_names.len(),
                location,
                new_name_ptr,
            ));

            Ok(())
        }
    }

    pub fn get_work_group_size_specialization_constants(
        &self,
    ) -> Result<spirv::WorkGroupSizeSpecializationConstants, ErrorCode> {
        let mut constants_raw = ptr::null_mut();

        unsafe {
            check!(
                br::sc_internal_compiler_get_work_group_size_specialization_constants(
                    self.sc_compiler,
                    &mut constants_raw,
                )
            );

            let x = read_from_ptr::<br::ScSpecializationConstant>(constants_raw.offset(0));
            let y = read_from_ptr::<br::ScSpecializationConstant>(constants_raw.offset(1));
            let z = read_from_ptr::<br::ScSpecializationConstant>(constants_raw.offset(2));

            let constants = spirv::WorkGroupSizeSpecializationConstants {
                x: spirv::SpecializationConstant {
                    id: x.id,
                    constant_id: x.constant_id,
                },
                y: spirv::SpecializationConstant {
                    id: y.id,
                    constant_id: y.constant_id,
                },
                z: spirv::SpecializationConstant {
                    id: z.id,
                    constant_id: z.constant_id,
                },
            };

            check!(br::sc_internal_free_pointer(constants_raw as *mut c_void));

            Ok(constants)
        }
    }
}

impl<TTargetData> Drop for Compiler<TTargetData> {
    fn drop(&mut self) {
        unsafe {
            br::sc_internal_compiler_delete(self.sc_compiler);
        }
    }
}
