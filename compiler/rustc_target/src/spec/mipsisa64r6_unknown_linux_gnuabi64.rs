use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "mipsisa64r6-unknown-linux-gnuabi64".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-i8:8:32-i16:16:32-i64:64-n32:64-S128".to_string(),
        arch: "mips64".to_string(),
        target_os: "linux".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            // NOTE(mips64r6) matches C toolchain
            cpu: "mips64r6".to_string(),
            features: "+mips64r6".to_string(),
            max_atomic_width: Some(64),
            target_mcount: "_mcount".to_string(),

            ..super::linux_base::opts()
        },
    }
}
