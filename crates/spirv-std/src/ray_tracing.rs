//! Ray-tracing data types

/// An acceleration structure type which is an opaque reference to an
/// acceleration structure handle as defined in the client API specification.
#[spirv(acceleration_structure_khr)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureKhr {
    pub(crate) _private: (),
}
