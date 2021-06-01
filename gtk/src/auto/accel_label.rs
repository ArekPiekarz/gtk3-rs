// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::Justification;
use crate::Label;
use crate::Misc;
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
    #[doc(alias = "GtkAccelLabel")]
    pub struct AccelLabel(Object<ffi::GtkAccelLabel, ffi::GtkAccelLabelClass>) @extends Label, Misc, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_accel_label_get_type(),
    }
}

impl AccelLabel {
    #[doc(alias = "gtk_accel_label_new")]
    pub fn new(string: &str) -> AccelLabel {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_accel_label_new(string.to_glib_none().0)).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`AccelLabel`].
    ///
    /// This method returns an instance of [`AccelLabelBuilder`] which can be used to create a [`AccelLabel`].
    pub fn builder() -> AccelLabelBuilder {
        AccelLabelBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`AccelLabel`].
pub struct AccelLabelBuilder {
    accel_closure: Option<glib::Closure>,
    accel_widget: Option<Widget>,
    angle: Option<f64>,
    attributes: Option<pango::AttrList>,
    ellipsize: Option<pango::EllipsizeMode>,
    justify: Option<Justification>,
    label: Option<String>,
    lines: Option<i32>,
    max_width_chars: Option<i32>,
    mnemonic_widget: Option<Widget>,
    pattern: Option<String>,
    selectable: Option<bool>,
    single_line_mode: Option<bool>,
    track_visited_links: Option<bool>,
    use_markup: Option<bool>,
    use_underline: Option<bool>,
    width_chars: Option<i32>,
    wrap: Option<bool>,
    wrap_mode: Option<pango::WrapMode>,
    xalign: Option<f32>,
    yalign: Option<f32>,
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

impl AccelLabelBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AccelLabelBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AccelLabel`].
    pub fn build(self) -> AccelLabel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accel_closure) = self.accel_closure {
            properties.push(("accel-closure", accel_closure));
        }
        if let Some(ref accel_widget) = self.accel_widget {
            properties.push(("accel-widget", accel_widget));
        }
        if let Some(ref angle) = self.angle {
            properties.push(("angle", angle));
        }
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref ellipsize) = self.ellipsize {
            properties.push(("ellipsize", ellipsize));
        }
        if let Some(ref justify) = self.justify {
            properties.push(("justify", justify));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref lines) = self.lines {
            properties.push(("lines", lines));
        }
        if let Some(ref max_width_chars) = self.max_width_chars {
            properties.push(("max-width-chars", max_width_chars));
        }
        if let Some(ref mnemonic_widget) = self.mnemonic_widget {
            properties.push(("mnemonic-widget", mnemonic_widget));
        }
        if let Some(ref pattern) = self.pattern {
            properties.push(("pattern", pattern));
        }
        if let Some(ref selectable) = self.selectable {
            properties.push(("selectable", selectable));
        }
        if let Some(ref single_line_mode) = self.single_line_mode {
            properties.push(("single-line-mode", single_line_mode));
        }
        if let Some(ref track_visited_links) = self.track_visited_links {
            properties.push(("track-visited-links", track_visited_links));
        }
        if let Some(ref use_markup) = self.use_markup {
            properties.push(("use-markup", use_markup));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref width_chars) = self.width_chars {
            properties.push(("width-chars", width_chars));
        }
        if let Some(ref wrap) = self.wrap {
            properties.push(("wrap", wrap));
        }
        if let Some(ref wrap_mode) = self.wrap_mode {
            properties.push(("wrap-mode", wrap_mode));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
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
        glib::Object::new::<AccelLabel>(&properties)
            .expect("Failed to create an instance of AccelLabel")
    }

    pub fn accel_closure(mut self, accel_closure: &glib::Closure) -> Self {
        self.accel_closure = Some(accel_closure.clone());
        self
    }

    pub fn accel_widget<P: IsA<Widget>>(mut self, accel_widget: &P) -> Self {
        self.accel_widget = Some(accel_widget.clone().upcast());
        self
    }

    pub fn angle(mut self, angle: f64) -> Self {
        self.angle = Some(angle);
        self
    }

    pub fn attributes(mut self, attributes: &pango::AttrList) -> Self {
        self.attributes = Some(attributes.clone());
        self
    }

    pub fn ellipsize(mut self, ellipsize: pango::EllipsizeMode) -> Self {
        self.ellipsize = Some(ellipsize);
        self
    }

    pub fn justify(mut self, justify: Justification) -> Self {
        self.justify = Some(justify);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn lines(mut self, lines: i32) -> Self {
        self.lines = Some(lines);
        self
    }

    pub fn max_width_chars(mut self, max_width_chars: i32) -> Self {
        self.max_width_chars = Some(max_width_chars);
        self
    }

    pub fn mnemonic_widget<P: IsA<Widget>>(mut self, mnemonic_widget: &P) -> Self {
        self.mnemonic_widget = Some(mnemonic_widget.clone().upcast());
        self
    }

    pub fn pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    pub fn single_line_mode(mut self, single_line_mode: bool) -> Self {
        self.single_line_mode = Some(single_line_mode);
        self
    }

    pub fn track_visited_links(mut self, track_visited_links: bool) -> Self {
        self.track_visited_links = Some(track_visited_links);
        self
    }

    pub fn use_markup(mut self, use_markup: bool) -> Self {
        self.use_markup = Some(use_markup);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn width_chars(mut self, width_chars: i32) -> Self {
        self.width_chars = Some(width_chars);
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: pango::WrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
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

pub const NONE_ACCEL_LABEL: Option<&AccelLabel> = None;

pub trait AccelLabelExt: 'static {
    #[doc(alias = "gtk_accel_label_get_accel")]
    #[doc(alias = "get_accel")]
    fn accel(&self) -> (u32, gdk::ModifierType);

    #[doc(alias = "gtk_accel_label_get_accel_widget")]
    #[doc(alias = "get_accel_widget")]
    fn accel_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_accel_label_get_accel_width")]
    #[doc(alias = "get_accel_width")]
    fn accel_width(&self) -> u32;

    #[doc(alias = "gtk_accel_label_refetch")]
    fn refetch(&self) -> bool;

    #[doc(alias = "gtk_accel_label_set_accel")]
    fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType);

    #[doc(alias = "gtk_accel_label_set_accel_closure")]
    fn set_accel_closure(&self, accel_closure: Option<&glib::Closure>);

    #[doc(alias = "gtk_accel_label_set_accel_widget")]
    fn set_accel_widget<P: IsA<Widget>>(&self, accel_widget: Option<&P>);

    #[doc(alias = "accel-closure")]
    fn accel_closure(&self) -> Option<glib::Closure>;

    #[doc(alias = "accel-closure")]
    fn connect_accel_closure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "accel-widget")]
    fn connect_accel_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AccelLabel>> AccelLabelExt for O {
    fn accel(&self) -> (u32, gdk::ModifierType) {
        unsafe {
            let mut accelerator_key = mem::MaybeUninit::uninit();
            let mut accelerator_mods = mem::MaybeUninit::uninit();
            ffi::gtk_accel_label_get_accel(
                self.as_ref().to_glib_none().0,
                accelerator_key.as_mut_ptr(),
                accelerator_mods.as_mut_ptr(),
            );
            let accelerator_key = accelerator_key.assume_init();
            let accelerator_mods = accelerator_mods.assume_init();
            (accelerator_key, from_glib(accelerator_mods))
        }
    }

    fn accel_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_accel_label_get_accel_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn accel_width(&self) -> u32 {
        unsafe { ffi::gtk_accel_label_get_accel_width(self.as_ref().to_glib_none().0) }
    }

    fn refetch(&self) -> bool {
        unsafe { from_glib(ffi::gtk_accel_label_refetch(self.as_ref().to_glib_none().0)) }
    }

    fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType) {
        unsafe {
            ffi::gtk_accel_label_set_accel(
                self.as_ref().to_glib_none().0,
                accelerator_key,
                accelerator_mods.into_glib(),
            );
        }
    }

    fn set_accel_closure(&self, accel_closure: Option<&glib::Closure>) {
        unsafe {
            ffi::gtk_accel_label_set_accel_closure(
                self.as_ref().to_glib_none().0,
                accel_closure.to_glib_none().0,
            );
        }
    }

    fn set_accel_widget<P: IsA<Widget>>(&self, accel_widget: Option<&P>) {
        unsafe {
            ffi::gtk_accel_label_set_accel_widget(
                self.as_ref().to_glib_none().0,
                accel_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn accel_closure(&self) -> Option<glib::Closure> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::Closure as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"accel-closure\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accel-closure` getter")
        }
    }

    #[doc(alias = "accel-closure")]
    fn connect_accel_closure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_closure_trampoline<
            P: IsA<AccelLabel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAccelLabel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AccelLabel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accel-closure\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_closure_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-widget")]
    fn connect_accel_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_widget_trampoline<
            P: IsA<AccelLabel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAccelLabel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AccelLabel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accel-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AccelLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AccelLabel")
    }
}
