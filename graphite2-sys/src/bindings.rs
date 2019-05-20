/* automatically generated by rust-bindgen */

pub type gr_uint8 = ::std::os::raw::c_uchar;
pub type gr_byte = gr_uint8;
pub type gr_int8 = ::std::os::raw::c_schar;
pub type gr_uint16 = ::std::os::raw::c_ushort;
pub type gr_int16 = ::std::os::raw::c_short;
pub type gr_uint32 = ::std::os::raw::c_uint;
pub type gr_int32 = ::std::os::raw::c_int;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_encform {
    gr_utf8 = 1,
    gr_utf16 = 2,
    gr_utf32 = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_face {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_font {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_feature_ref {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_feature_val {
    _unused: [u8; 0],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_face_options {
    gr_face_default = 0,
    gr_face_dumbRendering = 1,
    gr_face_preloadGlyphs = 2,
    gr_face_cacheCmap = 4,
    gr_face_preloadAll = 6,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct gr_faceinfo {
    pub _bindgen_opaque_blob: [u32; 4usize],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_faceinfo_gr_space_contextuals {
    gr_space_unknown = 0,
    gr_space_none = 1,
    gr_space_left_only = 2,
    gr_space_right_only = 3,
    gr_space_either_only = 4,
    gr_space_both = 5,
    gr_space_cross = 6,
}
#[test]
fn bindgen_test_layout_gr_faceinfo() {
    assert_eq!(
        ::std::mem::size_of::<gr_faceinfo>(),
        16usize,
        concat!("Size of: ", stringify!(gr_faceinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<gr_faceinfo>(),
        4usize,
        concat!("Alignment of ", stringify!(gr_faceinfo))
    );
}
pub type gr_get_table_fn = ::std::option::Option<
    unsafe extern "C" fn(
        appFaceHandle: *const ::std::os::raw::c_void,
        name: ::std::os::raw::c_uint,
        len: *mut usize,
    ) -> *const ::std::os::raw::c_void,
>;
pub type gr_release_table_fn = ::std::option::Option<
    unsafe extern "C" fn(
        appFaceHandle: *const ::std::os::raw::c_void,
        table_buffer: *const ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_face_ops {
    pub size: usize,
    pub get_table: gr_get_table_fn,
    pub release_table: gr_release_table_fn,
}
#[test]
fn bindgen_test_layout_gr_face_ops() {
    assert_eq!(
        ::std::mem::size_of::<gr_face_ops>(),
        ::std::mem::size_of::<[usize; 3usize]>(),
        concat!("Size of: ", stringify!(gr_face_ops))
    );
    assert_eq!(
        ::std::mem::align_of::<gr_face_ops>(),
        ::std::mem::size_of::<usize>(),
        concat!("Alignment of ", stringify!(gr_face_ops))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gr_face_ops>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gr_face_ops),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gr_face_ops>())).get_table as *const _ as usize },
        ::std::mem::size_of::<usize>(),
        concat!(
            "Offset of field: ",
            stringify!(gr_face_ops),
            "::",
            stringify!(get_table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gr_face_ops>())).release_table as *const _ as usize },
        ::std::mem::size_of::<[usize; 2usize]>(),
        concat!(
            "Offset of field: ",
            stringify!(gr_face_ops),
            "::",
            stringify!(release_table)
        )
    );
}
pub type gr_advance_fn = ::std::option::Option<
    unsafe extern "C" fn(appFontHandle: *const ::std::os::raw::c_void, glyphid: gr_uint16) -> f32,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_font_ops {
    pub size: usize,
    pub glyph_advance_x: gr_advance_fn,
    pub glyph_advance_y: gr_advance_fn,
}
#[test]
fn bindgen_test_layout_gr_font_ops() {
    assert_eq!(
        ::std::mem::size_of::<gr_font_ops>(),
        ::std::mem::size_of::<[usize; 3usize]>(),
        concat!("Size of: ", stringify!(gr_font_ops))
    );
    assert_eq!(
        ::std::mem::align_of::<gr_font_ops>(),
        ::std::mem::size_of::<usize>(),
        concat!("Alignment of ", stringify!(gr_font_ops))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gr_font_ops>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gr_font_ops),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gr_font_ops>())).glyph_advance_x as *const _ as usize },
        ::std::mem::size_of::<usize>(),
        concat!(
            "Offset of field: ",
            stringify!(gr_font_ops),
            "::",
            stringify!(glyph_advance_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gr_font_ops>())).glyph_advance_y as *const _ as usize },
        ::std::mem::size_of::<[usize; 2usize]>(),
        concat!(
            "Offset of field: ",
            stringify!(gr_font_ops),
            "::",
            stringify!(glyph_advance_y)
        )
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_break_weight {
    gr_breakNone = 0,
    gr_breakWhitespace = 10,
    gr_breakWord = 15,
    gr_breakIntra = 20,
    gr_breakLetter = 30,
    gr_breakClip = 40,
    gr_breakBeforeWhitespace = -10,
    gr_breakBeforeWord = -15,
    gr_breakBeforeIntra = -20,
    gr_breakBeforeLetter = -30,
    gr_breakBeforeClip = -40,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_justFlags {
    #[doc = " Indicates that this segment is a complete line"]
    gr_justCompleteLine = 0,
    #[doc = " Indicates that the start of the slot list is not at the start of a line"]
    gr_justStartInline = 1,
    #[doc = " Indicates that the end of the slot list is not at the end of a line"]
    gr_justEndInline = 2,
}
#[repr(u32)]
#[doc = " Used for looking up slot attributes. Most are already available in other functions"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_attrCode {
    #[doc = " adjusted glyph advance in x direction in design units"]
    gr_slatAdvX = 0,
    #[doc = " adjusted glyph advance in y direction (usually 0) in design units"]
    gr_slatAdvY = 1,
    #[doc = " returns 0. Deprecated."]
    gr_slatAttTo = 2,
    #[doc = " This slot attaches to its parent at the given design units in the x direction"]
    gr_slatAttX = 3,
    #[doc = " This slot attaches to its parent at the given design units in the y direction"]
    gr_slatAttY = 4,
    #[doc = " This slot attaches to its parent at the given glyph point (not implemented)"]
    gr_slatAttGpt = 5,
    #[doc = " x-direction adjustment from the given glyph point (not implemented)"]
    gr_slatAttXOff = 6,
    #[doc = " y-direction adjustment from the given glyph point (not implemented)"]
    gr_slatAttYOff = 7,
    #[doc = " Where on this glyph should align with the attachment point on the parent glyph in the x-direction."]
    gr_slatAttWithX = 8,
    #[doc = " Where on this glyph should align with the attachment point on the parent glyph in the y-direction"]
    gr_slatAttWithY = 9,
    #[doc = " Which glyph point on this glyph should align with the attachment point on the parent glyph (not implemented)."]
    gr_slatWithGpt = 10,
    #[doc = " Adjustment to gr_slatWithGpt in x-direction (not implemented)"]
    gr_slatAttWithXOff = 11,
    #[doc = " Adjustment to gr_slatWithGpt in y-direction (not implemented)"]
    gr_slatAttWithYOff = 12,
    #[doc = " Attach at given nesting level (not implemented)"]
    gr_slatAttLevel = 13,
    #[doc = " Line break breakweight for this glyph"]
    gr_slatBreak = 14,
    #[doc = " Ligature component reference (not implemented)"]
    gr_slatCompRef = 15,
    #[doc = " bidi directionality of this glyph (not implemented)"]
    gr_slatDir = 16,
    #[doc = " Whether insertion is allowed before this glyph"]
    gr_slatInsert = 17,
    #[doc = " Final positioned position of this glyph relative to its parent in x-direction in pixels"]
    gr_slatPosX = 18,
    #[doc = " Final positioned position of this glyph relative to its parent in y-direction in pixels"]
    gr_slatPosY = 19,
    #[doc = " Amount to shift glyph by in x-direction design units"]
    gr_slatShiftX = 20,
    #[doc = " Amount to shift glyph by in y-direction design units"]
    gr_slatShiftY = 21,
    #[doc = " attribute user1"]
    gr_slatUserDefnV1 = 22,
    #[doc = " not implemented"]
    gr_slatMeasureSol = 23,
    #[doc = " not implemented"]
    gr_slatMeasureEol = 24,
    #[doc = " Amount this slot can stretch (not implemented)"]
    gr_slatJStretch = 25,
    #[doc = " Amount this slot can shrink (not implemented)"]
    gr_slatJShrink = 26,
    #[doc = " Granularity by which this slot can stretch or shrink (not implemented)"]
    gr_slatJStep = 27,
    #[doc = " Justification weight for this glyph (not implemented)"]
    gr_slatJWeight = 28,
    #[doc = " Amount this slot mush shrink or stretch in design units"]
    gr_slatJWidth = 29,
    #[doc = " SubSegment split point"]
    gr_slatSegSplit = 54,
    #[doc = " User defined attribute, see subattr for user attr number"]
    gr_slatUserDefn = 55,
    #[doc = " Bidi level"]
    gr_slatBidiLevel = 56,
    #[doc = " Collision flags"]
    gr_slatColFlags = 57,
    #[doc = " Collision constraint rectangle left (bl.x)"]
    gr_slatColLimitblx = 58,
    #[doc = " Collision constraint rectangle lower (bl.y)"]
    gr_slatColLimitbly = 59,
    #[doc = " Collision constraint rectangle right (tr.x)"]
    gr_slatColLimittrx = 60,
    #[doc = " Collision constraint rectangle upper (tr.y)"]
    gr_slatColLimittry = 61,
    #[doc = " Collision shift x"]
    gr_slatColShiftx = 62,
    #[doc = " Collision shift y"]
    gr_slatColShifty = 63,
    #[doc = " Collision margin"]
    gr_slatColMargin = 64,
    #[doc = " Margin cost weight"]
    gr_slatColMarginWt = 65,
    #[doc = " Margin cost weight"]
    gr_slatColExclGlyph = 66,
    #[doc = " Margin cost weight"]
    gr_slatColExclOffx = 67,
    #[doc = " Margin cost weight"]
    gr_slatColExclOffy = 68,
    #[doc = " Margin cost weight"]
    gr_slatSeqClass = 69,
    #[doc = " Margin cost weight"]
    gr_slatSeqProxClass = 70,
    #[doc = " Margin cost weight"]
    gr_slatSeqOrder = 71,
    #[doc = " Margin cost weight"]
    gr_slatSeqAboveXoff = 72,
    #[doc = " Margin cost weight"]
    gr_slatSeqAboveWt = 73,
    #[doc = " Margin cost weight"]
    gr_slatSeqBelowXlim = 74,
    #[doc = " Margin cost weight"]
    gr_slatSeqBelowWt = 75,
    #[doc = " Margin cost weight"]
    gr_slatSeqValignHt = 76,
    #[doc = " Margin cost weight"]
    gr_slatSeqValignWt = 77,
    #[doc = " not implemented"]
    gr_slatMax = 78,
    #[doc = " not implemented"]
    gr_slatNoEffect = 79,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gr_bidirtl {
    #[doc = " Underlying paragraph direction is RTL"]
    gr_rtl = 1,
    #[doc = " Set this to not run the bidi pass internally, even if the font asks for it."]
    #[doc = " This presumes that the segment is in a single direction. Most of the time"]
    #[doc = " this bit should be set unless you know you are passing full paragraphs of text."]
    gr_nobidi = 2,
    #[doc = " Disable auto mirroring for rtl text"]
    gr_nomirror = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_char_info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_segment {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_slot {
    _unused: [u8; 0],
}