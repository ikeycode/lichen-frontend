# lichen-frontend

A desktop-agnostic, graphical frontend for the [Lichen](https://github.com/serpent-os/lichen) installer.
Primarily created for use within [Serpent OS](https://serpentos.com) but may be made more agnostic in time
if interest exists.

Written in Rust, utilising the [Slint](https://slint.dev) to provide an accelerated, elegant and native
installer that could in future be extended to support multiple usage scenarios, including kiosk modes or
handheld devices.

![Made With Slint](https://raw.githubusercontent.com/slint-ui/slint/refs/heads/master/logo/MadeWithSlint-logo-whitebg.png)

## Status

This is a brand new project and will be riddled with warts until we "find our way".
We currently track Slint from git, a little beyond `1.9.2`, and will settle on a locked
version in due course.

Planned:

 - Beautification-port of `lichen_cli` to Slint (The CLI will option will ALWAYS be supported)
 - Enable IPC for `lichen` and `moss` through a very tightly controlled test scenario.

Planned later:

 - Translations (bundled + dynamic selection at runtime)
 - `$stuff`

## License

`lichen-frontend` is available under the terms of the [MPL-2.0](https://spdx.org/licenses/MPL-2.0.html)

Our use of [Slint](https://slint.dev) is pursuant to the terms of the [Slint Royalty-free Desktop, Mobile, and Web Applications License](https://slint.dev/terms-and-conditions#royalty-free)

### Artwork

`ui/images/adwaita/*.svg` are created by/copyright of the [GNOME Project](https://www.gnome.org) and are licensed under the [CC-BY-SA-3.0](https://creativecommons.org/licenses/by-sa/3.0/) license.
