// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Orientation;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkOrientable")]
    pub struct Orientable(Interface<ffi::GtkOrientable, ffi::GtkOrientableIface>);

    match fn {
        type_ => || ffi::gtk_orientable_get_type(),
    }
}

pub const NONE_ORIENTABLE: Option<&Orientable> = None;

pub trait OrientableExt: 'static {
    #[doc(alias = "gtk_orientable_get_orientation")]
    #[doc(alias = "get_orientation")]
    fn orientation(&self) -> Orientation;

    #[doc(alias = "gtk_orientable_set_orientation")]
    fn set_orientation(&self, orientation: Orientation);

    #[doc(alias = "orientation")]
    fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Orientable>> OrientableExt for O {
    fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_orientable_get_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_orientable_set_orientation(
                self.as_ref().to_glib_none().0,
                orientation.into_glib(),
            );
        }
    }

    #[doc(alias = "orientation")]
    fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_orientation_trampoline<
            P: IsA<Orientable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkOrientable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Orientable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::orientation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_orientation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Orientable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Orientable")
    }
}
