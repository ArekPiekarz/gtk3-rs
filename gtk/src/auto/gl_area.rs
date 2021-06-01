// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkGLArea")]
    pub struct GLArea(Object<ffi::GtkGLArea, ffi::GtkGLAreaClass>) @extends Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_gl_area_get_type(),
    }
}

impl GLArea {
    #[doc(alias = "gtk_gl_area_new")]
    pub fn new() -> GLArea {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_gl_area_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`GLArea`].
    ///
    /// This method returns an instance of [`GLAreaBuilder`] which can be used to create a [`GLArea`].
    pub fn builder() -> GLAreaBuilder {
        GLAreaBuilder::default()
    }
}

impl Default for GLArea {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`GLArea`].
pub struct GLAreaBuilder {
    auto_render: Option<bool>,
    has_alpha: Option<bool>,
    has_depth_buffer: Option<bool>,
    has_stencil_buffer: Option<bool>,
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    use_es: Option<bool>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl GLAreaBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GLAreaBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GLArea`].
    pub fn build(self) -> GLArea {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref auto_render) = self.auto_render {
            properties.push(("auto-render", auto_render));
        }
        if let Some(ref has_alpha) = self.has_alpha {
            properties.push(("has-alpha", has_alpha));
        }
        if let Some(ref has_depth_buffer) = self.has_depth_buffer {
            properties.push(("has-depth-buffer", has_depth_buffer));
        }
        if let Some(ref has_stencil_buffer) = self.has_stencil_buffer {
            properties.push(("has-stencil-buffer", has_stencil_buffer));
        }
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        if let Some(ref use_es) = self.use_es {
            properties.push(("use-es", use_es));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new::<GLArea>(&properties).expect("Failed to create an instance of GLArea")
    }

    pub fn auto_render(mut self, auto_render: bool) -> Self {
        self.auto_render = Some(auto_render);
        self
    }

    pub fn has_alpha(mut self, has_alpha: bool) -> Self {
        self.has_alpha = Some(has_alpha);
        self
    }

    pub fn has_depth_buffer(mut self, has_depth_buffer: bool) -> Self {
        self.has_depth_buffer = Some(has_depth_buffer);
        self
    }

    pub fn has_stencil_buffer(mut self, has_stencil_buffer: bool) -> Self {
        self.has_stencil_buffer = Some(has_stencil_buffer);
        self
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn use_es(mut self, use_es: bool) -> Self {
        self.use_es = Some(use_es);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_GL_AREA: Option<&GLArea> = None;

pub trait GLAreaExt: 'static {
    #[doc(alias = "gtk_gl_area_attach_buffers")]
    fn attach_buffers(&self);

    #[doc(alias = "gtk_gl_area_get_auto_render")]
    #[doc(alias = "get_auto_render")]
    fn is_auto_render(&self) -> bool;

    #[doc(alias = "gtk_gl_area_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> Option<gdk::GLContext>;

    #[doc(alias = "gtk_gl_area_get_error")]
    #[doc(alias = "get_error")]
    fn error(&self) -> Option<glib::Error>;

    #[doc(alias = "gtk_gl_area_get_has_alpha")]
    #[doc(alias = "get_has_alpha")]
    fn has_alpha(&self) -> bool;

    #[doc(alias = "gtk_gl_area_get_has_depth_buffer")]
    #[doc(alias = "get_has_depth_buffer")]
    fn has_depth_buffer(&self) -> bool;

    #[doc(alias = "gtk_gl_area_get_has_stencil_buffer")]
    #[doc(alias = "get_has_stencil_buffer")]
    fn has_stencil_buffer(&self) -> bool;

    #[doc(alias = "gtk_gl_area_get_required_version")]
    #[doc(alias = "get_required_version")]
    fn required_version(&self) -> (i32, i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gtk_gl_area_get_use_es")]
    #[doc(alias = "get_use_es")]
    fn uses_es(&self) -> bool;

    #[doc(alias = "gtk_gl_area_make_current")]
    fn make_current(&self);

    #[doc(alias = "gtk_gl_area_queue_render")]
    fn queue_render(&self);

    #[doc(alias = "gtk_gl_area_set_auto_render")]
    fn set_auto_render(&self, auto_render: bool);

    #[doc(alias = "gtk_gl_area_set_error")]
    fn set_error(&self, error: Option<&glib::Error>);

    #[doc(alias = "gtk_gl_area_set_has_alpha")]
    fn set_has_alpha(&self, has_alpha: bool);

    #[doc(alias = "gtk_gl_area_set_has_depth_buffer")]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool);

    #[doc(alias = "gtk_gl_area_set_has_stencil_buffer")]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool);

    #[doc(alias = "gtk_gl_area_set_required_version")]
    fn set_required_version(&self, major: i32, minor: i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gtk_gl_area_set_use_es")]
    fn set_use_es(&self, use_es: bool);

    #[doc(alias = "create-context")]
    fn connect_create_context<F: Fn(&Self) -> Option<gdk::GLContext> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "render")]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "resize")]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "auto-render")]
    fn connect_auto_render_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "context")]
    fn connect_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "has-alpha")]
    fn connect_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "has-depth-buffer")]
    fn connect_has_depth_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "has-stencil-buffer")]
    fn connect_has_stencil_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "use-es")]
    fn connect_use_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GLArea>> GLAreaExt for O {
    fn attach_buffers(&self) {
        unsafe {
            ffi::gtk_gl_area_attach_buffers(self.as_ref().to_glib_none().0);
        }
    }

    fn is_auto_render(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_auto_render(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context(&self) -> Option<gdk::GLContext> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_context(self.as_ref().to_glib_none().0)) }
    }

    fn error(&self) -> Option<glib::Error> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_error(self.as_ref().to_glib_none().0)) }
    }

    fn has_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_alpha(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_depth_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_depth_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_stencil_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_stencil_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gtk_gl_area_get_required_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            let major = major.assume_init();
            let minor = minor.assume_init();
            (major, minor)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    fn uses_es(&self) -> bool {
        unsafe { from_glib(ffi::gtk_gl_area_get_use_es(self.as_ref().to_glib_none().0)) }
    }

    fn make_current(&self) {
        unsafe {
            ffi::gtk_gl_area_make_current(self.as_ref().to_glib_none().0);
        }
    }

    fn queue_render(&self) {
        unsafe {
            ffi::gtk_gl_area_queue_render(self.as_ref().to_glib_none().0);
        }
    }

    fn set_auto_render(&self, auto_render: bool) {
        unsafe {
            ffi::gtk_gl_area_set_auto_render(
                self.as_ref().to_glib_none().0,
                auto_render.into_glib(),
            );
        }
    }

    fn set_error(&self, error: Option<&glib::Error>) {
        unsafe {
            ffi::gtk_gl_area_set_error(self.as_ref().to_glib_none().0, error.to_glib_none().0);
        }
    }

    fn set_has_alpha(&self, has_alpha: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_alpha(self.as_ref().to_glib_none().0, has_alpha.into_glib());
        }
    }

    fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_depth_buffer(
                self.as_ref().to_glib_none().0,
                has_depth_buffer.into_glib(),
            );
        }
    }

    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_stencil_buffer(
                self.as_ref().to_glib_none().0,
                has_stencil_buffer.into_glib(),
            );
        }
    }

    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gtk_gl_area_set_required_version(self.as_ref().to_glib_none().0, major, minor);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    fn set_use_es(&self, use_es: bool) {
        unsafe {
            ffi::gtk_gl_area_set_use_es(self.as_ref().to_glib_none().0, use_es.into_glib());
        }
    }

    #[doc(alias = "create-context")]
    fn connect_create_context<F: Fn(&Self) -> Option<gdk::GLContext> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_context_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) -> Option<gdk::GLContext> + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            f: glib::ffi::gpointer,
        ) -> *mut gdk::ffi::GdkGLContext {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref()).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "render")]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P, &gdk::GLContext) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            context: *mut gdk::ffi::GdkGLContext,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &GLArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"render\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize")]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resize_trampoline<P: IsA<GLArea>, F: Fn(&P, i32, i32) + 'static>(
            this: *mut ffi::GtkGLArea,
            width: libc::c_int,
            height: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &GLArea::from_glib_borrow(this).unsafe_cast_ref(),
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "auto-render")]
    fn connect_auto_render_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_render_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-render\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "context")]
    fn connect_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_context_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-alpha")]
    fn connect_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_alpha_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_alpha_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-depth-buffer")]
    fn connect_has_depth_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_depth_buffer_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-depth-buffer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_depth_buffer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-stencil-buffer")]
    fn connect_has_stencil_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_stencil_buffer_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-stencil-buffer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_stencil_buffer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "use-es")]
    fn connect_use_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_es_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-es\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_es_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GLArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLArea")
    }
}
