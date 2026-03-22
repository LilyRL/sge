#![feature(allocator_api)]

use ref_type::gen_ref_type;

pub struct Obj;

gen_ref_type!(Obj, ObjRef, object);

fn main() {}
