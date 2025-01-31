// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

slint::include_modules!();

use i_slint_backend_winit::{winit, WinitWindowAccessor, WinitWindowEventResult};
use slint::{platform::WindowEvent, BackendSelector, CloseRequestResponse};

fn setup_wayland_integration(window: &MainWindow) {
    // Let winit handle the minimize event
    let ww = window.as_weak();
    window.on_request_minimize(move || {
        ww.upgrade()
            .unwrap()
            .window()
            .with_winit_window(|w| {
                w.set_minimized(true);
            })
            .expect("Failed to grab winit handle")
    });

    let ww = window.as_weak();
    window.window().on_winit_window_event(move |_, e| {
        if let winit::event::WindowEvent::Resized { .. } = e {
            // Update the state in the event loop
            ww.upgrade_in_event_loop(move |e| {
                if e.get_is_maximized() != e.window().is_maximized() {
                    e.set_is_maximized(e.window().is_maximized());
                    eprintln!("Max is now {}", e.window().is_maximized());
                }
            })
            .expect("Failed to update maximized state");
        }
        WinitWindowEventResult::Propagate
    });

    // Let winit notify compositor of drag
    let ww = window.as_weak();
    window.on_drag_begin(move || {
        let ww = ww.upgrade().unwrap();
        ww.window()
            .with_winit_window(|w| {
                w.drag_window().expect("Failed to drag Wayland window");
            })
            .expect("Failed to grab winit handle");
    });

    // Maximize handling
    let ww = window.as_weak();
    window.on_request_toggle_maximize(move || {
        ww.upgrade()
            .unwrap()
            .window()
            .with_winit_window(|w| {
                w.set_maximized(!w.is_maximized());
            })
            .expect("Failed to grab winit handle");
    });

    // Insert close event for single point of handling
    let ww = window.as_weak();
    window.on_request_close(move || {
        ww.upgrade()
            .unwrap()
            .window()
            .dispatch_event(WindowEvent::CloseRequested);
    });
}

/// Main entry point for the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    window.window().set_maximized(false);
    setup_wayland_integration(&window);

    slint::run_event_loop_until_quit()?;
    window.hide()?;

    Ok(())
}
