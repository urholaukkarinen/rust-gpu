use rustc_middle::bug;

use super::Builder;
use crate::builder_spirv::{BuilderCursor, SpirvValue, SpirvValueExt};
use crate::spirv_type::SpirvType;
use rspirv::spirv::Word;
use rustc_target::abi::{Abi, Align, Scalar, Size};

use crate::rustc_codegen_ssa::traits::BuilderMethods;

impl<'a, 'tcx> Builder<'a, 'tcx> {
    // walk down every member in the ADT recursively and load their values as uints
    // this will break up larger data types into uint sized sections, for
    // each load, this also has an offset in dwords.
    fn recurse_adt_for_stores(
        &mut self,
        uint_ty: u32,
        val: SpirvValue,
        base_offset: u32,
        uint_values_and_offsets: &mut Vec<(u32, SpirvValue)>,
    ) {
        let ty = self.lookup_type(val.ty);

        match ty {
            SpirvType::Adt {
                ref field_types,
                ref field_offsets,
                ..
            } => {
                for (element_idx, (ty, offset)) in
                    field_types.iter().zip(field_offsets.iter()).enumerate()
                {
                    let load_res = self.extract_value(val, element_idx as u64);

                    let offset = offset.bytes() as u32 / 4;
                    self.recurse_adt_for_stores(
                        uint_ty,
                        load_res,
                        base_offset + offset,
                        uint_values_and_offsets,
                    );
                }
            }
            SpirvType::Integer(bits, signed) => {
                /*
                         %25 = OpUConvert %uint %ulong_8702358937509254
                         %26 = OpShiftRightLogical %ulong %ulong_8702358937509254 %uint_32
                         %27 = OpUConvert %uint %26
                */

                let val_def = val.def(self);

                let bitcast_res = self
                    .emit()
                    .bitcast(uint_ty, None, val_def)
                    .unwrap()
                    .with_type(uint_ty);

                uint_values_and_offsets.push((base_offset, bitcast_res));
            }
            _ => {
                bug!(
                    "codegen_internal_buffer_store doesn't support this type: {:?}",
                    val
                );
            }
        }
    }

    pub(crate) fn codegen_internal_buffer_store(
        &mut self,
        result_type: Word,
        args: &[SpirvValue],
    ) -> SpirvValue {
        let uint_ty = SpirvType::Integer(32, false).def(rustc_span::DUMMY_SP, self);

        let uniform_uint_ptr =
            SpirvType::Pointer { pointee: uint_ty }.def(rustc_span::DUMMY_SP, self);

        let zero = self.constant_int(uint_ty, 0).def(self);

        let sets = self.bindless_descriptor_sets.borrow().unwrap();

        let mut data = self.lookup_type(args[2].ty);

        let bindless_idx = args[0].def(self);
        let offset_arg = args[1].def(self);

        let two = self.constant_int(uint_ty, 2).def(self);

        let dword_offset = self
            .emit()
            .shift_right_arithmetic(uint_ty, None, offset_arg, two)
            .unwrap();

        let mut uint_values_and_offsets = vec![];
        self.recurse_adt_for_stores(uint_ty, args[2], 0, &mut uint_values_and_offsets);

        for (offset, uint_value) in uint_values_and_offsets {
            let offset = if offset > 0 {
                let element_offset = self.constant_int(uint_ty, offset as u64).def(self);

                self.emit()
                    .i_add(uint_ty, None, dword_offset, element_offset)
                    .unwrap()
            } else {
                dword_offset
            };

            let indices = vec![bindless_idx, zero, offset];

            let access_chain = self
                .emit()
                .access_chain(uniform_uint_ptr, None, sets.buffers, indices)
                .unwrap()
                .with_type(uniform_uint_ptr);

            self.store(uint_value, access_chain, Align::from_bytes(0).unwrap());
        }

        self.emit_global()
            .type_void()
            .with_type(SpirvType::Void.def(rustc_span::DUMMY_SP, self))
    }

    pub(crate) fn codegen_internal_buffer_load(
        &mut self,
        result_type: Word,
        args: &[SpirvValue],
    ) -> SpirvValue {
        let uint_ty = SpirvType::Integer(32, false).def(rustc_span::DUMMY_SP, self);

        let uniform_uint_ptr =
            SpirvType::Pointer { pointee: uint_ty }.def(rustc_span::DUMMY_SP, self);

        let zero = self.constant_int(uint_ty, 0).def(self);

        let sets = self.bindless_descriptor_sets.borrow().unwrap();

        let member_accessor = |builder: &mut Self,
                               offset: u32,
                               dword_offset: u32,
                               bindless_idx: u32,
                               element_ty: u32|
         -> u32 {
            let offset = if offset > 0 {
                let element_offset = builder.constant_int(uint_ty, offset as u64).def(builder);

                builder
                    .emit()
                    .i_add(uint_ty, None, dword_offset, element_offset)
                    .unwrap()
            } else {
                dword_offset
            };

            let indices = vec![bindless_idx, zero, offset];

            let result = builder
                .emit()
                .access_chain(uniform_uint_ptr, None, sets.buffers, indices)
                .unwrap();

            let load_res = builder
                .emit()
                .load(uint_ty, None, result, None, std::iter::empty())
                .unwrap();

            let bitcast_res = builder.emit().bitcast(element_ty, None, load_res).unwrap();

            bitcast_res as u32
        };

        let load_type = |builder: &mut Self, result_type: u32, args: &[SpirvValue]| -> SpirvValue {
            let data = builder.lookup_type(result_type);

            let bindless_idx = args[0].def(builder);
            let offset_arg = args[1].def(builder);

            let two = builder.constant_int(uint_ty, 2).def(builder);

            let dword_offset = builder
                .emit()
                .shift_right_arithmetic(uint_ty, None, offset_arg, two)
                .unwrap();

            match data {
                SpirvType::Vector { count, element } => {
                    let mut composite_components = vec![];

                    for offset in 0..count {
                        composite_components.push(member_accessor(
                            builder,
                            offset,
                            dword_offset,
                            bindless_idx,
                            element,
                        ));
                    }

                    let adt = data.def(rustc_span::DUMMY_SP, builder);

                    builder
                        .emit()
                        .composite_construct(adt, None, composite_components)
                        .unwrap()
                        .with_type(adt)
                }
                SpirvType::Adt {
                    ref field_types,
                    ref field_offsets,
                    ..
                } => {
                    let mut composite_components = vec![];

                    for (ty, offset) in field_types.iter().zip(field_offsets.iter()) {
                        // jb-todo: this needs to recurse if `ty` is an Adt, or at least
                        // use OpCompositeExtract on each of those members recursively
                        composite_components.push(member_accessor(
                            builder,
                            offset.bytes() as u32 / 4,
                            dword_offset,
                            bindless_idx,
                            *ty,
                        ));
                    }

                    let adt = data.def(rustc_span::DUMMY_SP, builder);

                    builder
                        .emit()
                        .composite_construct(adt, None, composite_components)
                        .unwrap()
                        .with_type(adt)
                }
                SpirvType::Integer(bits, signed) => {
                    assert!(bits == 32); // jb-todo: 8, 16 and 64-bits
                    assert!(signed == false); // jb-todo: signed

                    let indices = vec![bindless_idx, zero, dword_offset];

                    let result = builder
                        .emit()
                        .access_chain(uniform_uint_ptr, None, sets.buffers, indices)
                        .unwrap();

                    let load_res = builder
                        .emit()
                        .load(uint_ty, None, result, None, std::iter::empty())
                        .unwrap();

                    load_res.with_type(uint_ty)
                }
                _ => {
                    bug!(
                        "Unhandled case for `internal_buffer_load` return / args: {:?}",
                        args
                    );
                }
            }
        };

        // simple structs are returned as values, complex ones are returned as out parameters
        match self.lookup_type(result_type) {
            SpirvType::Void => {
                if let SpirvType::Pointer { .. } = self.lookup_type(args[0].ty) {
                    let pointer = self.load(args[0], Align::from_bytes(0).unwrap());
                    let stuff = load_type(self, pointer.ty, &args[1..]);
                    self.store(stuff, args[0], Align::from_bytes(0).unwrap())
                } else {
                    bug!(
                        "Unhandled case for `internal_buffer_load` intrinsic / args: {:?}",
                        args
                    );
                }
            }
            _ => load_type(self, result_type, args),
        }
    }
}