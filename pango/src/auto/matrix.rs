// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
#[cfg(any(feature = "v1_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_38")))]
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Matrix(Boxed<ffi::PangoMatrix>);

    match fn {
        copy => |ptr| ffi::pango_matrix_copy(ptr),
        free => |ptr| ffi::pango_matrix_free(ptr),
        get_type => || ffi::pango_matrix_get_type(),
    }
}

impl Matrix {
    #[doc(alias = "pango_matrix_concat")]
    pub fn concat(&mut self, new_matrix: &Matrix) {
        unsafe {
            ffi::pango_matrix_concat(self.to_glib_none_mut().0, new_matrix.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_matrix_get_font_scale_factor")]
    pub fn get_font_scale_factor(&self) -> f64 {
        unsafe { ffi::pango_matrix_get_font_scale_factor(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_38")))]
    #[doc(alias = "pango_matrix_get_font_scale_factors")]
    pub fn get_font_scale_factors(&self) -> (f64, f64) {
        unsafe {
            let mut xscale = mem::MaybeUninit::uninit();
            let mut yscale = mem::MaybeUninit::uninit();
            ffi::pango_matrix_get_font_scale_factors(
                self.to_glib_none().0,
                xscale.as_mut_ptr(),
                yscale.as_mut_ptr(),
            );
            let xscale = xscale.assume_init();
            let yscale = yscale.assume_init();
            (xscale, yscale)
        }
    }

    #[doc(alias = "pango_matrix_rotate")]
    pub fn rotate(&mut self, degrees: f64) {
        unsafe {
            ffi::pango_matrix_rotate(self.to_glib_none_mut().0, degrees);
        }
    }

    #[doc(alias = "pango_matrix_scale")]
    pub fn scale(&mut self, scale_x: f64, scale_y: f64) {
        unsafe {
            ffi::pango_matrix_scale(self.to_glib_none_mut().0, scale_x, scale_y);
        }
    }

    #[doc(alias = "pango_matrix_transform_distance")]
    pub fn transform_distance(&self, dx: &mut f64, dy: &mut f64) {
        unsafe {
            ffi::pango_matrix_transform_distance(self.to_glib_none().0, dx, dy);
        }
    }

    //#[doc(alias = "pango_matrix_transform_pixel_rectangle")]
    //pub fn transform_pixel_rectangle(&self, rect: /*Unimplemented*/Option<Rectangle>) {
    //    unsafe { TODO: call ffi:pango_matrix_transform_pixel_rectangle() }
    //}

    #[doc(alias = "pango_matrix_transform_point")]
    pub fn transform_point(&self, x: &mut f64, y: &mut f64) {
        unsafe {
            ffi::pango_matrix_transform_point(self.to_glib_none().0, x, y);
        }
    }

    //#[doc(alias = "pango_matrix_transform_rectangle")]
    //pub fn transform_rectangle(&self, rect: /*Unimplemented*/Option<Rectangle>) {
    //    unsafe { TODO: call ffi:pango_matrix_transform_rectangle() }
    //}

    #[doc(alias = "pango_matrix_translate")]
    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            ffi::pango_matrix_translate(self.to_glib_none_mut().0, tx, ty);
        }
    }
}