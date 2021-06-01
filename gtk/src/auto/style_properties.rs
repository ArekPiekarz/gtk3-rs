// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StyleProvider;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkStyleProperties")]
    pub struct StyleProperties(Object<ffi::GtkStyleProperties, ffi::GtkStylePropertiesClass>) @implements StyleProvider;

    match fn {
        type_ => || ffi::gtk_style_properties_get_type(),
    }
}

impl StyleProperties {}

pub const NONE_STYLE_PROPERTIES: Option<&StyleProperties> = None;

impl fmt::Display for StyleProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleProperties")
    }
}
