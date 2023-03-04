use std::collections::HashSet;
use crate::{compiler, ErrorCode};
use std::marker::PhantomData;

/// A stage or compute kernel.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CombinedImageSampler {
    pub combined_id: u32,
    pub image_id: u32,
    pub sampler_id: u32,
}

/// A stage or compute kernel.
pub use crate::bindings::spv::ExecutionModel;

/// A decoration.
pub use crate::bindings::spv::Decoration;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum BuiltIn {
    Position,
    PointSize,
    ClipDistance,
    CullDistance,
    VertexId,
    InstanceId,
    PrimitiveId,
    InvocationId,
    Layer,
    ViewportIndex,
    TessLevelOuter,
    TessLevelInner,
    TessCoord,
    PatchVertices,
    FragCoord,
    PointCoord,
    FrontFacing,
    SampleId,
    SamplePosition,
    SampleMask,
    FragDepth,
    HelperInvocation,
    NumWorkgroups,
    WorkgroupSize,
    WorkgroupId,
    LocalInvocationId,
    GlobalInvocationId,
    LocalInvocationIndex,
    WorkDim,
    GlobalSize,
    EnqueuedWorkgroupSize,
    GlobalOffset,
    GlobalLinearId,
    SubgroupSize,
    SubgroupMaxSize,
    NumSubgroups,
    NumEnqueuedSubgroups,
    SubgroupId,
    SubgroupLocalInvocationId,
    VertexIndex,
    InstanceIndex,
    SubgroupEqMask,
    SubgroupGeMask,
    SubgroupGtMask,
    SubgroupLeMask,
    SubgroupLtMask,
    BaseVertex,
    BaseInstance,
    DrawIndex,
    PrimitiveShadingRateKhr,
    DeviceIndex,
    ViewIndex,
    ShadingRateKhr,
    BaryCoordNoPerspAmd,
    BaryCoordNoPerspCentroidAmd,
    BaryCoordNoPerspSampleAmd,
    BaryCoordSmoothAmd,
    BaryCoordSmoothCentroidAmd,
    BaryCoordSmoothSampleAmd,
    BaryCoordPullModelAmd,
    FragStencilRefExt,
    ViewportMaskNv,
    SecondaryPositionNv,
    SecondaryViewportMaskNv,
    PositionPerViewNv,
    ViewportMaskPerViewNv,
    FullyCoveredExt,
    TaskCountNv,
    PrimitiveCountNv,
    PrimitiveIndicesNv,
    ClipDistancePerViewNv,
    CullDistancePerViewNv,
    LayerPerViewNv,
    MeshViewCountNv,
    MeshViewIndicesNv,
    BaryCoordKhr,
    BaryCoordNoPerspKhr,
    FragSizeExt,
    FragInvocationCountExt,
    PrimitivePointIndicesExt,
    PrimitiveLineIndicesExt,
    PrimitiveTriangleIndicesExt,
    CullPrimitiveExt,
    LaunchIdKhr,
    LaunchSizeKhr,
    WorldRayOriginKhr,
    WorldRayDirectionKhr,
    ObjectRayOriginKhr,
    ObjectRayDirectionKhr,
    RayTminKhr,
    RayTmaxKhr,
    InstanceCustomIndexKhr,
    ObjectToWorldKhr,
    WorldToObjectKhr,
    HitTnv,
    HitKindKhr,
    CurrentRayTimeNv,
    IncomingRayFlagsKhr,
    RayGeometryIndexKhr,
    WarpsPerSmnv,
    SmCountNv,
    WarpIdnv,
    Smidnv,
    CullMaskKhr,
}

impl BuiltIn {
    pub(crate) fn from_raw(v: spv::BuiltIn) -> Self {
        match v {
            spv::BuiltIn::Position => Self::Position,
            spv::BuiltIn::PointSize => Self::PointSize,
            spv::BuiltIn::ClipDistance => Self::ClipDistance,
            spv::BuiltIn::CullDistance => Self::CullDistance,
            spv::BuiltIn::VertexId => Self::VertexId,
            spv::BuiltIn::InstanceId => Self::InstanceId,
            spv::BuiltIn::PrimitiveId => Self::PrimitiveId,
            spv::BuiltIn::InvocationId => Self::InvocationId,
            spv::BuiltIn::Layer => Self::Layer,
            spv::BuiltIn::ViewportIndex => Self::ViewportIndex,
            spv::BuiltIn::TessLevelOuter => Self::TessLevelOuter,
            spv::BuiltIn::TessLevelInner => Self::TessLevelInner,
            spv::BuiltIn::TessCoord => Self::TessCoord,
            spv::BuiltIn::PatchVertices => Self::PatchVertices,
            spv::BuiltIn::FragCoord => Self::FragCoord,
            spv::BuiltIn::PointCoord => Self::PointCoord,
            spv::BuiltIn::FrontFacing => Self::FrontFacing,
            spv::BuiltIn::SampleId => Self::SampleId,
            spv::BuiltIn::SamplePosition => Self::SamplePosition,
            spv::BuiltIn::SampleMask => Self::SampleMask,
            spv::BuiltIn::FragDepth => Self::FragDepth,
            spv::BuiltIn::HelperInvocation => Self::HelperInvocation,
            spv::BuiltIn::NumWorkgroups => Self::NumWorkgroups,
            spv::BuiltIn::WorkgroupSize => Self::WorkgroupSize,
            spv::BuiltIn::WorkgroupId => Self::WorkgroupId,
            spv::BuiltIn::LocalInvocationId => Self::LocalInvocationId,
            spv::BuiltIn::GlobalInvocationId => Self::GlobalInvocationId,
            spv::BuiltIn::LocalInvocationIndex => Self::LocalInvocationIndex,
            spv::BuiltIn::WorkDim => Self::WorkDim,
            spv::BuiltIn::GlobalSize => Self::GlobalSize,
            spv::BuiltIn::EnqueuedWorkgroupSize => Self::EnqueuedWorkgroupSize,
            spv::BuiltIn::GlobalOffset => Self::GlobalOffset,
            spv::BuiltIn::GlobalLinearId => Self::GlobalLinearId,
            spv::BuiltIn::SubgroupSize => Self::SubgroupSize,
            spv::BuiltIn::SubgroupMaxSize => Self::SubgroupMaxSize,
            spv::BuiltIn::NumSubgroups => Self::NumSubgroups,
            spv::BuiltIn::NumEnqueuedSubgroups => Self::NumEnqueuedSubgroups,
            spv::BuiltIn::SubgroupId => Self::SubgroupId,
            spv::BuiltIn::SubgroupLocalInvocationId => Self::SubgroupLocalInvocationId,
            spv::BuiltIn::VertexIndex => Self::VertexIndex,
            spv::BuiltIn::InstanceIndex => Self::InstanceIndex,
            spv::BuiltIn::SubgroupEqMask => Self::SubgroupEqMask,
            spv::BuiltIn::SubgroupGeMask => Self::SubgroupGeMask,
            spv::BuiltIn::SubgroupGtMask => Self::SubgroupGtMask,
            spv::BuiltIn::SubgroupLeMask => Self::SubgroupLeMask,
            spv::BuiltIn::SubgroupLtMask => Self::SubgroupLtMask,
            spv::BuiltIn::BaseVertex => Self::BaseVertex,
            spv::BuiltIn::BaseInstance => Self::BaseInstance,
            spv::BuiltIn::DrawIndex => Self::DrawIndex,
            spv::BuiltIn::PrimitiveShadingRateKhr => Self::PrimitiveShadingRateKhr,
            spv::BuiltIn::DeviceIndex => Self::DeviceIndex,
            spv::BuiltIn::ViewIndex => Self::ViewIndex,
            spv::BuiltIn::ShadingRateKhr => Self::ShadingRateKhr,
            spv::BuiltIn::BaryCoordNoPerspAmd => Self::BaryCoordNoPerspAmd,
            spv::BuiltIn::BaryCoordNoPerspCentroidAmd => Self::BaryCoordNoPerspCentroidAmd,
            spv::BuiltIn::BaryCoordNoPerspSampleAmd => Self::BaryCoordNoPerspSampleAmd,
            spv::BuiltIn::BaryCoordSmoothAmd => Self::BaryCoordSmoothAmd,
            spv::BuiltIn::BaryCoordSmoothCentroidAmd => Self::BaryCoordSmoothCentroidAmd,
            spv::BuiltIn::BaryCoordSmoothSampleAmd => Self::BaryCoordSmoothSampleAmd,
            spv::BuiltIn::BaryCoordPullModelAmd => Self::BaryCoordPullModelAmd,
            spv::BuiltIn::FragStencilRefExt => Self::FragStencilRefExt,
            spv::BuiltIn::ViewportMaskNv => Self::ViewportMaskNv,
            spv::BuiltIn::SecondaryPositionNv => Self::SecondaryPositionNv,
            spv::BuiltIn::SecondaryViewportMaskNv => Self::SecondaryViewportMaskNv,
            spv::BuiltIn::PositionPerViewNv => Self::PositionPerViewNv,
            spv::BuiltIn::ViewportMaskPerViewNv => Self::ViewportMaskPerViewNv,
            spv::BuiltIn::FullyCoveredExt => Self::FullyCoveredExt,
            spv::BuiltIn::TaskCountNv => Self::TaskCountNv,
            spv::BuiltIn::PrimitiveCountNv => Self::PrimitiveCountNv,
            spv::BuiltIn::PrimitiveIndicesNv => Self::PrimitiveIndicesNv,
            spv::BuiltIn::ClipDistancePerViewNv => Self::ClipDistancePerViewNv,
            spv::BuiltIn::CullDistancePerViewNv => Self::CullDistancePerViewNv,
            spv::BuiltIn::LayerPerViewNv => Self::LayerPerViewNv,
            spv::BuiltIn::MeshViewCountNv => Self::MeshViewCountNv,
            spv::BuiltIn::MeshViewIndicesNv => Self::MeshViewIndicesNv,
            spv::BuiltIn::BaryCoordKhr => Self::BaryCoordKhr,
            spv::BuiltIn::BaryCoordNoPerspKhr => Self::BaryCoordNoPerspKhr,
            spv::BuiltIn::FragSizeExt => Self::FragSizeExt,
            spv::BuiltIn::FragInvocationCountExt => Self::FragInvocationCountExt,
            spv::BuiltIn::PrimitivePointIndicesExt => Self::PrimitivePointIndicesExt,
            spv::BuiltIn::PrimitiveLineIndicesExt => Self::PrimitiveLineIndicesExt,
            spv::BuiltIn::PrimitiveTriangleIndicesExt => Self::PrimitiveTriangleIndicesExt,
            spv::BuiltIn::CullPrimitiveExt => Self::CullPrimitiveExt,
            spv::BuiltIn::LaunchIdKhr => Self::LaunchIdKhr,
            spv::BuiltIn::LaunchSizeKhr => Self::LaunchSizeKhr,
            spv::BuiltIn::WorldRayOriginKhr => Self::WorldRayOriginKhr,
            spv::BuiltIn::WorldRayDirectionKhr => Self::WorldRayDirectionKhr,
            spv::BuiltIn::ObjectRayOriginKhr => Self::ObjectRayOriginKhr,
            spv::BuiltIn::ObjectRayDirectionKhr => Self::ObjectRayDirectionKhr,
            spv::BuiltIn::RayTminKhr => Self::RayTminKhr,
            spv::BuiltIn::RayTmaxKhr => Self::RayTmaxKhr,
            spv::BuiltIn::InstanceCustomIndexKhr => Self::InstanceCustomIndexKhr,
            spv::BuiltIn::ObjectToWorldKhr => Self::ObjectToWorldKhr,
            spv::BuiltIn::WorldToObjectKhr => Self::WorldToObjectKhr,
            spv::BuiltIn::HitTnv => Self::HitTnv,
            spv::BuiltIn::HitKindKhr => Self::HitKindKhr,
            spv::BuiltIn::CurrentRayTimeNv => Self::CurrentRayTimeNv,
            spv::BuiltIn::IncomingRayFlagsKhr => Self::IncomingRayFlagsKhr,
            spv::BuiltIn::RayGeometryIndexKhr => Self::RayGeometryIndexKhr,
            spv::BuiltIn::WarpsPerSmnv => Self::WarpsPerSmnv,
            spv::BuiltIn::SmCountNv => Self::SmCountNv,
            spv::BuiltIn::WarpIdnv => Self::WarpIdnv,
            spv::BuiltIn::Smidnv => Self::Smidnv,
            spv::BuiltIn::CullMaskKhr => Self::CullMaskKhr,
            spv::BuiltIn::Max => unreachable!("invalid builtin")
        }
    }
}

#[cfg(feature = "msl")]
pub(crate) fn built_in_as_raw(built_in: Option<BuiltIn>) -> crate::bindings::spv::BuiltIn {
    use crate::bindings as br;
    use BuiltIn::*;
    match built_in {
        None => br::spv::BuiltIn::Max,
        Some(Position) => spv::BuiltIn::Position,
        Some(PointSize) => spv::BuiltIn::PointSize,
        Some(ClipDistance) => spv::BuiltIn::ClipDistance,
        Some(CullDistance) => spv::BuiltIn::CullDistance,
        Some(VertexId) => spv::BuiltIn::VertexId,
        Some(InstanceId) => spv::BuiltIn::InstanceId,
        Some(PrimitiveId) => spv::BuiltIn::PrimitiveId,
        Some(InvocationId) => spv::BuiltIn::InvocationId,
        Some(Layer) => spv::BuiltIn::Layer,
        Some(ViewportIndex) => spv::BuiltIn::ViewportIndex,
        Some(TessLevelOuter) => spv::BuiltIn::TessLevelOuter,
        Some(TessLevelInner) => spv::BuiltIn::TessLevelInner,
        Some(TessCoord) => spv::BuiltIn::TessCoord,
        Some(PatchVertices) => spv::BuiltIn::PatchVertices,
        Some(FragCoord) => spv::BuiltIn::FragCoord,
        Some(PointCoord) => spv::BuiltIn::PointCoord,
        Some(FrontFacing) => spv::BuiltIn::FrontFacing,
        Some(SampleId) => spv::BuiltIn::SampleId,
        Some(SamplePosition) => spv::BuiltIn::SamplePosition,
        Some(SampleMask) => spv::BuiltIn::SampleMask,
        Some(FragDepth) => spv::BuiltIn::FragDepth,
        Some(HelperInvocation) => spv::BuiltIn::HelperInvocation,
        Some(NumWorkgroups) => spv::BuiltIn::NumWorkgroups,
        Some(WorkgroupSize) => spv::BuiltIn::WorkgroupSize,
        Some(WorkgroupId) => spv::BuiltIn::WorkgroupId,
        Some(LocalInvocationId) => spv::BuiltIn::LocalInvocationId,
        Some(GlobalInvocationId) => spv::BuiltIn::GlobalInvocationId,
        Some(LocalInvocationIndex) => spv::BuiltIn::LocalInvocationIndex,
        Some(WorkDim) => spv::BuiltIn::WorkDim,
        Some(GlobalSize) => spv::BuiltIn::GlobalSize,
        Some(EnqueuedWorkgroupSize) => spv::BuiltIn::EnqueuedWorkgroupSize,
        Some(GlobalOffset) => spv::BuiltIn::GlobalOffset,
        Some(GlobalLinearId) => spv::BuiltIn::GlobalLinearId,
        Some(SubgroupSize) => spv::BuiltIn::SubgroupSize,
        Some(SubgroupMaxSize) => spv::BuiltIn::SubgroupMaxSize,
        Some(NumSubgroups) => spv::BuiltIn::NumSubgroups,
        Some(NumEnqueuedSubgroups) => spv::BuiltIn::NumEnqueuedSubgroups,
        Some(SubgroupId) => spv::BuiltIn::SubgroupId,
        Some(SubgroupLocalInvocationId) => spv::BuiltIn::SubgroupLocalInvocationId,
        Some(VertexIndex) => spv::BuiltIn::VertexIndex,
        Some(InstanceIndex) => spv::BuiltIn::InstanceIndex,
        Some(SubgroupEqMask) => spv::BuiltIn::SubgroupEqMask,
        Some(SubgroupGeMask) => spv::BuiltIn::SubgroupGeMask,
        Some(SubgroupGtMask) => spv::BuiltIn::SubgroupGtMask,
        Some(SubgroupLeMask) => spv::BuiltIn::SubgroupLeMask,
        Some(SubgroupLtMask) => spv::BuiltIn::SubgroupLtMask,
        Some(BaseVertex) => spv::BuiltIn::BaseVertex,
        Some(BaseInstance) => spv::BuiltIn::BaseInstance,
        Some(DrawIndex) => spv::BuiltIn::DrawIndex,
        Some(PrimitiveShadingRateKhr) => spv::BuiltIn::PrimitiveShadingRateKhr,
        Some(DeviceIndex) => spv::BuiltIn::DeviceIndex,
        Some(ViewIndex) => spv::BuiltIn::ViewIndex,
        Some(ShadingRateKhr) => spv::BuiltIn::ShadingRateKhr,
        Some(BaryCoordNoPerspAmd) => spv::BuiltIn::BaryCoordNoPerspAmd,
        Some(BaryCoordNoPerspCentroidAmd) => spv::BuiltIn::BaryCoordNoPerspCentroidAmd,
        Some(BaryCoordNoPerspSampleAmd) => spv::BuiltIn::BaryCoordNoPerspSampleAmd,
        Some(BaryCoordSmoothAmd) => spv::BuiltIn::BaryCoordSmoothAmd,
        Some(BaryCoordSmoothCentroidAmd) => spv::BuiltIn::BaryCoordSmoothCentroidAmd,
        Some(BaryCoordSmoothSampleAmd) => spv::BuiltIn::BaryCoordSmoothSampleAmd,
        Some(BaryCoordPullModelAmd) => spv::BuiltIn::BaryCoordPullModelAmd,
        Some(FragStencilRefExt) => spv::BuiltIn::FragStencilRefExt,
        Some(ViewportMaskNv) => spv::BuiltIn::ViewportMaskNv,
        Some(SecondaryPositionNv) => spv::BuiltIn::SecondaryPositionNv,
        Some(SecondaryViewportMaskNv) => spv::BuiltIn::SecondaryViewportMaskNv,
        Some(PositionPerViewNv) => spv::BuiltIn::PositionPerViewNv,
        Some(ViewportMaskPerViewNv) => spv::BuiltIn::ViewportMaskPerViewNv,
        Some(FullyCoveredExt) => spv::BuiltIn::FullyCoveredExt,
        Some(TaskCountNv) => spv::BuiltIn::TaskCountNv,
        Some(PrimitiveCountNv) => spv::BuiltIn::PrimitiveCountNv,
        Some(PrimitiveIndicesNv) => spv::BuiltIn::PrimitiveIndicesNv,
        Some(ClipDistancePerViewNv) => spv::BuiltIn::ClipDistancePerViewNv,
        Some(CullDistancePerViewNv) => spv::BuiltIn::CullDistancePerViewNv,
        Some(LayerPerViewNv) => spv::BuiltIn::LayerPerViewNv,
        Some(MeshViewCountNv) => spv::BuiltIn::MeshViewCountNv,
        Some(MeshViewIndicesNv) => spv::BuiltIn::MeshViewIndicesNv,
        Some(BaryCoordKhr) => spv::BuiltIn::BaryCoordKhr,
        Some(BaryCoordNoPerspKhr) => spv::BuiltIn::BaryCoordNoPerspKhr,
        Some(FragSizeExt) => spv::BuiltIn::FragSizeExt,
        Some(FragInvocationCountExt) => spv::BuiltIn::FragInvocationCountExt,
        Some(PrimitivePointIndicesExt) => spv::BuiltIn::PrimitivePointIndicesExt,
        Some(PrimitiveLineIndicesExt) => spv::BuiltIn::PrimitiveLineIndicesExt,
        Some(PrimitiveTriangleIndicesExt) => spv::BuiltIn::PrimitiveTriangleIndicesExt,
        Some(CullPrimitiveExt) => spv::BuiltIn::CullPrimitiveExt,
        Some(LaunchIdKhr) => spv::BuiltIn::LaunchIdKhr,
        Some(LaunchSizeKhr) => spv::BuiltIn::LaunchSizeKhr,
        Some(WorldRayOriginKhr) => spv::BuiltIn::WorldRayOriginKhr,
        Some(WorldRayDirectionKhr) => spv::BuiltIn::WorldRayDirectionKhr,
        Some(ObjectRayOriginKhr) => spv::BuiltIn::ObjectRayOriginKhr,
        Some(ObjectRayDirectionKhr) => spv::BuiltIn::ObjectRayDirectionKhr,
        Some(RayTminKhr) => spv::BuiltIn::RayTminKhr,
        Some(RayTmaxKhr) => spv::BuiltIn::RayTmaxKhr,
        Some(InstanceCustomIndexKhr) => spv::BuiltIn::InstanceCustomIndexKhr,
        Some(ObjectToWorldKhr) => spv::BuiltIn::ObjectToWorldKhr,
        Some(WorldToObjectKhr) => spv::BuiltIn::WorldToObjectKhr,
        Some(HitTnv) => spv::BuiltIn::HitTnv,
        Some(HitKindKhr) => spv::BuiltIn::HitKindKhr,
        Some(CurrentRayTimeNv) => spv::BuiltIn::CurrentRayTimeNv,
        Some(IncomingRayFlagsKhr) => spv::BuiltIn::IncomingRayFlagsKhr,
        Some(RayGeometryIndexKhr) => spv::BuiltIn::RayGeometryIndexKhr,
        Some(WarpsPerSmnv) => spv::BuiltIn::WarpsPerSmnv,
        Some(SmCountNv) => spv::BuiltIn::SmCountNv,
        Some(WarpIdnv) => spv::BuiltIn::WarpIdnv,
        Some(Smidnv) => spv::BuiltIn::Smidnv,
        Some(CullMaskKhr) => spv::BuiltIn::CullMaskKhr,
    }
}

/// A work group size.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkGroupSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// An entry point for a SPIR-V module.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EntryPoint {
    pub name: String,
    pub execution_model: ExecutionModel,
    pub work_group_size: WorkGroupSize,
}

/// Description of struct member's range.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BufferRange {
    /// An index. Useful for passing to `get_member_name` and `get_member_decoration`.
    pub index: u32,
    /// Bytes from start of buffer not beginning of struct.
    pub offset: usize,
    /// Size of field in bytes.
    pub range: usize,
}

/// A resource.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Resource {
    pub id: u32,
    pub type_id: u32,
    pub base_type_id: u32,
    pub name: String,
}

/// A built-in resource.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BuiltInResource {
    pub builtin: BuiltIn,
    pub value_type_id: u32,
    pub resource: Resource,
}

/// Specialization constant reference.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SpecializationConstant {
    pub id: u32,
    pub constant_id: u32,
}

/// Work group size specialization constants.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkGroupSizeSpecializationConstants {
    pub x: SpecializationConstant,
    pub y: SpecializationConstant,
    pub z: SpecializationConstant,
}

/// Shader resources.
#[derive(Debug, Clone)]
pub struct ShaderResources {
    pub uniform_buffers: Vec<Resource>,
    pub storage_buffers: Vec<Resource>,
    pub stage_inputs: Vec<Resource>,
    pub stage_outputs: Vec<Resource>,
    pub subpass_inputs: Vec<Resource>,
    pub storage_images: Vec<Resource>,
    pub sampled_images: Vec<Resource>,
    pub atomic_counters: Vec<Resource>,
    pub acceleration_structures: Vec<Resource>,
    pub push_constant_buffers: Vec<Resource>,
    pub shader_record_buffers: Vec<Resource>,
    pub separate_images: Vec<Resource>,
    pub separate_samplers: Vec<Resource>,
    pub builtin_inputs: Vec<BuiltInResource>,
    pub builtin_outputs: Vec<BuiltInResource>,
}

pub use crate::bindings::spv::Dim;
pub use crate::bindings::spv::ImageFormat;
pub use crate::bindings::spirv_cross::SPIRType_BaseType;
use crate::bindings::spv;

#[derive(Debug, Clone)]
pub struct ImageType {
    pub type_id: u32,
    pub dim: Dim,
    pub depth: bool,
    pub arrayed: bool,
    pub ms: bool,
    pub sampled: u32,
    pub format: ImageFormat,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Type {
    // TODO: Add missing fields to relevant variants from SPIRType
    Unknown,
    Void,
    Boolean {
        vecsize: u32,
        columns: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Char {
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Int {
        vecsize: u32,
        columns: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    UInt {
        vecsize: u32,
        columns: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Int64 {
        vecsize: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    UInt64 {
        vecsize: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    AtomicCounter {
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Half {
        vecsize: u32,
        columns: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Float {
        vecsize: u32,
        columns: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Double {
        vecsize: u32,
        columns: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Struct {
        member_types: Vec<u32>,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Image {
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
        image: ImageType,
    },
    SampledImage {
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
        image: ImageType,
    },
    Sampler {
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    SByte {
        vecsize: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    UByte {
        vecsize: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    Short {
        vecsize: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    UShort {
        vecsize: u32,
        array: Vec<u32>,
        array_size_literal: Vec<bool>,
    },
    ControlPointArray,
    AccelerationStructure,
    RayQuery,
    Interpolant,
}

/// A SPIR-V shader module.
#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub(crate) words: &'a [u32],
}

impl<'a> Module<'a> {
    /// Creates a shader module from SPIR-V words.
    pub fn from_words(words: &[u32]) -> Module {
        Module { words }
    }
}

pub trait Target {
    type Data;
}

/// An abstract syntax tree that corresponds to a SPIR-V module.
pub struct Ast<TTarget>
    where
        TTarget: Target,
{
    pub(crate) compiler: compiler::Compiler<TTarget::Data>,
    pub(crate) target_type: PhantomData<TTarget>,
}

pub trait Parse<TTarget>: Sized {
    fn parse(module: &Module) -> Result<Self, ErrorCode>;
}

pub trait Compile<TTarget> {
    type CompilerOptions;

    fn set_compiler_options(
        &mut self,
        compiler_options: &Self::CompilerOptions,
    ) -> Result<(), ErrorCode>;
    fn compile(&mut self) -> Result<String, ErrorCode>;
}

impl<TTarget> Ast<TTarget>
    where
        Self: Parse<TTarget> + Compile<TTarget>,
        TTarget: Target,
{
    /// Gets a decoration.
    pub fn get_decoration(&self, id: u32, decoration: Decoration) -> Result<u32, ErrorCode> {
        self.compiler.get_decoration(id, decoration)
    }

    /// Gets a name. If not defined, an empty string will be returned.
    pub fn get_name(&mut self, id: u32) -> Result<String, ErrorCode> {
        self.compiler.get_name(id)
    }

    /// Sets a name.
    pub fn set_name(&mut self, id: u32, name: &str) -> Result<(), ErrorCode> {
        self.compiler.set_name(id, name)
    }

    /// Sets a member name.
    pub fn set_member_name(&mut self, id: u32, index: u32, name: &str) -> Result<(), ErrorCode> {
        self.compiler.set_member_name(id, index, name)
    }

    /// Unsets a decoration.
    pub fn unset_decoration(&mut self, id: u32, decoration: Decoration) -> Result<(), ErrorCode> {
        self.compiler.unset_decoration(id, decoration)
    }

    /// Sets a decoration.
    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        self.compiler.set_decoration(id, decoration, argument)
    }

    /// Gets entry points.
    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        self.compiler.get_entry_points()
    }

    /// Gets cleansed entry point names. `compile` must be called first.
    pub fn get_cleansed_entry_point_name(
        &self,
        entry_point_name: &str,
        execution_model: ExecutionModel,
    ) -> Result<String, ErrorCode> {
        if self.compiler.has_been_compiled {
            self.compiler
                .get_cleansed_entry_point_name(entry_point_name, execution_model)
        } else {
            Err(ErrorCode::CompilationError(String::from(
                "`compile` must be called first",
            )))
        }
    }

    /// Gets active buffer ragnes.  Useful for push constants.
    pub fn get_active_buffer_ranges(&self, id: u32) -> Result<Vec<BufferRange>, ErrorCode> {
        self.compiler.get_active_buffer_ranges(id)
    }

    /// Gets all specialization constants.
    pub fn get_specialization_constants(&self) -> Result<Vec<SpecializationConstant>, ErrorCode> {
        self.compiler.get_specialization_constants()
    }

    /// Set reference of a scalar constant to a value, overriding the default.
    ///
    /// Can be used to override specialization constants.
    pub fn set_scalar_constant(&mut self, id: u32, value: u64) -> Result<(), ErrorCode> {
        self.compiler.set_scalar_constant(id, value)
    }

    /// Gets shader resources.
    pub fn get_shader_resources(&self) -> Result<ShaderResources, ErrorCode> {
        self.compiler.get_shader_resources()
    }

    /// Gets the SPIR-V type associated with an ID.
    pub fn get_type(&self, id: u32) -> Result<Type, ErrorCode> {
        self.compiler.get_type(id)
    }

    /// Gets the identifier for a member located at `index` within an `OpTypeStruct`.
    pub fn get_member_name(&self, id: u32, index: u32) -> Result<String, ErrorCode> {
        self.compiler.get_member_name(id, index)
    }

    /// Gets a decoration for a member located at `index` within an `OpTypeStruct`.
    pub fn get_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
    ) -> Result<u32, ErrorCode> {
        self.compiler.get_member_decoration(id, index, decoration)
    }

    /// Sets a decoration for a member located at `index` within an `OpTypeStruct`.
    pub fn set_member_decoration(
        &mut self,
        id: u32,
        index: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        self.compiler
            .set_member_decoration(id, index, decoration, argument)
    }

    /// Gets the effective size of a buffer block.
    pub fn get_declared_struct_size(&self, id: u32) -> Result<u32, ErrorCode> {
        self.compiler.get_declared_struct_size(id)
    }

    /// Gets the effective size of a buffer block struct member.
    pub fn get_declared_struct_member_size(&self, id: u32, index: u32) -> Result<u32, ErrorCode> {
        self.compiler.get_declared_struct_member_size(id, index)
    }

    /// Renames an interface variable.
    pub fn rename_interface_variable(
        &mut self,
        resources: &[Resource],
        location: u32,
        name: &str,
    ) -> Result<(), ErrorCode> {
        self.compiler
            .rename_interface_variable(resources, location, name)
    }

    /// get the active interface variable.
    pub fn get_active_interface_variables(&mut self) -> Result<HashSet<u32>, ErrorCode> {
        self.compiler
            .get_active_interface_variables()
    }

    /// Gets work group size specialization constants.
    pub fn get_work_group_size_specialization_constants(
        &self,
    ) -> Result<WorkGroupSizeSpecializationConstants, ErrorCode> {
        self.compiler.get_work_group_size_specialization_constants()
    }

    /// Parses a module into `Ast`.
    pub fn parse(module: &Module) -> Result<Self, ErrorCode> {
        Parse::<TTarget>::parse(module)
    }

    /// Sets compile options.
    pub fn set_compiler_options(
        &mut self,
        options: &<Self as Compile<TTarget>>::CompilerOptions,
    ) -> Result<(), ErrorCode> {
        Compile::<TTarget>::set_compiler_options(self, options)
    }

    /// Compiles an abstract syntax tree to a `String` in the specified `TTarget` language.
    pub fn compile(&mut self) -> Result<String, ErrorCode> {
        self.compiler.has_been_compiled = true;
        Compile::<TTarget>::compile(self)
    }
}
