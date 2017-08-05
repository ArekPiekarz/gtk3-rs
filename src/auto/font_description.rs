// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

use FontMask;
use Gravity;
use Stretch;
use Style;
use Variant;
use Weight;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontDescription(Boxed<ffi::PangoFontDescription>);

    match fn {
        copy => |ptr| ffi::pango_font_description_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_font_description_free(ptr),
        get_type => || ffi::pango_font_description_get_type(),
    }
}

impl FontDescription {
    pub fn new() -> FontDescription {
        unsafe {
            from_glib_full(ffi::pango_font_description_new())
        }
    }

    pub fn better_match<'a, P: Into<Option<&'a FontDescription>>>(&self, old_match: P, new_match: &FontDescription) -> bool {
        let old_match = old_match.into();
        let old_match = old_match.to_glib_none();
        unsafe {
            from_glib(ffi::pango_font_description_better_match(self.to_glib_none().0, old_match.0, new_match.to_glib_none().0))
        }
    }

    fn equal(&self, desc2: &FontDescription) -> bool {
        unsafe {
            from_glib(ffi::pango_font_description_equal(self.to_glib_none().0, desc2.to_glib_none().0))
        }
    }

    pub fn get_family(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_description_get_family(self.to_glib_none().0))
        }
    }

    pub fn get_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_font_description_get_gravity(self.to_glib_none().0))
        }
    }

    pub fn get_set_fields(&self) -> FontMask {
        unsafe {
            from_glib(ffi::pango_font_description_get_set_fields(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::pango_font_description_get_size(self.to_glib_none().0)
        }
    }

    pub fn get_size_is_absolute(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_description_get_size_is_absolute(self.to_glib_none().0))
        }
    }

    pub fn get_stretch(&self) -> Stretch {
        unsafe {
            from_glib(ffi::pango_font_description_get_stretch(self.to_glib_none().0))
        }
    }

    pub fn get_style(&self) -> Style {
        unsafe {
            from_glib(ffi::pango_font_description_get_style(self.to_glib_none().0))
        }
    }

    pub fn get_variant(&self) -> Variant {
        unsafe {
            from_glib(ffi::pango_font_description_get_variant(self.to_glib_none().0))
        }
    }

    pub fn get_weight(&self) -> Weight {
        unsafe {
            from_glib(ffi::pango_font_description_get_weight(self.to_glib_none().0))
        }
    }

    pub fn hash(&self) -> u32 {
        unsafe {
            ffi::pango_font_description_hash(self.to_glib_none().0)
        }
    }

    pub fn merge<'a, P: Into<Option<&'a FontDescription>>>(&mut self, desc_to_merge: P, replace_existing: bool) {
        let desc_to_merge = desc_to_merge.into();
        let desc_to_merge = desc_to_merge.to_glib_none();
        unsafe {
            ffi::pango_font_description_merge(self.to_glib_none_mut().0, desc_to_merge.0, replace_existing.to_glib());
        }
    }

    pub fn set_absolute_size(&mut self, size: f64) {
        unsafe {
            ffi::pango_font_description_set_absolute_size(self.to_glib_none_mut().0, size);
        }
    }

    pub fn set_family(&mut self, family: &str) {
        unsafe {
            ffi::pango_font_description_set_family(self.to_glib_none_mut().0, family.to_glib_none().0);
        }
    }

    pub fn set_gravity(&mut self, gravity: Gravity) {
        unsafe {
            ffi::pango_font_description_set_gravity(self.to_glib_none_mut().0, gravity.to_glib());
        }
    }

    pub fn set_size(&mut self, size: i32) {
        unsafe {
            ffi::pango_font_description_set_size(self.to_glib_none_mut().0, size);
        }
    }

    pub fn set_stretch(&mut self, stretch: Stretch) {
        unsafe {
            ffi::pango_font_description_set_stretch(self.to_glib_none_mut().0, stretch.to_glib());
        }
    }

    pub fn set_style(&mut self, style: Style) {
        unsafe {
            ffi::pango_font_description_set_style(self.to_glib_none_mut().0, style.to_glib());
        }
    }

    pub fn set_variant(&mut self, variant: Variant) {
        unsafe {
            ffi::pango_font_description_set_variant(self.to_glib_none_mut().0, variant.to_glib());
        }
    }

    pub fn set_weight(&mut self, weight: Weight) {
        unsafe {
            ffi::pango_font_description_set_weight(self.to_glib_none_mut().0, weight.to_glib());
        }
    }

    pub fn to_filename(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::pango_font_description_to_filename(self.to_glib_none().0))
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::pango_font_description_to_string(self.to_glib_none().0))
        }
    }

    pub fn unset_fields(&mut self, to_unset: FontMask) {
        unsafe {
            ffi::pango_font_description_unset_fields(self.to_glib_none_mut().0, to_unset.to_glib());
        }
    }

    pub fn from_string(str: &str) -> FontDescription {
        unsafe {
            from_glib_full(ffi::pango_font_description_from_string(str.to_glib_none().0))
        }
    }
}

impl Default for FontDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for FontDescription {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for FontDescription {}

impl fmt::Display for FontDescription {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
