// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FontMask;
use crate::Gravity;
use crate::Stretch;
use crate::Style;
use crate::Variant;
use crate::Weight;
use glib::translate::*;
use std::fmt;
use std::hash;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord)]
    pub struct FontDescription(Boxed<ffi::PangoFontDescription>);

    match fn {
        copy => |ptr| ffi::pango_font_description_copy(ptr),
        free => |ptr| ffi::pango_font_description_free(ptr),
        get_type => || ffi::pango_font_description_get_type(),
    }
}

impl FontDescription {
    #[doc(alias = "pango_font_description_new")]
    pub fn new() -> FontDescription {
        unsafe { from_glib_full(ffi::pango_font_description_new()) }
    }

    #[doc(alias = "pango_font_description_better_match")]
    pub fn better_match(
        &self,
        old_match: Option<&FontDescription>,
        new_match: &FontDescription,
    ) -> bool {
        unsafe {
            from_glib(ffi::pango_font_description_better_match(
                self.to_glib_none().0,
                old_match.to_glib_none().0,
                new_match.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_equal")]
    fn equal(&self, desc2: &FontDescription) -> bool {
        unsafe {
            from_glib(ffi::pango_font_description_equal(
                self.to_glib_none().0,
                desc2.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_family")]
    pub fn get_family(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::pango_font_description_get_family(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_gravity")]
    pub fn get_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_font_description_get_gravity(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_set_fields")]
    pub fn get_set_fields(&self) -> FontMask {
        unsafe {
            from_glib(ffi::pango_font_description_get_set_fields(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_size")]
    pub fn get_size(&self) -> i32 {
        unsafe { ffi::pango_font_description_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_description_get_size_is_absolute")]
    pub fn get_size_is_absolute(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_description_get_size_is_absolute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_stretch")]
    pub fn get_stretch(&self) -> Stretch {
        unsafe {
            from_glib(ffi::pango_font_description_get_stretch(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_style")]
    pub fn get_style(&self) -> Style {
        unsafe { from_glib(ffi::pango_font_description_get_style(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_font_description_get_variant")]
    pub fn get_variant(&self) -> Variant {
        unsafe {
            from_glib(ffi::pango_font_description_get_variant(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "pango_font_description_get_variations")]
    pub fn get_variations(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::pango_font_description_get_variations(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_get_weight")]
    pub fn get_weight(&self) -> Weight {
        unsafe {
            from_glib(ffi::pango_font_description_get_weight(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_hash")]
    fn hash(&self) -> u32 {
        unsafe { ffi::pango_font_description_hash(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_font_description_merge")]
    pub fn merge(&mut self, desc_to_merge: Option<&FontDescription>, replace_existing: bool) {
        unsafe {
            ffi::pango_font_description_merge(
                self.to_glib_none_mut().0,
                desc_to_merge.to_glib_none().0,
                replace_existing.to_glib(),
            );
        }
    }

    #[doc(alias = "pango_font_description_set_absolute_size")]
    pub fn set_absolute_size(&mut self, size: f64) {
        unsafe {
            ffi::pango_font_description_set_absolute_size(self.to_glib_none_mut().0, size);
        }
    }

    #[doc(alias = "pango_font_description_set_family")]
    pub fn set_family(&mut self, family: &str) {
        unsafe {
            ffi::pango_font_description_set_family(
                self.to_glib_none_mut().0,
                family.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "pango_font_description_set_gravity")]
    pub fn set_gravity(&mut self, gravity: Gravity) {
        unsafe {
            ffi::pango_font_description_set_gravity(self.to_glib_none_mut().0, gravity.to_glib());
        }
    }

    #[doc(alias = "pango_font_description_set_size")]
    pub fn set_size(&mut self, size: i32) {
        unsafe {
            ffi::pango_font_description_set_size(self.to_glib_none_mut().0, size);
        }
    }

    #[doc(alias = "pango_font_description_set_stretch")]
    pub fn set_stretch(&mut self, stretch: Stretch) {
        unsafe {
            ffi::pango_font_description_set_stretch(self.to_glib_none_mut().0, stretch.to_glib());
        }
    }

    #[doc(alias = "pango_font_description_set_style")]
    pub fn set_style(&mut self, style: Style) {
        unsafe {
            ffi::pango_font_description_set_style(self.to_glib_none_mut().0, style.to_glib());
        }
    }

    #[doc(alias = "pango_font_description_set_variant")]
    pub fn set_variant(&mut self, variant: Variant) {
        unsafe {
            ffi::pango_font_description_set_variant(self.to_glib_none_mut().0, variant.to_glib());
        }
    }

    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "pango_font_description_set_variations")]
    pub fn set_variations(&mut self, variations: &str) {
        unsafe {
            ffi::pango_font_description_set_variations(
                self.to_glib_none_mut().0,
                variations.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "pango_font_description_set_variations_static")]
    pub fn set_variations_static(&mut self, variations: &str) {
        unsafe {
            ffi::pango_font_description_set_variations_static(
                self.to_glib_none_mut().0,
                variations.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "pango_font_description_set_weight")]
    pub fn set_weight(&mut self, weight: Weight) {
        unsafe {
            ffi::pango_font_description_set_weight(self.to_glib_none_mut().0, weight.to_glib());
        }
    }

    #[doc(alias = "pango_font_description_to_filename")]
    pub fn to_filename(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::pango_font_description_to_filename(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_description_to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::pango_font_description_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_font_description_unset_fields")]
    pub fn unset_fields(&mut self, to_unset: FontMask) {
        unsafe {
            ffi::pango_font_description_unset_fields(self.to_glib_none_mut().0, to_unset.to_glib());
        }
    }

    #[doc(alias = "pango_font_description_from_string")]
    pub fn from_string(str: &str) -> FontDescription {
        unsafe {
            from_glib_full(ffi::pango_font_description_from_string(
                str.to_glib_none().0,
            ))
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
        f.write_str(&self.to_str())
    }
}

impl hash::Hash for FontDescription {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.hash(), state)
    }
}