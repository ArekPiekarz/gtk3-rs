// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EulerOrder;
use crate::Matrix;
use crate::Quaternion;
use crate::Vec3;
use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Euler(Boxed<ffi::graphene_euler_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_euler_get_type(), ptr as *mut _) as *mut ffi::graphene_euler_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_euler_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ffi::graphene_euler_get_type(),
    }
}

impl Euler {
    #[doc(alias = "graphene_euler_equal")]
    fn equal(&self, b: &Euler) -> bool {
        unsafe {
            from_glib(ffi::graphene_euler_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_euler_get_alpha")]
    pub fn get_alpha(&self) -> f32 {
        unsafe { ffi::graphene_euler_get_alpha(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_euler_get_beta")]
    pub fn get_beta(&self) -> f32 {
        unsafe { ffi::graphene_euler_get_beta(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_euler_get_gamma")]
    pub fn get_gamma(&self) -> f32 {
        unsafe { ffi::graphene_euler_get_gamma(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_euler_get_order")]
    pub fn get_order(&self) -> EulerOrder {
        unsafe { from_glib(ffi::graphene_euler_get_order(self.to_glib_none().0)) }
    }

    #[doc(alias = "graphene_euler_get_x")]
    pub fn get_x(&self) -> f32 {
        unsafe { ffi::graphene_euler_get_x(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_euler_get_y")]
    pub fn get_y(&self) -> f32 {
        unsafe { ffi::graphene_euler_get_y(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_euler_get_z")]
    pub fn get_z(&self) -> f32 {
        unsafe { ffi::graphene_euler_get_z(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_euler_init")]
    pub fn init(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::graphene_euler_init(self.to_glib_none_mut().0, x, y, z);
        }
    }

    #[doc(alias = "graphene_euler_init_from_euler")]
    pub fn init_from_euler(&mut self, src: Option<&Euler>) {
        unsafe {
            ffi::graphene_euler_init_from_euler(self.to_glib_none_mut().0, src.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_euler_init_from_matrix")]
    pub fn init_from_matrix(&mut self, m: Option<&Matrix>, order: EulerOrder) {
        unsafe {
            ffi::graphene_euler_init_from_matrix(
                self.to_glib_none_mut().0,
                m.to_glib_none().0,
                order.to_glib(),
            );
        }
    }

    #[doc(alias = "graphene_euler_init_from_quaternion")]
    pub fn init_from_quaternion(&mut self, q: Option<&Quaternion>, order: EulerOrder) {
        unsafe {
            ffi::graphene_euler_init_from_quaternion(
                self.to_glib_none_mut().0,
                q.to_glib_none().0,
                order.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_euler_init_from_radians")]
    pub fn init_from_radians(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        order: EulerOrder,
    ) -> Option<Euler> {
        unsafe {
            from_glib_none(ffi::graphene_euler_init_from_radians(
                self.to_glib_none_mut().0,
                x,
                y,
                z,
                order.to_glib(),
            ))
        }
    }

    #[doc(alias = "graphene_euler_init_from_vec3")]
    pub fn init_from_vec3(&mut self, v: Option<&Vec3>, order: EulerOrder) {
        unsafe {
            ffi::graphene_euler_init_from_vec3(
                self.to_glib_none_mut().0,
                v.to_glib_none().0,
                order.to_glib(),
            );
        }
    }

    #[doc(alias = "graphene_euler_init_with_order")]
    pub fn init_with_order(&mut self, x: f32, y: f32, z: f32, order: EulerOrder) {
        unsafe {
            ffi::graphene_euler_init_with_order(
                self.to_glib_none_mut().0,
                x,
                y,
                z,
                order.to_glib(),
            );
        }
    }

    #[doc(alias = "graphene_euler_reorder")]
    pub fn reorder(&self, order: EulerOrder) -> Euler {
        unsafe {
            let mut res = Euler::uninitialized();
            ffi::graphene_euler_reorder(
                self.to_glib_none().0,
                order.to_glib(),
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_euler_to_matrix")]
    pub fn to_matrix(&self) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_euler_to_matrix(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_euler_to_quaternion")]
    pub fn to_quaternion(&self) -> Quaternion {
        unsafe {
            let mut res = Quaternion::uninitialized();
            ffi::graphene_euler_to_quaternion(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_euler_to_vec3")]
    pub fn to_vec3(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            ffi::graphene_euler_to_vec3(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }
}

impl PartialEq for Euler {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Euler {}