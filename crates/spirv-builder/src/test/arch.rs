use super::val;

#[test]
fn any() {
    val(r#"

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    let vector = glam::BVec2::new(true, false);
    assert!(arch::any(vector));
}
"#);
}

#[test]
fn all() {
    val(r#"
#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    let vector = glam::BVec2::new(true, true);
    assert!(spirv_std::arch::all(vector));
}
"#);
}

#[test]
fn is_nan() {
    val(r#"

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    assert!(arch::is_nan(f32::NAN));
    assert!(!arch::is_nan(0.0));
    assert!(!arch::all::<glam::BVec2, 2>(arch::is_nan_vector(glam::Vec2::new(0.0, 1.0))));
}
"#);
}

#[test]
fn is_inf() {
    val(r#"
#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    let component: f32 = 0.0;
    let vector = glam::Vec2::new(component, 1.0);
    assert!(!arch::is_inf(component));
    assert!(!arch::all::<glam::BVec2, 2>(arch::is_inf_vector(vector)));
}
"#);
}

#[test]
fn is_finite() {
    val(r#"
#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    unsafe {
        asm! {
            "OpCapability Kernel"
        };
    }
    let component: f32 = 0.0;
    let vector = glam::Vec2::new(component, 1.0);
    assert!(arch::is_inf(component));
    assert!(arch::all::<glam::BVec2, 2>(arch::is_finite_vector(vector)));
}
"#);
}

#[test]
fn vector_extract_dynamic() {
    val(r#"
#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    let vector = glam::Vec2::new(1.0, 2.0);
    let element = unsafe { spirv_std::arch::vector_extract_dynamic(vector, 1) };
    assert!(2.0 == element);
}
"#);
}

#[test]
fn vector_insert_dynamic() {
    val(r#"
#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main() {
    let vector = glam::Vec2::new(1.0, 2.0);
    let expected = glam::Vec2::new(1.0, 3.0);
    let new_vector = unsafe { spirv_std::arch::vector_insert_dynamic(vector, 1, 3.0) };
    assert!(new_vector == expected);
}
"#);
}


