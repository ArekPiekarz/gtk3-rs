initSidebarItems({"attr":[["gflags","Attribute macro for defining flags using the `bitflags` crate. This macro will also define a `GFlags::type_` function and the `glib::Value` traits."],["object_interface","Macro for boilerplate of `ObjectInterface` implementations."],["object_subclass","Macro for boilerplate of `ObjectSubclass` implementations."]],"constant":[["CLONE_MACRO_LOG_DOMAIN","This is the log domain used by the [`clone!`][crate::clone!] macro. If you want to use a custom logger (it prints to stdout by default), you can set your own logger using the corresponding `log` functions."]],"derive":[["Downgrade","Macro for deriving implementations of `glib::clone::Downgrade` and `glib::clone::Upgrade` traits and a weak type."],["GBoxed","Derive macro for defining a `BoxedType``::type_` function and the `glib::Value` traits."],["GEnum",""],["GErrorDomain","Derive macro for defining a GLib error domain and its associated `ErrorDomain` trait."],["GSharedBoxed","Derive macro for defining a `SharedType``::get_type` function and the `glib::Value` traits."]],"enum":[["ChecksumType",""],["DateMonth",""],["DateWeekday",""],["FileError",""],["GlibLoggerDomain","Enumeration of the possible domain handling behaviours for a `GlibLogger`."],["GlibLoggerFormat","Enumeration of the possible formatting behaviours for a `GlibLogger`."],["KeyFileError",""],["LogLevel",""],["OptionArg",""],["SeekType",""],["TimeType","Disambiguates a given time in two ways."],["UriError","Error codes returned by `Uri` methods."],["UserDirectory","These are logical ids for special directories which are defined depending on the platform used. You should use `g_get_user_special_dir()` to retrieve the full path associated to the logical id."]],"fn":[["access",""],["application_name",""],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["charset","Obtain the character set for the current locale."],["chdir",""],["check_version",""],["child_watch_future","Create a `Future` that will resolve once the child process with the given pid exits"],["child_watch_future_with_priority","Create a `Future` that will resolve once the child process with the given pid exits"],["clear_error",""],["codeset",""],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_checksum_for_string",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["compute_hmac_for_string",""],["console_charset",""],["current_dir",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ",""],["environ_getenv",""],["file_get_contents",""],["file_open_tmp",""],["file_read_link",""],["file_set_contents",""],["file_set_contents_full",""],["file_test",""],["filename_display_basename",""],["filename_display_name",""],["filename_from_uri",""],["filename_to_uri",""],["find_program_in_path",""],["format_size",""],["format_size_full",""],["getenv",""],["home_dir",""],["host_name",""],["hostname_is_ascii_encoded",""],["hostname_is_ip_address",""],["hostname_is_non_ascii",""],["hostname_to_ascii",""],["hostname_to_unicode",""],["interval_stream","Create a `Stream` that will provide a value every given number of milliseconds."],["interval_stream_seconds","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_seconds_with_priority","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_with_priority","Create a `Stream` that will provide a value every given number of milliseconds."],["is_canonical_pspec_name",""],["language_names",""],["language_names_with_category",""],["listenv",""],["locale_variants",""],["log_default_handler",""],["log_remove_handler",""],["log_set_always_fatal",""],["log_set_default_handler","To set back the default print handler, use the [`log_unset_default_handler`] function."],["log_set_fatal_mask",""],["log_set_handler",""],["log_unset_default_handler","To set the default print handler, use the [`log_set_default_handler`] function."],["main_current_source",""],["main_depth",""],["markup_escape_text",""],["mem_is_system_malloc",""],["mem_profile",""],["mkdir_with_parents",""],["mkdtemp",""],["mkdtemp_full",""],["mkstemp",""],["mkstemp_full",""],["monotonic_time",""],["num_processors",""],["on_error_query",""],["on_error_stack_trace",""],["os_info",""],["path_get_basename",""],["path_get_dirname",""],["path_is_absolute",""],["path_skip_root",""],["pattern_match_simple",""],["prgname",""],["program_name","Same as `get_prgname()`."],["random_double",""],["random_double_range",""],["random_int",""],["random_int_range",""],["random_set_seed",""],["real_name",""],["real_time",""],["reload_user_special_dirs_cache",""],["return_if_fail_warning",""],["rmdir",""],["rust_log_handler","Provides a glib log handler which routes all logging messages to the `log crate`."],["set_application_name",""],["set_prgname",""],["set_print_handler","To set back the default print handler, use the [`unset_print_handler`] function."],["set_printerr_handler","To set back the default print handler, use the [`unset_printerr_handler`] function."],["set_program_name","Same as `set_prgname()`."],["setenv",""],["shell_parse_argv",""],["shell_quote",""],["shell_unquote",""],["spaced_primes_closest",""],["spawn_async",""],["spawn_async_with_fds",""],["spawn_async_with_pipes",""],["spawn_check_exit_status",""],["spawn_command_line_async",""],["stpcpy",""],["system_config_dirs",""],["system_data_dirs",""],["timeout_future","Create a `Future` that will resolve after the given number of milliseconds."],["timeout_future_seconds","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_seconds_with_priority","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_with_priority","Create a `Future` that will resolve after the given number of milliseconds."],["tmp_dir",""],["unix_open_pipe",""],["unix_signal_future","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_future_with_priority","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_stream","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unix_signal_stream_with_priority","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unlink",""],["unset_print_handler","To set the default print handler, use the [`set_print_handler`] function."],["unset_printerr_handler","To set the default print handler, use the [`set_printerr_handler`] function."],["unsetenv",""],["user_cache_dir",""],["user_config_dir",""],["user_data_dir",""],["user_name",""],["user_runtime_dir",""],["user_special_dir",""],["usleep",""],["uuid_string_is_valid",""],["uuid_string_random",""],["warn_message",""]],"macro":[["bool_error","Generic error used for functions that fail without any further information"],["clone","Macro for passing variables as strong or weak references into a closure."],["debug","A macro which behaves exactly as `log::debug!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["error","A macro which behaves exactly as `log::error!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["g_critical","Macro used to log using GLib logging system. It uses g_log."],["g_debug","Macro used to log using GLib logging system. It uses g_log."],["g_error","Macro used to log using GLib logging system. It uses g_log."],["g_info","Macro used to log using GLib logging system. It uses g_log."],["g_log","Macro used to log using GLib logging system. It uses g_log."],["g_message","Macro used to log using GLib logging system. It uses g_log."],["g_print","Macro used to print messages. It uses g_print."],["g_printerr","Macro used to print error messages. It uses g_printerr."],["g_warning","Macro used to log using GLib logging system. It uses g_log."],["glib_boxed_wrapper","Wrapper implementations for Boxed types. See `wrapper!`."],["glib_object_wrapper","ObjectType implementations for Object types. See `wrapper!`."],["glib_shared_wrapper","Wrapper implementations for shared types. See `wrapper!`."],["info","A macro which behaves exactly as `log::info!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["result_from_gboolean",""],["trace","A macro which behaves exactly as `log::trace!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["warn","A macro which behaves exactly as `log::warn!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["wrapper","Defines a wrapper type and implements the appropriate traits."]],"mod":[["boxed","`IMPL` Boxed wrapper implementation."],["char",""],["clone",""],["closure",""],["error","`Error` binding and helper trait."],["functions",""],["object","`IMPL` Object wrapper implementation and `Object` binding."],["prelude","Traits and essential types intended for blanket imports."],["send_unique",""],["shared","`IMPL` Shared (reference counted) wrapper implementation."],["signal","`IMPL` Low level signal support."],["source",""],["subclass","Module containing infrastructure for subclassing `GObject`s and registering boxed types."],["translate","Translation between GLib/GLib-based FFI types and their Rust counterparts."],["types","Runtime type information."],["value","`Value` binding and helper traits."],["variant","`Variant` binding and helper traits."],["wrapper","`IMPL` The `wrapper!` macro and miscellaneous wrapper traits."]],"static":[["CSET_A_2_Z",""],["CSET_DIGITS",""],["CSET_a_2_z",""],["KEY_FILE_DESKTOP_GROUP",""],["KEY_FILE_DESKTOP_KEY_ACTIONS",""],["KEY_FILE_DESKTOP_KEY_CATEGORIES",""],["KEY_FILE_DESKTOP_KEY_COMMENT",""],["KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE",""],["KEY_FILE_DESKTOP_KEY_EXEC",""],["KEY_FILE_DESKTOP_KEY_GENERIC_NAME",""],["KEY_FILE_DESKTOP_KEY_HIDDEN",""],["KEY_FILE_DESKTOP_KEY_ICON",""],["KEY_FILE_DESKTOP_KEY_MIME_TYPE",""],["KEY_FILE_DESKTOP_KEY_NAME",""],["KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN",""],["KEY_FILE_DESKTOP_KEY_NO_DISPLAY",""],["KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN",""],["KEY_FILE_DESKTOP_KEY_PATH",""],["KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY",""],["KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS",""],["KEY_FILE_DESKTOP_KEY_TERMINAL",""],["KEY_FILE_DESKTOP_KEY_TRY_EXEC",""],["KEY_FILE_DESKTOP_KEY_TYPE",""],["KEY_FILE_DESKTOP_KEY_URL",""],["KEY_FILE_DESKTOP_KEY_VERSION",""],["KEY_FILE_DESKTOP_TYPE_APPLICATION",""],["KEY_FILE_DESKTOP_TYPE_DIRECTORY",""],["KEY_FILE_DESKTOP_TYPE_LINK",""],["OPTION_REMAINING",""],["STR_DELIMITERS",""],["TEST_OPTION_ISOLATE_DIRS",""],["URI_RESERVED_CHARS_GENERIC_DELIMITERS",""],["URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS",""],["g_param_spec_types",""]],"struct":[["Array",""],["Binding","`Binding` is the representation of a binding between a property on a `Object` instance (or source) and another property on another `Object` instance (or target). Whenever the source property changes, the same value is applied to the target property; for instance, the following binding:"],["BindingFlags","Flags to be passed to `ObjectExt::bind_property()` or `ObjectExt::bind_property_full()`."],["ByteArray",""],["Bytes","A shared immutable byte slice (the equivalent of `Rc<[u8]>`)."],["Checksum",""],["Date",""],["DateTime",""],["EnumClass","Representation of an `enum` for dynamically, at runtime, querying the values of the enum and using them."],["EnumValue","Representation of a single enum value of an `EnumClass`."],["FileSetContentsFlags",""],["FileTest",""],["FlagsBuilder","Builder for conveniently setting/unsetting flags and returning a `Value`."],["FlagsClass","Representation of a `flags` for dynamically, at runtime, querying the values of the enum and using them"],["FlagsValue","Representation of a single flags value of a `FlagsClass`."],["FormatSizeFlags",""],["GString",""],["GlibLogger","An implementation of a `log` compatible logger which logs over glib logging facilities."],["IOCondition",""],["KeyFile",""],["KeyFileFlags",""],["LogHandlerId",""],["LogLevelFlags",""],["LogLevels",""],["MainContext",""],["MainLoop",""],["OptionFlags",""],["ParamFlags","Through the `ParamFlags` flag values, certain aspects of parameters can be configured. See also `G_PARAM_STATIC_STRINGS`. Through the `ParamFlags` flag values, certain aspects of parameters can be configured. See also `G_PARAM_STATIC_STRINGS`."],["ParamSpec",""],["ParamSpecBoolean",""],["ParamSpecBoxed",""],["ParamSpecChar",""],["ParamSpecDouble",""],["ParamSpecEnum",""],["ParamSpecFlags",""],["ParamSpecFloat",""],["ParamSpecGType",""],["ParamSpecInt",""],["ParamSpecInt64",""],["ParamSpecLong",""],["ParamSpecObject",""],["ParamSpecOverride",""],["ParamSpecParam",""],["ParamSpecPointer",""],["ParamSpecString",""],["ParamSpecUChar",""],["ParamSpecUInt",""],["ParamSpecUInt64",""],["ParamSpecULong",""],["ParamSpecUnichar",""],["ParamSpecValueArray",""],["ParamSpecVariant",""],["Quark",""],["Receiver","A `Receiver` that can be attached to a main context to receive items from its corresponding `Sender` or `SyncSender`."],["Sender","A `Sender` that can be used to send items to the corresponding main context receiver."],["SignalFlags","The signal flags are used to specify a signal’s behaviour, the overall signal description outlines how especially the RUN flags control the stages of a signal emission. The signal flags are used to specify a signal’s behaviour, the overall signal description outlines how especially the RUN flags control the stages of a signal emission."],["Source",""],["SourceFuture","Represents a `Future` around a `glib::Source`. The future will be resolved once the source has provided a value"],["SourceStream","Represents a `Stream` around a `glib::Source`. The stream will be provide all values that are provided by the source"],["SpawnFlags","Flags passed to `g_spawn_sync()`, `g_spawn_async()` and `g_spawn_async_with_pipes()`. Flags passed to `g_spawn_sync()`, `g_spawn_async()` and `g_spawn_async_with_pipes()`."],["String","A mutable text buffer that grows automatically."],["SyncSender","A `SyncSender` that can be used to send items to the corresponding main context receiver."],["ThreadPool",""],["TimeZone","`TimeZone` is an opaque structure whose members cannot be accessed directly. `TimeZone` is an opaque structure whose members cannot be accessed directly."],["Uri","The `Uri` type and related functions can be used to parse URIs into their components, and build valid URIs from individual components."],["UriFlags","Flags that describe a URI."],["UriHideFlags","Flags describing what parts of the URI to hide in `Uri::to_string_partial()`. Note that `Password` and `AuthParams` will only work if the `Uri` was parsed with the corresponding flags. Flags describing what parts of the URI to hide in `Uri::to_string_partial()`. Note that `Password` and `AuthParams` will only work if the `Uri` was parsed with the corresponding flags."],["UriParamsFlags","Flags modifying the way parameters are handled by `Uri::parse_params()` and `UriParamsIter`. Flags modifying the way parameters are handled by `Uri::parse_params()` and `UriParamsIter`."],["ValueArray",""],["VariantDict","`VariantDict` is a mutable key/value store where the keys are always strings and the values are `Variant`s."],["VariantIter","Iterator over items in a variant."],["VariantTy","Describes `Variant` types."],["VariantType","Describes `Variant` types."]],"trait":[["ParamSpecType",""]],"type":[["DateDay",""],["DateYear",""],["Time",""],["TimeSpan",""]]});