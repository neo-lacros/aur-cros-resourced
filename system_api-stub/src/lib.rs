// Minimal system_api stub for cros-resourced on Arch Linux.
// Only protobuf modules required by resourced are generated.

#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]
pub mod arc {
    include!(concat!(env!("OUT_DIR"), "/arc.rs"));
}

#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]
pub mod battery_saver {
    include!(concat!(env!("OUT_DIR"), "/battery_saver.rs"));
}

#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]
pub mod concierge_service {
    include!(concat!(env!("OUT_DIR"), "/concierge_service.rs"));
}

#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]
pub mod resource_manager {
    include!(concat!(env!("OUT_DIR"), "/resource_manager.rs"));
}

#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]
pub mod vm_memory_management {
    include!(concat!(env!("OUT_DIR"), "/vm_memory_management.rs"));
}
