// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SeparatorMenuItem(Object<ffi::GtkSeparatorMenuItem, ffi::GtkSeparatorMenuItemClass>): MenuItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_separator_menu_item_get_type(),
    }
}

impl SeparatorMenuItem {
    pub fn new() -> SeparatorMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_separator_menu_item_new()).downcast_unchecked()
        }
    }
}

impl Default for SeparatorMenuItem {
    fn default() -> Self {
        Self::new()
    }
}
