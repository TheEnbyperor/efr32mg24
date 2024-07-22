fn main() {
    println!("cargo::rerun-if-changed=config/");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let compile_options = &[
        "-ggdb",
        "-march=armv8-m.main+fp",
        "-mfloat-abi=hard",
        "-mcmse",
        "-DEFR32MG24B210F1536IM48",
        "-DSL_CATALOG_POWER_MANAGER_PRESENT",
        "-D__VECTOR_TABLE=__vector_table",
        "-D__WEAK=__attribute__((weak))",
        "-Isisdk-sdk/platform/CMSIS/Core/Include/",
        "-Isisdk-sdk/platform/Device/SiliconLabs/EFR32MG24/Include",
        "-Isisdk-sdk/platform/common/inc/",
        "-Isisdk-sdk/platform/emlib/inc/",
        "-Isisdk-sdk/platform/service/hfxo_manager/inc/",
        "-Isisdk-sdk/platform/service/device_manager/inc/",
        "-Isisdk-sdk/platform/service/clock_manager/inc/",
        "-Isisdk-sdk/platform/service/power_manager/inc/",
        "-Isisdk-sdk/platform/service/sleeptimer/inc/",
        "-Iconfig",
    ];

    let compile_sources = &[
        &std::path::Path::new("sisdk-sdk/platform/emlib/src/em_timer.c"),
        &std::path::Path::new("sisdk-sdk/platform/emlib/src/em_cmu.c"),
        &std::path::Path::new("sisdk-sdk/platform/emlib/src/em_emu.c"),
        &std::path::Path::new("sisdk-sdk/platform/emlib/src/em_gpio.c"),
        &std::path::Path::new("sisdk-sdk/platform/common/src/sl_core_cortexm.c"),
        &std::path::Path::new("sisdk-sdk/platform/common/src/sl_slist.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/hfxo_manager/src/sl_hfxo_manager.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/hfxo_manager/src/sl_hfxo_manager_hal_s2.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/sleeptimer/src/sl_sleeptimer.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/sleeptimer/src/sl_sleeptimer_hal_timer.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/power_manager/src/sl_power_manager.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/power_manager/src/sl_power_manager_hal_s2.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/clock_manager/src/sl_clock_manager.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/clock_manager/src/sl_clock_manager_hal_s2.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/device_manager/src/sl_device_peripheral.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/device_manager/devices/sl_device_peripheral_hal_efr32xg24.c"),
        &std::path::Path::new("sisdk-sdk/platform/service/device_manager/clocks/sl_device_clock_efr32xg24.c"),
        &std::path::Path::new("sisdk-sdk/platform/Device/SiliconLabs/EFR32MG24/Source/system_efr32mg24.c"),
    ];

    let mut compiled_sources = vec![];

    for source in compile_sources {
        let file_name = source.file_name().unwrap().to_str().unwrap();
        let obj_name = file_name.replace(".c", ".o");

        if !std::process::Command::new("arm-none-eabi-gcc")
            .args(compile_options)
            .arg(source)
            .arg("-c")
            .arg("-Os")
            .arg("-o")
            .arg(&format!("{}/{}", out_dir, obj_name))
            .status().unwrap().success() {
            panic!("Failed to build {}", file_name);
        }

        compiled_sources.push(obj_name);
    }

    let mut ar_cmd = std::process::Command::new("arm-none-eabi-ar");
    ar_cmd.current_dir(&std::path::Path::new(&out_dir));
    ar_cmd.arg("crus");
    ar_cmd.arg("libsdk.a");

    for obj_file in &compiled_sources {
        ar_cmd.arg(obj_file);
    }

    if !ar_cmd.status().unwrap().success() {
        panic!("Failed to link SDK");
    }

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static:+whole-archive=sdk");

    let mut bindings_b = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-D__INLINE=__inline__")
        .clang_arg("-Iconfig")
        .clang_arg("-Isisdk-sdk/platform/Device/SiliconLabs/EFR32MG24/Include")
        .clang_arg("-Isisdk-sdk/platform/CMSIS/Core/Include")
        .clang_arg("-Isisdk-sdk/platform/common/inc")
        .clang_arg("-Isisdk-sdk/platform/service/system/inc")
        .clang_arg("-Isisdk-sdk/platform/service/sleeptimer/inc")
        .clang_arg("-Isisdk-sdk/platform/service/power_manager/inc")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core();

    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("xcode-select")
            .arg("-p")
            .output()
            .expect("Failed to get MacOS SDK path");
        let sdk_path = String::from_utf8(output.stdout)
            .expect("Failed to get MacOS SDK path");
        bindings_b = bindings_b.clang_arg(format!("-I{}/SDKs/MacOSX.sdk/usr/include", sdk_path.trim()));
    }

    let bindings = bindings_b.generate() .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}