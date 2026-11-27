// Minimal system_api build script for cros-resourced on Arch Linux.
// Generates only the protobuf modules required by resourced.

use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let proto_root = manifest_dir.join("proto");

    let protos: &[(&str, &str)] = &[
        ("arc", "dbus/arc/arc.proto"),
        ("battery_saver", "dbus/power_manager/battery_saver.proto"),
        (
            "concierge_service",
            "dbus/vm_concierge/concierge_service.proto",
        ),
        (
            "resource_manager",
            "dbus/resource_manager/resource_manager.proto",
        ),
        (
            "vm_memory_management",
            "non_standard_ipc/vm_memory_management/vm_memory_management.proto",
        ),
    ];

    for (module, rel) in protos {
        let input_path = proto_root.join(rel);
        let input_dir = input_path.parent().unwrap();
        let dbus_include = proto_root.join("dbus");

        protobuf_codegen::Codegen::new()
            .input(input_path.to_str().unwrap())
            .include(input_dir.to_str().unwrap())
            .include(dbus_include.to_str().unwrap())
            .out_dir(out_dir.to_str().unwrap())
            .run()
            .expect("protobuf-codegen");

        let out_file = out_dir.join(format!("{module}.rs"));
        if let Ok(content) = std::fs::read_to_string(&out_file) {
            let sanitized = content
                .lines()
                .filter(|line| !line.starts_with("#!"))
                .map(|line| {
                    if let Some(rest) = line.strip_prefix("//!") {
                        format!("//{rest}")
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write(&out_file, sanitized).expect("write sanitized proto");
        }

        println!("cargo:rerun-if-changed={}", input_path.display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}
