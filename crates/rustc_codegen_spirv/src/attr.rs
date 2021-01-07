//! `#[spirv(...)]` attribute support.
//!
//! The attribute-checking parts of this try to follow `rustc_passes::check_attr`.

use crate::symbols::{SpirvAttribute, Symbols};
use rustc_ast::Attribute;
use rustc_hir as hir;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::{self, NestedVisitorMap, Visitor};
use rustc_hir::{HirId, MethodKind, Target, CRATE_HIR_ID};
use rustc_middle::hir::map::Map;
use rustc_middle::ty::query::Providers;
use rustc_middle::ty::TyCtxt;
use std::rc::Rc;

// FIXME(eddyb) make this reusable from somewhere in `rustc`.
pub(crate) fn target_from_impl_item<'tcx>(
    tcx: TyCtxt<'tcx>,
    impl_item: &hir::ImplItem<'_>,
) -> Target {
    match impl_item.kind {
        hir::ImplItemKind::Const(..) => Target::AssocConst,
        hir::ImplItemKind::Fn(..) => {
            let parent_hir_id = tcx.hir().get_parent_item(impl_item.hir_id());
            let containing_item = tcx.hir().expect_item(parent_hir_id);
            let containing_impl_is_for_trait = match &containing_item.kind {
                hir::ItemKind::Impl(hir::Impl { of_trait, .. }) => of_trait.is_some(),
                _ => unreachable!("parent of an ImplItem must be an Impl"),
            };
            if containing_impl_is_for_trait {
                Target::Method(MethodKind::Trait { body: true })
            } else {
                Target::Method(MethodKind::Inherent)
            }
        }
        hir::ImplItemKind::TyAlias(..) => Target::AssocTy,
    }
}

struct CheckSpirvAttrVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    sym: Rc<Symbols>,
}

impl CheckSpirvAttrVisitor<'_> {
    fn check_spirv_attributes(&self, hir_id: HirId, attrs: &[Attribute], target: Target) {
        let parse_attrs = |attrs| crate::symbols::parse_attrs_for_checking(&self.sym, attrs);

        for (attr, parse_attr_result) in parse_attrs(attrs) {
            // Make sure to mark the whole `#[spirv(...)]` attribute as used,
            // to avoid warnings about unused attributes.
            self.tcx.sess.mark_attr_used(attr);

            let (span, parsed_attr) = match parse_attr_result {
                Ok(span_and_parsed_attr) => span_and_parsed_attr,
                Err((span, msg)) => {
                    self.tcx.sess.span_err(span, &msg);
                    continue;
                }
            };

            /// Error newtype marker used below for readability.
            struct Expected<T>(T);

            let valid_target = match parsed_attr {
                SpirvAttribute::Builtin(_)
                | SpirvAttribute::DescriptorSet(_)
                | SpirvAttribute::Binding(_)
                | SpirvAttribute::Flat => match target {
                    Target::Param => {
                        let parent_hir_id = self.tcx.hir().get_parent_node(hir_id);
                        let parent_is_entry_point =
                            parse_attrs(self.tcx.hir().attrs(parent_hir_id))
                                .filter_map(|(_, r)| r.ok())
                                .any(|(_, attr)| matches!(attr, SpirvAttribute::Entry(_)));
                        if !parent_is_entry_point {
                            self.tcx.sess.span_err(
                                span,
                                "attribute is only valid on a parameter of an entry-point function",
                            );
                        }
                        Ok(())
                    }

                    _ => Err(Expected("function parameter")),
                },

                SpirvAttribute::Entry(_) => match target {
                    Target::Fn
                    | Target::Method(MethodKind::Trait { body: true })
                    | Target::Method(MethodKind::Inherent) => {
                        // FIXME(eddyb) further check entry-point attribute validity,
                        // e.g. signature, shouldn't have `#[inline]` or generics, etc.
                        Ok(())
                    }

                    _ => Err(Expected("function")),
                },

                SpirvAttribute::UnrollLoops => match target {
                    Target::Fn
                    | Target::Closure
                    | Target::Method(MethodKind::Trait { body: true })
                    | Target::Method(MethodKind::Inherent) => Ok(()),

                    _ => Err(Expected("function or closure")),
                },

                SpirvAttribute::StorageClass(_)
                | SpirvAttribute::ImageType
                | SpirvAttribute::Sampler
                | SpirvAttribute::SampledImage
                | SpirvAttribute::Block => match target {
                    Target::Struct => {
                        // FIXME(eddyb) further check type attribute validity,
                        // e.g. layout, generics, other attributes, etc.
                        Ok(())
                    }

                    _ => Err(Expected("struct")),
                },
            };
            match valid_target {
                Ok(()) => {}
                Err(Expected(expected_target)) => self.tcx.sess.span_err(
                    span,
                    &format!(
                        "attribute is only valid on a {}, not on a {}",
                        expected_target, target
                    ),
                ),
            }
        }
    }
}

// FIXME(eddyb) DRY this somehow and make it reusable from somewhere in `rustc`.
impl<'tcx> Visitor<'tcx> for CheckSpirvAttrVisitor<'tcx> {
    type Map = Map<'tcx>;

    fn nested_visit_map(&mut self) -> NestedVisitorMap<Self::Map> {
        NestedVisitorMap::OnlyBodies(self.tcx.hir())
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
        let target = Target::from_item(item);
        self.check_spirv_attributes(item.hir_id(), item.attrs, target);
        intravisit::walk_item(self, item)
    }

    fn visit_generic_param(&mut self, generic_param: &'tcx hir::GenericParam<'tcx>) {
        let target = Target::from_generic_param(generic_param);
        self.check_spirv_attributes(generic_param.hir_id, generic_param.attrs, target);
        intravisit::walk_generic_param(self, generic_param)
    }

    fn visit_trait_item(&mut self, trait_item: &'tcx hir::TraitItem<'tcx>) {
        let target = Target::from_trait_item(trait_item);
        self.check_spirv_attributes(trait_item.hir_id(), trait_item.attrs, target);
        intravisit::walk_trait_item(self, trait_item)
    }

    fn visit_struct_field(&mut self, struct_field: &'tcx hir::StructField<'tcx>) {
        self.check_spirv_attributes(struct_field.hir_id, struct_field.attrs, Target::Field);
        intravisit::walk_struct_field(self, struct_field);
    }

    fn visit_arm(&mut self, arm: &'tcx hir::Arm<'tcx>) {
        self.check_spirv_attributes(arm.hir_id, arm.attrs, Target::Arm);
        intravisit::walk_arm(self, arm);
    }

    fn visit_foreign_item(&mut self, f_item: &'tcx hir::ForeignItem<'tcx>) {
        let target = Target::from_foreign_item(f_item);
        self.check_spirv_attributes(f_item.hir_id(), f_item.attrs, target);
        intravisit::walk_foreign_item(self, f_item)
    }

    fn visit_impl_item(&mut self, impl_item: &'tcx hir::ImplItem<'tcx>) {
        let target = target_from_impl_item(self.tcx, impl_item);
        self.check_spirv_attributes(impl_item.hir_id(), impl_item.attrs, target);
        intravisit::walk_impl_item(self, impl_item)
    }

    fn visit_stmt(&mut self, stmt: &'tcx hir::Stmt<'tcx>) {
        // When checking statements ignore expressions, they will be checked later.
        if let hir::StmtKind::Local(l) = stmt.kind {
            self.check_spirv_attributes(l.hir_id, &l.attrs, Target::Statement);
        }
        intravisit::walk_stmt(self, stmt)
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        let target = match expr.kind {
            hir::ExprKind::Closure(..) => Target::Closure,
            _ => Target::Expression,
        };

        self.check_spirv_attributes(expr.hir_id, &expr.attrs, target);
        intravisit::walk_expr(self, expr)
    }

    fn visit_variant(
        &mut self,
        variant: &'tcx hir::Variant<'tcx>,
        generics: &'tcx hir::Generics<'tcx>,
        item_id: HirId,
    ) {
        self.check_spirv_attributes(variant.id, variant.attrs, Target::Variant);
        intravisit::walk_variant(self, variant, generics, item_id)
    }

    fn visit_macro_def(&mut self, macro_def: &'tcx hir::MacroDef<'tcx>) {
        self.check_spirv_attributes(macro_def.hir_id(), macro_def.attrs, Target::MacroDef);
        intravisit::walk_macro_def(self, macro_def);
    }

    fn visit_param(&mut self, param: &'tcx hir::Param<'tcx>) {
        self.check_spirv_attributes(param.hir_id, param.attrs, Target::Param);

        intravisit::walk_param(self, param);
    }
}

fn check_invalid_macro_level_spirv_attr(tcx: TyCtxt<'_>, sym: &Symbols, attrs: &[Attribute]) {
    for attr in attrs {
        if tcx.sess.check_name(attr, sym.spirv) {
            tcx.sess
                .span_err(attr.span, "#[spirv(..)] cannot be applied to a macro");
        }
    }
}

// FIXME(eddyb) DRY this somehow and make it reusable from somewhere in `rustc`.
fn check_mod_attrs(tcx: TyCtxt<'_>, module_def_id: LocalDefId) {
    let check_spirv_attr_visitor = &mut CheckSpirvAttrVisitor {
        tcx,
        sym: Symbols::get(),
    };
    tcx.hir().visit_item_likes_in_module(
        module_def_id,
        &mut check_spirv_attr_visitor.as_deep_visitor(),
    );
    // FIXME(eddyb) use `tcx.hir().visit_exported_macros_in_krate(...)` after rustup.
    for id in tcx.hir().krate().exported_macros {
        check_spirv_attr_visitor.visit_macro_def(match tcx.hir().find(id.hir_id()) {
            Some(hir::Node::MacroDef(macro_def)) => macro_def,
            _ => unreachable!(),
        });
    }
    check_invalid_macro_level_spirv_attr(
        tcx,
        &check_spirv_attr_visitor.sym,
        tcx.hir().krate().non_exported_macro_attrs,
    );
    if module_def_id.is_top_level_module() {
        check_spirv_attr_visitor.check_spirv_attributes(
            CRATE_HIR_ID,
            tcx.hir().krate_attrs(),
            Target::Mod,
        );
    }
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers {
        check_mod_attrs: |tcx, def_id| {
            // Run both the default checks, and our `#[spirv(...)]` ones.
            (rustc_interface::DEFAULT_QUERY_PROVIDERS.check_mod_attrs)(tcx, def_id);
            check_mod_attrs(tcx, def_id)
        },
        ..*providers
    };
}