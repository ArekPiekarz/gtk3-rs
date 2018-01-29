// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use EventController;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use EventSequenceState;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gdk;
use glib;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Gesture(Object<ffi::GtkGesture, ffi::GtkGestureClass>): EventController;

    match fn {
        get_type => || ffi::gtk_gesture_get_type(),
    }
}

pub trait GestureExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_bounding_box(&self) -> Option<gdk::Rectangle>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_bounding_box_center(&self) -> Option<(f64, f64)>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_device(&self) -> Option<gdk::Device>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_group(&self) -> Vec<Gesture>;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_last_event(&self, sequence: /*Ignored*/&gdk::EventSequence) -> Option<gdk::Event>;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_last_updated_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence>;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_point<'a, P: Into<Option<&'a /*Ignored*/gdk::EventSequence>>>(&self, sequence: P) -> Option<(f64, f64)>;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence) -> EventSequenceState;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_sequences(&self) -> /*Ignored*/Vec<gdk::EventSequence>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_window(&self) -> Option<gdk::Window>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn group<P: IsA<Gesture>>(&self, gesture: &P);

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn handles_sequence<'a, P: Into<Option<&'a /*Ignored*/gdk::EventSequence>>>(&self, sequence: P) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_active(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_grouped_with<P: IsA<Gesture>>(&self, other: &P) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_recognized(&self) -> bool;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn set_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence, state: EventSequenceState) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_state(&self, state: EventSequenceState) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_window<'a, P: Into<Option<&'a gdk::Window>>>(&self, window: P);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn ungroup(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_property_n_points(&self) -> u32;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_begin<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_cancel<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_end<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_sequence_state_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_update<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_n_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Gesture> + IsA<glib::object::Object>> GestureExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_bounding_box(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_bounding_box_center(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box_center(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_device(&self) -> Option<gdk::Device> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_device(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_group(&self) -> Vec<Gesture> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_gesture_get_group(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_last_event(&self, sequence: /*Ignored*/&gdk::EventSequence) -> Option<gdk::Event> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_last_event() }
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_last_updated_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_last_updated_sequence() }
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_point<'a, P: Into<Option<&'a /*Ignored*/gdk::EventSequence>>>(&self, sequence: P) -> Option<(f64, f64)> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_point() }
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence) -> EventSequenceState {
    //    unsafe { TODO: call ffi::gtk_gesture_get_sequence_state() }
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_sequences(&self) -> /*Ignored*/Vec<gdk::EventSequence> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_sequences() }
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn group<P: IsA<Gesture>>(&self, gesture: &P) {
        unsafe {
            ffi::gtk_gesture_group(self.to_glib_none().0, gesture.to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn handles_sequence<'a, P: Into<Option<&'a /*Ignored*/gdk::EventSequence>>>(&self, sequence: P) -> bool {
    //    unsafe { TODO: call ffi::gtk_gesture_handles_sequence() }
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_active(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_grouped_with<P: IsA<Gesture>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_grouped_with(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_recognized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_recognized(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn set_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence, state: EventSequenceState) -> bool {
    //    unsafe { TODO: call ffi::gtk_gesture_set_sequence_state() }
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_state(&self, state: EventSequenceState) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_set_state(self.to_glib_none().0, state.to_glib()))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_window<'a, P: Into<Option<&'a gdk::Window>>>(&self, window: P) {
        let window = window.into();
        let window = window.to_glib_none();
        unsafe {
            ffi::gtk_gesture_set_window(self.to_glib_none().0, window.0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn ungroup(&self) {
        unsafe {
            ffi::gtk_gesture_ungroup(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_property_n_points(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "n-points".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_begin<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_cancel<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_end<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_sequence_state_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn connect_update<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored sequence: Gdk.EventSequence
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_n_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::n-points",
                transmute(notify_n_points_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window",
                transmute(notify_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_n_points_trampoline<P>(this: *mut ffi::GtkGesture, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Gesture> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Gesture::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_window_trampoline<P>(this: *mut ffi::GtkGesture, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Gesture> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Gesture::from_glib_borrow(this).downcast_unchecked())
}
