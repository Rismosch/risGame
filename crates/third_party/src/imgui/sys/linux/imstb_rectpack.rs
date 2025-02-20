#![allow(
non_camel_case_types,
dead_code,
non_upper_case_globals,
non_snake_case,
clashing_extern_declarations
)]

/* automatically generated by rust-bindgen 0.71.1 */

pub const STB_RECT_PACK_VERSION: u32 = 1;
pub const STBRP__MAXVAL: u32 = 2147483647;
pub type stbrp_coord = ::std::os::raw::c_int;
extern "C" {
    pub fn stbrp_pack_rects(
        context: *mut stbrp_context,
        rects: *mut stbrp_rect,
        num_rects: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stbrp_rect {
    pub id: ::std::os::raw::c_int,
    pub w: stbrp_coord,
    pub h: stbrp_coord,
    pub x: stbrp_coord,
    pub y: stbrp_coord,
    pub was_packed: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of stbrp_rect"][::std::mem::size_of::<stbrp_rect>() - 24usize];
    ["Alignment of stbrp_rect"][::std::mem::align_of::<stbrp_rect>() - 4usize];
    ["Offset of field: stbrp_rect::id"][::std::mem::offset_of!(stbrp_rect, id) - 0usize];
    ["Offset of field: stbrp_rect::w"][::std::mem::offset_of!(stbrp_rect, w) - 4usize];
    ["Offset of field: stbrp_rect::h"][::std::mem::offset_of!(stbrp_rect, h) - 8usize];
    ["Offset of field: stbrp_rect::x"][::std::mem::offset_of!(stbrp_rect, x) - 12usize];
    ["Offset of field: stbrp_rect::y"][::std::mem::offset_of!(stbrp_rect, y) - 16usize];
    ["Offset of field: stbrp_rect::was_packed"]
        [::std::mem::offset_of!(stbrp_rect, was_packed) - 20usize];
};
extern "C" {
    pub fn stbrp_init_target(
        context: *mut stbrp_context,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        nodes: *mut stbrp_node,
        num_nodes: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn stbrp_setup_allow_out_of_mem(
        context: *mut stbrp_context,
        allow_out_of_mem: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn stbrp_setup_heuristic(context: *mut stbrp_context, heuristic: ::std::os::raw::c_int);
}
pub const STBRP_HEURISTIC_Skyline_default: _bindgen_ty_1 = 0;
pub const STBRP_HEURISTIC_Skyline_BL_sortHeight: _bindgen_ty_1 = 0;
pub const STBRP_HEURISTIC_Skyline_BF_sortHeight: _bindgen_ty_1 = 1;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stbrp_node {
    pub x: stbrp_coord,
    pub y: stbrp_coord,
    pub next: *mut stbrp_node,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of stbrp_node"][::std::mem::size_of::<stbrp_node>() - 16usize];
    ["Alignment of stbrp_node"][::std::mem::align_of::<stbrp_node>() - 8usize];
    ["Offset of field: stbrp_node::x"][::std::mem::offset_of!(stbrp_node, x) - 0usize];
    ["Offset of field: stbrp_node::y"][::std::mem::offset_of!(stbrp_node, y) - 4usize];
    ["Offset of field: stbrp_node::next"][::std::mem::offset_of!(stbrp_node, next) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stbrp_context {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub align: ::std::os::raw::c_int,
    pub init_mode: ::std::os::raw::c_int,
    pub heuristic: ::std::os::raw::c_int,
    pub num_nodes: ::std::os::raw::c_int,
    pub active_head: *mut stbrp_node,
    pub free_head: *mut stbrp_node,
    pub extra: [stbrp_node; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of stbrp_context"][::std::mem::size_of::<stbrp_context>() - 72usize];
    ["Alignment of stbrp_context"][::std::mem::align_of::<stbrp_context>() - 8usize];
    ["Offset of field: stbrp_context::width"]
        [::std::mem::offset_of!(stbrp_context, width) - 0usize];
    ["Offset of field: stbrp_context::height"]
        [::std::mem::offset_of!(stbrp_context, height) - 4usize];
    ["Offset of field: stbrp_context::align"]
        [::std::mem::offset_of!(stbrp_context, align) - 8usize];
    ["Offset of field: stbrp_context::init_mode"]
        [::std::mem::offset_of!(stbrp_context, init_mode) - 12usize];
    ["Offset of field: stbrp_context::heuristic"]
        [::std::mem::offset_of!(stbrp_context, heuristic) - 16usize];
    ["Offset of field: stbrp_context::num_nodes"]
        [::std::mem::offset_of!(stbrp_context, num_nodes) - 20usize];
    ["Offset of field: stbrp_context::active_head"]
        [::std::mem::offset_of!(stbrp_context, active_head) - 24usize];
    ["Offset of field: stbrp_context::free_head"]
        [::std::mem::offset_of!(stbrp_context, free_head) - 32usize];
    ["Offset of field: stbrp_context::extra"]
        [::std::mem::offset_of!(stbrp_context, extra) - 40usize];
};
