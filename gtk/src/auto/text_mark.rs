// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TextBuffer;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkTextMark")]
    pub struct TextMark(Object<ffi::GtkTextMark, ffi::GtkTextMarkClass>);

    match fn {
        type_ => || ffi::gtk_text_mark_get_type(),
    }
}

impl TextMark {
    #[doc(alias = "gtk_text_mark_new")]
    pub fn new(name: Option<&str>, left_gravity: bool) -> TextMark {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_mark_new(
                name.to_glib_none().0,
                left_gravity.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`TextMark`].
    ///
    /// This method returns an instance of [`TextMarkBuilder`] which can be used to create a [`TextMark`].
    pub fn builder() -> TextMarkBuilder {
        TextMarkBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`TextMark`].
pub struct TextMarkBuilder {
    left_gravity: Option<bool>,
    name: Option<String>,
}

impl TextMarkBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`TextMarkBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TextMark`].
    pub fn build(self) -> TextMark {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref left_gravity) = self.left_gravity {
            properties.push(("left-gravity", left_gravity));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        glib::Object::new::<TextMark>(&properties)
            .expect("Failed to create an instance of TextMark")
    }

    pub fn left_gravity(mut self, left_gravity: bool) -> Self {
        self.left_gravity = Some(left_gravity);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

pub const NONE_TEXT_MARK: Option<&TextMark> = None;

pub trait TextMarkExt: 'static {
    #[doc(alias = "gtk_text_mark_get_buffer")]
    #[doc(alias = "get_buffer")]
    fn buffer(&self) -> Option<TextBuffer>;

    #[doc(alias = "gtk_text_mark_get_deleted")]
    #[doc(alias = "get_deleted")]
    fn is_deleted(&self) -> bool;

    #[doc(alias = "gtk_text_mark_get_left_gravity")]
    #[doc(alias = "get_left_gravity")]
    fn is_left_gravity(&self) -> bool;

    #[doc(alias = "gtk_text_mark_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_text_mark_get_visible")]
    #[doc(alias = "get_visible")]
    fn is_visible(&self) -> bool;

    #[doc(alias = "gtk_text_mark_set_visible")]
    fn set_visible(&self, setting: bool);
}

impl<O: IsA<TextMark>> TextMarkExt for O {
    fn buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_deleted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_left_gravity(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_left_gravity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_text_mark_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_mark_set_visible(self.as_ref().to_glib_none().0, setting.into_glib());
        }
    }
}

impl fmt::Display for TextMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextMark")
    }
}
