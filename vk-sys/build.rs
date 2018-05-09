extern crate vk_generator;
extern crate vk_api;

use std::path::Path;
use std::fs::{File, DirBuilder};
use std::env;

use vk_generator::{GenConfig, VkVersion, VariantPaddingConfig};


fn main() {
    let out = env::var("OUT_DIR").unwrap();
    DirBuilder::new().recursive(true).create(&out).unwrap();

    let extensions = &[
        "VK_KHR_surface",
        "VK_KHR_display",
        "VK_KHR_xlib_surface",
        "VK_KHR_xcb_surface",
        "VK_KHR_wayland_surface",
        "VK_KHR_mir_surface",
        "VK_KHR_android_surface",
        "VK_KHR_win32_surface",
        "VK_EXT_debug_report",
        "VK_MVK_ios_surface",
        "VK_MVK_macos_surface",
        "VK_MVK_moltenvk",
        "VK_NN_vi_surface",
        "VK_EXT_swapchain_colorspace",
        "VK_KHR_get_physical_device_properties2",
        "VK_KHR_swapchain",
        "VK_KHR_display_swapchain",
        "VK_KHR_sampler_mirror_clamp_to_edge",
        "VK_KHR_maintenance1",
        "VK_KHR_get_memory_requirements2",
        "VK_KHR_dedicated_allocation",
        "VK_KHR_incremental_present",
        "VK_EXT_debug_marker",
        "VK_NV_glsl_shader"
    ];

    let mut file = File::create(&Path::new(&out).join("vk_bindings.rs")).unwrap();
    vk_generator::VkRegistry::new(vk_api::VK_XML).gen_struct(
        &mut file,
        VkVersion(1, 0),
        extensions,
        GenConfig {
            use_native_enums: false,
            snake_case_commands: false,
            snake_case_members: false,
            camel_case_variants: false,
            wrap_bitmasks: false,
            wrap_non_dispatchable_handles: false,
            variant_padding: VariantPaddingConfig::RemovePrefix,
            remove_type_prefix: true,
            use_native_unions: true,
            extern_type_overrides: &[
                // Without correciton, vk_generator mis-defines these types as pointers instead of
                // c_ulongs.
                ("xcb_window_t", "std::os::raw::c_ulong"),
                ("Window", "std::os::raw::c_ulong"),
            ],
            ..GenConfig::default()
        }
    );
}
