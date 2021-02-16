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
