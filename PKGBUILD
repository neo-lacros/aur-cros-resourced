# Maintainer: Shadichy <shadichy@users.noreply.github.com>

pkgname=cros-resourced
pkgver=0.1.0
pkgrel=1
pkgdesc='ChromiumOS resource manager daemon (memory/CPU/thermal/cgroup management)'
arch=('x86_64')
url='https://chromium.googlesource.com/chromiumos/platform2/+/HEAD/resourced'
license=('BSD-3-Clause')
depends=('dbus' 'gcc-libs')
makedepends=('cargo' 'git' 'patch' 'pkgconf' 'protobuf' 'rust')
source=(
  'git+https://chromium.googlesource.com/chromiumos/platform2.git'
  '0001-arch-linux-port.patch'
  'cros-resourced.service'
  'cros-resourced-sysusers.conf'
  'cros-resourced-tmpfiles.conf'
  'LICENSE'
)
sha256sums=(
  'SKIP'
  '9f21c8ad606246a042782dab153c76df8d2cc700b324e2ca0292571aa1e8b355'
  'a1d6001c417a0f3ecaac11fedf0577e9283fc2ba284686898d8f4ab76cd769e0'
  '508e5366e8911efad21c9686e8d38d68e0213216a11c5f41d722d6502f133b6d'
  '56510045824ccffb143fbff9b4c40844a8e38fc88fd9de236a70754ce0526e6c'
  '279c3789b9f439f79bc0ce6929accf3cf733f71223e9a39678f226efff6d00a5'
)

_src_root() {
  if [[ -n "${CROS_LOCAL_SRC:-}" && -d "${CROS_LOCAL_SRC}/tree/src/platform2" ]]; then
    printf '%s\n' "${CROS_LOCAL_SRC}/tree/src/platform2"
  else
    printf '%s\n' "${srcdir}/platform2"
  fi
}

prepare() {
  local src_dir
  src_dir="$(_src_root)"

  rm -rf "${srcdir}/work"
  mkdir -p "${srcdir}/work"

  cp -a "${src_dir}/resourced" "${srcdir}/work/resourced"

  # Minimal system_api stub providing only protobuf modules required by resourced.
  mkdir -p "${srcdir}/work/system_api/src"
  mkdir -p "${srcdir}/work/system_api/proto/dbus/power_manager"
  mkdir -p "${srcdir}/work/system_api/proto/dbus/vm_concierge"
  mkdir -p "${srcdir}/work/system_api/proto/dbus/resource_manager"
  mkdir -p "${srcdir}/work/system_api/proto/dbus/arc"
  mkdir -p "${srcdir}/work/system_api/proto/non_standard_ipc/vm_memory_management"
  cp "${startdir}/system_api-stub/Cargo.toml" "${srcdir}/work/system_api/Cargo.toml"
  cp "${startdir}/system_api-stub/build.rs" "${srcdir}/work/system_api/build.rs"
  cp "${startdir}/system_api-stub/src/lib.rs" "${srcdir}/work/system_api/src/lib.rs"
  cp "${src_dir}/system_api/dbus/power_manager/battery_saver.proto" \
    "${srcdir}/work/system_api/proto/dbus/power_manager/battery_saver.proto"
  cp "${src_dir}/system_api/dbus/vm_concierge/concierge_service.proto" \
    "${srcdir}/work/system_api/proto/dbus/vm_concierge/concierge_service.proto"
  cp "${src_dir}/system_api/dbus/resource_manager/resource_manager.proto" \
    "${srcdir}/work/system_api/proto/dbus/resource_manager/resource_manager.proto"
  cp "${src_dir}/system_api/dbus/arc/arc.proto" \
    "${srcdir}/work/system_api/proto/dbus/arc/arc.proto"
  cp "${src_dir}/system_api/non_standard_ipc/vm_memory_management/vm_memory_management.proto" \
    "${srcdir}/work/system_api/proto/non_standard_ipc/vm_memory_management/vm_memory_management.proto"

  cd "${srcdir}/work/resourced"
  patch -Np1 -i "${srcdir}/0001-arch-linux-port.patch"
}

build() {
  cd "${srcdir}/work/resourced"
  export CARGO_HOME="${srcdir}/cargo-home"
  export RUSTUP_TOOLCHAIN=stable
  cargo build --release --no-default-features
}

check() {
  cd "${srcdir}/work/resourced"
  export CARGO_HOME="${srcdir}/cargo-home"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --release --no-default-features
}

package() {
  install -Dm755 "${srcdir}/work/resourced/target/release/resourced" \
    "${pkgdir}/usr/bin/resourced"
  install -Dm755 "${srcdir}/work/resourced/target/release/chromeos-pgmem" \
    "${pkgdir}/usr/bin/chromeos-pgmem"
  install -Dm644 "${srcdir}/work/resourced/dbus/org.chromium.ResourceManager.conf" \
    "${pkgdir}/usr/share/dbus-1/system.d/org.chromium.ResourceManager.conf"
  install -Dm644 "${srcdir}/cros-resourced.service" \
    "${pkgdir}/usr/lib/systemd/system/cros-resourced.service"
  install -Dm644 "${srcdir}/cros-resourced-sysusers.conf" \
    "${pkgdir}/usr/lib/sysusers.d/cros-resourced.conf"
  install -Dm644 "${srcdir}/cros-resourced-tmpfiles.conf" \
    "${pkgdir}/usr/lib/tmpfiles.d/cros-resourced.conf"
  install -Dm644 "${srcdir}/work/resourced/udev/99-resourced.rules" \
    "${pkgdir}/usr/lib/udev/rules.d/99-cros-resourced.rules"
  install -Dm644 "${srcdir}/LICENSE" \
    "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
