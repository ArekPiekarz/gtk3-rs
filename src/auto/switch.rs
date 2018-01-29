// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Buildable;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Switch(Object<ffi::GtkSwitch, ffi::GtkSwitchClass>): Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_switch_get_type(),
    }
}

impl Switch {
    pub fn new() -> Switch {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_switch_new()).downcast_unchecked()
        }
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SwitchExt {
    fn get_active(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_state(&self) -> bool;

    fn set_active(&self, is_active: bool);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_state(&self, state: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_state_set<F: Fn(&Self, bool) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Switch> + IsA<glib::object::Object> + glib::object::ObjectExt> SwitchExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_switch_get_active(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_switch_get_state(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_switch_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_state(&self, state: bool) {
        unsafe {
            ffi::gtk_switch_set_state(self.to_glib_none().0, state.to_glib());
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate(&self) {
        let _ = self.emit("activate", &[]).unwrap();
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_state_set<F: Fn(&Self, bool) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "state-set",
                transmute(state_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::state",
                transmute(notify_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkSwitch, f: glib_ffi::gpointer)
where P: IsA<Switch> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Switch::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn state_set_trampoline<P>(this: *mut ffi::GtkSwitch, state: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Switch> {
    callback_guard!();
    let f: &&(Fn(&P, bool) -> Inhibit + 'static) = transmute(f);
    f(&Switch::from_glib_borrow(this).downcast_unchecked(), from_glib(state)).to_glib()
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkSwitch, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Switch> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Switch::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_state_trampoline<P>(this: *mut ffi::GtkSwitch, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Switch> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Switch::from_glib_borrow(this).downcast_unchecked())
}
