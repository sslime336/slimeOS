macro_rules! export_macro_mods { ($($mod:ident)*) => { $( #[macro_use] pub mod $mod;)* }; }

export_macro_mods! {
    kernel_segment
    console
    hsm
    mm
    on_boot
    // proc
}
