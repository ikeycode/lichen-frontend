// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

slint::include_modules!();

use slint::{BackendSelector, CloseRequestResponse, PlatformError};

/// Main entry point for the application.
fn main() -> Result<(), PlatformError> {
    // Forced winit as Qt version quite.. er.. fugly
    // Also need to support the KMS backend at some point for
    // a more minimal installer ISO
    BackendSelector::new()
        .backend_name("winit".to_owned())
        .select()?;

    slint::set_xdg_app_id("com.serpentos.lichen-gui")?;

    // Create a new main window.
    let window = MainWindow::new()?;

    // Placeholder: Suppress closure of installer window.
    window.window().on_close_requested(|| {
        slint::quit_event_loop().expect("Whoops");
        CloseRequestResponse::HideWindow
    });

    // Run the main loop.
    window.show()?;
    slint::run_event_loop_until_quit()?;
    window.hide()?;

    Ok(())
}
