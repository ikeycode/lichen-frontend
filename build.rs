// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

fn main() {
    let config = slint_build::CompilerConfiguration::new();
    slint_build::compile_with_config("ui/index.slint", config)
        .expect("Failed to compile UI assets");
}
