# Slint UI Development Docs

Source root: `D:\DEV\dell-controller\slint-docs\Slint docs\docs.slint.dev\latest\docs\slint`

HTML pages included: 113
Mirrored 404 pages skipped: 2

Skipped pages:

- `reference/std-widgets/style/slint-node_functions/loadFile.html`
- `reference/std-widgets/style/slint-node_interfaces/LoadFileOptions.html`

# Home

## Welcome to Slint

Source: `/`
Official URL: https://docs.slint.dev/latest/docs/slint/

Seamlessly build elegant GUIs for Embedded, Desktop, and Mobile

![Slint examples running on a range of devices](https://docs.slint.dev/latest/docs/slint/_astro/banner.9SyOpZa4_Z1bmAmv.webp)

[YouTube video](https://www.youtube.com/watch?v=vWLXaXJkCWw)

The documentation is split into several sections:

[Guide](https://docs.slint.dev/latest/docs/slint/index.html) Get up and running with the tooling for Slint development including the IDE integration via the Slint Language Server (LSP) and Slint Viewer. Learn all the key concepts of Slint to understand and use the Slint language.

[Reference](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html) Browse the API reference for all aspects of the Slint language: The elements, properties, functions, callbacks, namespaces, as well as the std-widgets library. A set of cross platform components that can be used to build desktop applications.

[Tutorial](https://docs.slint.dev/latest/docs/slint/tutorial/quickstart/index.html) Learn by example and see how to structure a simple application via a step-by-step tutorial that walks you through the creation of a simple memory game example.

[Language Integrations](https://docs.slint.dev/latest/docs/slint/language-integrations/index.html) API reference for the Rust, C++, JavaScript, and Python versions of Slint.

#### Documentation features

The documentation includes a lot of code snippets. The language hint lets you know what language the snippet is written in.

![Screenshot showing language hint](https://docs.slint.dev/latest/docs/slint/_astro/language-hint.DWqXTVMy_ZEdr0e.webp)

Some snippets of Slint code are interactive. You can click the `run` button to run the snippet in a web based live-editing tool called [SlintPad↗](https://slintpad.com/).

![Screenshot showing run button](https://docs.slint.dev/latest/docs/slint/_astro/run-in-slintpad.CRjFQrOU_1xLqm8.webp)

To easily copy the text of a snippet use the copy button.

![Screenshot showing copy button](https://docs.slint.dev/latest/docs/slint/_astro/copy-snippet.D35aR1Qg_fzW27.webp)

Examples that want to help focus on a specific part of the code will have highlights. They are only a documentation feature and you won’t see this kind of highlight when writing your own code.

![Screenshot showing text highlights](https://docs.slint.dev/latest/docs/slint/_astro/line-highlight.CV2WK2QV_ZonX6t.webp)

#### Get in touch

[Chat](https://chat.slint.dev/)

[Discussions](https://github.com/slint-ui/slint/discussions)

[Report Bugs](https://github.com/slint-ui/slint/issues)

[Email](https://docs.slint.dev/cdn-cgi/l/email-protection#1871767e77586b7471766c367c7d6e)

# Guide

## LinuxKMS Backend

Source: `guide/backends-and-renderers/backend_linuxkms/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_linuxkms/

The LinuxKMS backend runs only on Linux and eliminates the need for a windowing system such as Wayland or X11. Instead it uses the following libraries and interface to render directly to the screen and react to touch, mouse, and keyboard input.

- OpenGL via KMS/DRI.
- Vulkan via wgpu and the Vulkan KHR Display Extension.
- DRM dumb buffers for software rendering, as well as legacy LinuxFB rendering.
- libinput/libudev for input event handling from mice, touch screens, or keyboards.
- libseat for GPU and input device access without requiring root access. (optional)

### Dependencies

For compilation, pkg-config is used to determine the location of the following required system libraries:

| pkg-config package name | Package name on Debian based distros |
| --- | --- |
| `gbm` | `libgbm-dev` |
| `xkbcommon` | `libxkbcommon-dev` |
| `libudev` | `libudev-dev` |
| `libseat` | `libseat-dev` |

> **Note**
> If you don’t have `libseat` available on your target system, then instead of selecting `backend-linuxkms`, select `backend-linuxkms-noseat`. This variant of the LinuxKMS backend eliminates the need to have libseat installed, but in exchange requires running the application as a user that’s privileged to access all input and DRM/KMS device files; typically that’s the root user.

### Renderers

The LinuxKMS backend supports different renderers. They can be explicitly selected for use through the `SLINT_BACKEND` environment variable.

| Renderer name | Required Graphics APIs | `SLINT_BACKEND` value to select renderer |
| --- | --- | --- |
| FemtoVG | OpenGL ES 2.0 | `linuxkms-femtovg` |
| FemtoVG (wgpu) | Vulkan | `linuxkms-femtovg-wgpu` |
| Skia | OpenGL ES 2.0, Vulkan | `linuxkms-skia-opengl`, `linuxkms-skia-vulkan`, or `linuxkms-skia-software` |
| Software | None | `linuxkms-software` |

> **Note**
> This backend is still experimental. The backend has not undergone a great variety of testing on different devices and there are [known issues↗](https://github.com/slint-ui/slint/labels/a%3Abackend-linuxkms).

> **Note**
> A mouse is supported as input device, but rendering of the mouse cursor only works with the Skia and FemtoVG renderers, not with the Slint software renderer.

### Display Selection

All renderers use Linux’s direct rendering manager (DRM) subsystem to configure display outputs. Slint defaults to selecting the first connected display and configures it at either its preferred resolution (if available) or its highest. Set the `SLINT_DRM_OUTPUT` environment variable to select a specific display. To get a list of available outputs, set `SLINT_DRM_OUTPUT` to `list`, run your program, and observe the output.

For example, the output may look like this on a laptop with a built-in screen (eDP-1) and an externally connected monitor (DP-3):

Setting `SLINT_DRM_OUTPUT` to `DP-3` will render on the second monitor.

To select a specific resolution and refresh rate (mode), set the `SLINT_DRM_MODE` variable. Set it to `list` and run your program to get a list of available modes. For example the program output could look like this:

```plaintext
DRM Mode List Requested:
Index: 0 Width: 3840 Height: 2160 Refresh Rate: 60
Index: 1 Width: 3840 Height: 2160 Refresh Rate: 50
Index: 2 Width: 3840 Height: 2160 Refresh Rate: 30
Index: 3 Width: 2560 Height: 1440 Refresh Rate: 59
Index: 4 Width: 1920 Height: 1080 Refresh Rate: 60
Index: 5 Width: 1680 Height: 1050 Refresh Rate: 59
...
```

Set `SLINT_DRM_MODE` to `4` to select 1920x1080@60.

### Configuring the Keyboard

By default the keyboard layout and model is assumed to be a US model and layout. Set the following environment variables to configure support for different keyboards:

- `XKB_DEFAULT_LAYOUT` : A comma separated list of layouts (languages) to include in the keymap. See the layouts section in [xkeyboard-config(7)↗](https://manpages.debian.org/testing/xkb-data/xkeyboard-config.7.en.html) for a list of accepted language codes. for a list of supported layouts.
- `XKB_DEFAULT_MODEL` : The keyboard model by which to interpreter keys. See the models section in [xkeyboard-config(7)↗](https://manpages.debian.org/testing/xkb-data/xkeyboard-config.7.en.html) for a list of accepted model codes.
- `XKB_DEFAULT_VARIANT` : A comma separated list of variants, one per layout, which configures layout specific variants. See the values in parentheses in the layouts section in [xkeyboard-config(7)↗](https://manpages.debian.org/testing/xkb-data/xkeyboard-config.7.en.html) for a list of accepted variant codes.
- `XKB_DEFAULT_OPTIONS` : A comma separated list of options to configure layout-independent key combinations. See the options section in [xkeyboard-config(7)↗](https://manpages.debian.org/testing/xkb-data/xkeyboard-config.7.en.html) for a list of accepted option codes.

### Display Rotation

If your display’s default orientation does not match the desired orientation of your user interface, then you can set the `SLINT_KMS_ROTATION` environment variable to instruct Slint to rotate at rendering time. Supported values are the rotation in degrees: `0`, `90`, `180`, and `270`.

Note that this variable merely rotates the rendering output. If you’re using a touch screen attached to the same display, then you may need to configure it to also apply a rotation on the touch events generated. For configuring libinput’s `LIBINPUT_CALIBRATION_MATRIX` see the [libinput Documentation↗](https://wayland.freedesktop.org/libinput/doc/latest/device-configuration-via-udev.html#static-device-configuration-via-udev) for a list of valid values. Values can typically be set by writing them into a rules file under `/etc/udev/rules.d`.

The following example configures libinput to apply a 90 degree clockwise rotation for any attached touch screen:

```bash
echo 'ENV{LIBINPUT_CALIBRATION_MATRIX}="0 -1 1 1 0 0"' > /etc/udev/rules.d/libinput.rules
udevadm control --reload-rules
udevadm trigger
```

### Legacy LinuxFB Interface

For software rendering, DRM dumb buffers are the preferred default way of posting frame buffers to the display. If DRM dumb buffers are not supported, the LinuxKMS backend falls back to using the Linux legacy framebuffer interface (`/dev/fbX`).

To override this default and use only the legacy framebuffer interface, set the `SLINT_BACKEND_LINUXFB=1` environment variable.

## Qt Backend

Source: `guide/backends-and-renderers/backend_qt/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_qt/

The Qt backend uses the [Qt↗](https://www.qt.io/) library to interact with the windowing system, for rendering, as well as widget style for a native look and feel.

The Qt backend supports practically all relevant operating systems and windowing systems, including macOS, Windows, Linux with Wayland and X11, and direct full-screen rendering via KMS or proprietary drivers.

The Qt backend only supports software rendering at the moment. That means it runs with any graphics driver, but it does not utilize GPU hardware acceleration.

The compilation step will detect whether Qt is installed or not using the qttype crate. See the instructions in the [qttypes documentation↗](https://docs.rs/qttypes/latest/qttypes/#finding-qt) on how to set environment variables to point to the Qt installation.

If Qt is not installed, the backend will be disabled, and Slint will fallback to another backend, usually the [Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/index.html).

### Configuration Options

The Qt backend reads and interprets the following environment variables:

| Name | Accepted Values | Description |
| --- | --- | --- |
| `SLINT_FULLSCREEN` | any value | If this variable is set, every window is shown in fullscreen mode. |

## Winit Backend

Source: `guide/backends-and-renderers/backend_winit/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/

The Winit backend uses the [winit↗](https://docs.rs/winit/latest/winit/) library to interact with the windowing system.

The Winit backend supports practically all relevant operating systems and windowing systems, including macOS, Windows, Linux with Wayland and X11.

The Winit backend supports different renderers. They can be explicitly selected for use through the `SLINT_BACKEND` environment variable.

| Renderer name | Supported/Required Graphics APIs | `SLINT_BACKEND` value to select renderer |
| --- | --- | --- |
| FemtoVG | OpenGL | `winit-femtovg` |
| FemtoVG (WGPU) | Metal, Direct3D, Vulkan with ([http://wgpu.rs↗](http://wgpu.rs/)) | `winit-femtovg-wgpu` |
| Skia | OpenGL, Metal, Direct3D, Software-rendering | `winit-skia` |
| Skia Software | Software-only rendering with Skia | `winit-skia-software` |
| Skia OpenGL | OpenGL rendering with Skia (not supported on iOS) | `winit-skia-opengl` |
| software | Software-rendering, no GPU required | `winit-software` |

If no renderer is explicitly set, the backend will first try to use the Skia renderer, if it was enabled at compile time. If that fails, it will fall back to the FemtoVG renderer, and if that also fails, it will use the software renderer.

### Configuration Options

The Winit backend reads and interprets the following environment variables:

| Name | Accepted Values | Description |
| --- | --- | --- |
| `SLINT_FULLSCREEN` | any value | If this variable is set, every window is shown in fullscreen mode. |

### Linux Dependencies

On Linux, the Winit backend requires either X11 or Wayland to be available. Support of either can be enabled or disabled at compile time by setting the `backend-winit-x11` or `backend-winit-wayland` features (instead of `backend-winit`).

For X11 the following runtime dependencies are required: libx11-xcb, xinput, libxcursor, libxkbcommon-x11, libx11. On Debian-based systems, these can be installed with:

## Backends & Renderers

Source: `guide/backends-and-renderers/backends_and_renderers/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backends_and_renderers/

In Slint, a backend is the module that encapsulates the interaction with the operating system, in particular the windowing sub-system. Multiple backends can be compiled into Slint and one backend is selected for use at run-time on application start-up. You can configure Slint without any built-in backends, and instead develop your own backend by implementing Slint’s platform abstraction and window adapter interfaces.

The backend is selected as follows:

1. The developer provides their own backend and sets it programmatically.
2. Else, the backend is selected by the value of the `SLINT_BACKEND` environment variable, if it is set.
3. Else, backends are tried for initialization in the following order:
  1. qt
  2. winit
  3. linuxkms

The following table provides an overview over the built-in backends. For more information about the backend’s capabilities and their configuration options, see the respective sub-pages.

| Backend Name | Description | Built-in by Default |
| --- | --- | --- |
| qt | The Qt library is used for windowing system integration, rendering, and native widget styling. | On Linux if Qt is installed |
| winit | The [winit↗](https://docs.rs/winit/latest/winit/) library is used to interact with the windowing system. | Yes |
| linuxkms | Linux’s KMS/DRI infrastructure is used for rendering. No windowing system or compositor is required. | No |

A backend is also responsible for selecting a renderer. See the [Renderers](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backends_and_renderers/#renderers) section for an overview. Override the choice of renderer by adding the name to the `SLINT_BACKEND` environment variable, separated by a dash. For example if you want to choose the `winit` backend in combination with the `software` renderer, set `SLINT_BACKEND=winit-software`. Similarly, `SLINT_BACKEND=linuxkms-skia` chooses the `linuxkms` backend and then instructs the LinuxKMS backend to use Skia for rendering.

### Renderers

Slint comes with different renderers that use different techniques and libraries to turn your scene of elements into pixels. Slint picks a renderer based on your choice of backend as well as the features you’ve selected at Slint compilation time.

**C++**

When building Slint from source, check the [`SLINT_FEATURE_RENDERER_*`](https://docs.slint.dev/latest/docs/cpp/cmake.html#features) cmake options to enable one of the available renderers.

**Rust**

With Rust, enable one of the [`renderer-*`](https://docs.slint.dev/latest/docs/rust/slint/docs/cargo_features/) features.

**NodeJS**

With Node.js, all renderers are built into the packages.

**Python**

With Python, all renderers are built into the packages.

#### Qt Renderer

The Qt renderer comes with the [Qt backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_qt/index.html) and renders using QPainter:

- Software rendering, no GPU acceleration.
- Available only in the Qt backend.

#### Software Renderer

- Runs anywhere, highly portable, and lightweight.
- Software rendering, no GPU acceleration.
- Supports partial rendering.
- Supports line-by-line rendering (Rust only).
- Suitable for Microcontrollers.
- Some features haven’t been implemented yet:
  - No support for rotations or scaling.
  - No support for `drop-shadow-*` properties.
  - No support for `border-radius` in combination with `clip: true` .
  - No text stroking/outlining.
- Text rendering currently limited to western scripts.
- `Path` elements in `no_std` environments requires enabling the `software-renderer-path` feature.
- Available in `no_std` environments as well as in the [Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/index.html) and [LinuxKMS backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_linuxkms/index.html) .
- Public [Rust](https://docs.slint.dev/latest/docs/rust/slint/platform/software_renderer/) and [C++](https://docs.slint.dev/latest/docs/cpp/api/classslint_1_1platform_1_1SoftwareRenderer) API.

#### FemtoVG Renderer

- Highly portable.
- GPU acceleration with OpenGL (required). When selected as `renderer-femtovg-wgpu` , GPU acceleration with Metal, Vulkan, and Direct3D.
- Text and path rendering quality sometimes sub-optimal.
- Available in the [Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/index.html) and [LinuxKMS backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_linuxkms/index.html) .
- Public [Rust](https://docs.slint.dev/latest/docs/rust/slint/platform/femtovg_renderer/) API.

#### Skia Renderer

- Sophisticated GPU acceleration with OpenGL, Metal, Vulkan, and Direct3D.
- Heavy disk-footprint compared to other renderers.
- Available in the [Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/index.html) and [LinuxKMS backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_linuxkms/index.html) .
- Public [C++](https://docs.slint.dev/latest/docs/cpp/api/classslint_1_1platform_1_1SkiaRenderer) API.

##### Troubleshooting

You may run into compile issues when enabling the Skia renderer. The following sections track issues we’re aware of and how to resolve them.

- Compilation error on Windows with messages about multiple source files and unused linker input You may see compile errors that contain this error and warning from clang-cl: The Skia sources are checked out in a path that’s managed by Cargo, the Rust package manager. The error happens when that path contains spaces. By default that’s in `%HOMEPATH%\.cargo`, which contains spaces if the login name contains spaces. To resolve this issue, set the `CARGO_HOME` environment variable to a path without spaces, such as `c:\cargo_home`.
- Compilation error when compiling for ARMv7 with hardware floating-point support You may see compiler errors that contain this message: ```plaintext Unable to generate bindings: ClangDiagnostic("/home/runner/work/slint/yocto-sdk/sysroots/cortexa15t2hf-neon-poky-linux-gnueabi/usr/include/gnu/stubs-32.h:7:11: fatal error: 'gnu/stubs-soft.h' file not found\n") ``` The Skia build invokes clang in multiple occasions and is sensitive to compiler flags that affect the floating point abi (such as `-mfloat-abi=hard`), as they affect header file lookups. The solve this, set the `BINDGEN_EXTRA_CLANG_ARGS` environment variable to contain the same flags that your build environment also passes to the C++ compiler. For example, if you’re building against a Yocto SDK, then you can find these flags in the `OECORE_TUNE_CCARGS` environment variable.
- Compilation error when linking on Windows You may see compiler errors that contain this message: ```plaintext error: linking with `link.exe` failed: exit code: 1120 | ... = note: skunicode.lib(icu.SkLoadICU.obj) : error LNK2019: unresolved external symbol __std_init_once_begin_initialize_clr referenced in function "bool __cdecl SkLoadICU(void)" (?SkLoadICU@@YA_NXZ) ... skia.lib(skia.SkNWayCanvas.obj) : error LNK2001: unresolved external symbol __std_find_trivial_8 ``` The Skia build requires the use of Microsoft Visual Studio 2022 as compiler. Make sure to have the latest patches to the compiler installed.
- Compilation error on macOS:

The build fails and somewhere in the log output you see this message:

```plaintext
cargo:warning=xcrun: error: unable to lookup item 'PlatformVersion' from command line tools installation
cargo:warning=xcrun: error: unable to lookup item 'PlatformVersion' in SDK '/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk'
```

This is due the build process calling `xcrun --show-sdk-platform-version` to determine the SDK version, and that’s unfortunately not supported by the Xcode command line tools. To solve this issue, run the following command once:

```plaintext
sudo xcode-select -switch /Applications/Xcode.app/Contents/Developer
```

- Compilation error when cross-compiling with Yocto:

The build fails and towards the end of the log message you see this message:

```plaintext
    error occurred: unknown target `arm-org-linux-gnueabi`
```

This is caused by an unfortunate combination of updates of the `cc` crate and the way the Skia Rust bindings use it. We anticipate solving this in a future release. In the meantime, you can work around this by downgrading the `cc` crate in your `Cargo.lock` file:

```plaintext
cargo update -p cc --precise 1.1.31
```

- The application doesn’t start on Windows but exist immediately

The build succeeds, but running the `.exe` on Windows terminates immediately without error messages.

This might be caused by missing MSVC runtime libraries. To solve this install, install the [Microsoft Visual C++ Redistributable package↗](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)

## Best Practices

Source: `guide/development/best-practices/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/best-practices/

In the following sections, we provide you with lessons we’ve learned of the years. This will help you avoid some pitfalls, tricky situations, and improve maintainability of your UI code.

### Accessibility

When designing custom components, consider early on to declare [accessibility properties](https://docs.slint.dev/latest/docs/slint/reference/common/index.html#accessibility-properties). At least a role, possibly a label, as well as actions.

If you’re developing on Windows, then [Accessibility Insights↗](https://accessibilityinsights.io/docs/windows/overview/) tool for windows is a great tool to help you find and fix accessibility issues.

If you’re developing on macOS, the [Accessibility Inspector↗](https://developer.apple.com/documentation/accessibility/accessibility-inspector) offers similar functionality. Note that it requires the application to be built as bundle.

### Separate Code, UI, and Assets

Many projects start out small, with just a few files. But before you know it, your team grows, files get added, and it becomes harder to see forest for the trees. We recommend starting with the following basic directory structure:

```plaintext
my-project
├── src
│   ├── main.cpp / main.rs / main.js / main.py
│        <this is where your main business logic lives>
├── ui
    ├── app-window.slint <the entry point for your Slint UI>
    ├── <additional .slint files here>
    ├── images
        ├── logo.svg
        ├── highlight-marker.svg
        ├── <all your images go here>
```

### Translations

- When adding user-visible strings to your UI, consider early on to mark them as [translatable](https://docs.slint.dev/latest/docs/slint/guide/development/translations/index.html) by wrapping them in `@tr("...")` .
- Avoid `+` for concatenating strings, prefer `{}` substitutions. This gives translators the option of re-ordering the arguments for the most natural translation.

> **Caution**
> ```slint
> export component Example {
>     property <string> name;
>
>     Text {
>         text: "Ink Level Controls"; // Oooops, forgot to mark as translatable
>     }
>     Text {
>         text: @tr("Hello, ") + name; // Oooops, this is difficult to translate
>     }
> }
> ```

> **Tip**
> ```slint
> export component Example {
>     property <string> name;
>
>     Text {
>         text: @tr("Ink Level Controls");
>     }
>     Text {
>         text: @tr("Hello, {}", name); // Good, now the translator can re-order
>     }
> }
> ```

## Custom Controls

Source: `guide/development/custom-controls/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/custom-controls/

## Custom Control Introduction

### A Clickable Button

In this first example, you see the basics of the Slint language:

- We import the `VerticalBox` layout and the `Button` widget from the standard library using the `import` statement. This statement can import widgets or your own components declared in different files. You don’t need to import built-in element such as `Window` or `Rectangle` .
- We declare the `Recipe` component using the `component` keyword. `Recipe` inherits from `Window` and has elements: A layout ( `VerticalBox` ) with one button.
- You instantiate elements using their name followed by a pair of braces (with optional contents. You can assign a name to a specific element using `:=`
- Elements have properties. Use `:` to set property values. Here we assign a binding that computes a string by concatenating some string literals, and the `counter` property to the `Button` ’s `text` property.
- You can declare custom properties for any element with `property <...>` . A property needs to have a type, and can have a default value and an access specifier. Access specifiers like `private` , `in` , `out` or `in-out` defines how outside elements can interact with the property. `Private` is the default value and stops any outside element from accessing the property. The `counter` property is custom in this example.
- Elements can also have callback. In this case we assign a callback handler to the `clicked` callback of the `button` with `=> { ... }` .
- Property bindings are automatically re-evaluated if any of the properties the binding depends on changes. The `text` binding of the button is automatically re-computed whenever the `counter` changes.

### React to a Button Click in Native Code

This example increments the `counter` using native code:

```slint
import { VerticalBox, Button } from "std-widgets.slint";
export component Recipe inherits Window {
    in-out property <int> counter: 0;
    callback button-pressed <=> button.clicked;
    VerticalBox {
        button := Button {
            text: "Button, pressed " + root.counter + " times";
        }
    }
}
```

The `<=>` syntax binds two callbacks together. Here the new `button-pressed` callback binds to `button.clicked`.

The root element of the main component exposes all non-`private` properties and callbacks to native code.

In Slint, `-` and `_` are equivalent and interchangeable in all identifiers. This is different in native code: Most programming languages forbid `-` in identifiers, so `-` is replaced with `_`.

**Rust**

For technical reasons, this example uses `export {Recipe}` in the `slint!` macro. In real code, you can put the whole Slint code in the `slint!` macro, or use an external `.slint` file together with a build script.

```rust
slint::slint!(export { Recipe } from "docs/reference/src/recipes/button_native.slint";);

fn main() {
    let recipe = Recipe::new().unwrap();
    let recipe_weak = recipe.as_weak();
    recipe.on_button_pressed(move || {
        let recipe = recipe_weak.upgrade().unwrap();
        let mut value = recipe.get_counter();
        value = value + 1;
        recipe.set_counter(value);
    });
    recipe.run().unwrap();
}
```

The Slint compiler generates a `struct Recipe` with a getter (`get_counter`) and a setter (`set_counter`) for each accessible property of the root element of the `Recipe` component. It also generates a function for each accessible callback, like in this case `on_button_pressed`.

The `Recipe` struct implements the [`slint::ComponentHandle`](https://docs.slint.dev/latest/docs/rust/slint/trait.ComponentHandle.html) trait. A component manages a strong and a weak reference count, similar to an `Rc`. We call the `as_weak` function to get a weak handle to the component, which we can move into the callback.

We can’t use a strong handle here, because that would form a cycle: The component handle has ownership of the callback, which itself has ownership of the closure’s captured variables.

**C++**

In C++ you can write

```cpp
#include "button_native.h"

int main(int argc, char **argv)
{
    auto recipe = Recipe::create();
    recipe->on_button_pressed([&]() {
        auto value = recipe->get_counter();
        value += 1;
        recipe->set_counter(value);
    });
    recipe->run();
}
```

The CMake integration handles the Slint compiler invocations as needed, which will parse the `.slint` file and generate the `button_native.h` header.

This header file contains a `Recipe` class with a getter and setter for each accessible property, as well as a function to set up a callback for each accessible callback in `Recipe`. In this case we will have `get_counter`, `set_counter` to access the `counter` property and `on_button_pressed` to set up the callback.

**Python**

In Python, you can write:

```python
import slint

class App(slint.loader.recipe.Recipe):
    @slint.callback
    def button_pressed(self):
        value = self.counter
        value = value + 1
        self.counter = value

app = App()
app.run()
```

The Slint auto-loader provides a `Recipe` class originating from `recipe.slint`, which is subclassed. The `Recipe` class provides the `counter` property, and the `@slint.callback` decorator connects the `button_pressed` method with the `button-pressed` callback.

### Use Property Bindings to Synchronize Controls

```slint
import { VerticalBox, Slider } from "std-widgets.slint";
export component Recipe inherits Window {
    VerticalBox {
        slider := Slider {
            maximum: 100;
        }
        Text {
            text: "Value: \{round(slider.value)}";
        }
    }
}
```

This example introduces the `Slider` widget.

It also introduces interpolation in string literals: Use `\{...}` to render the result of code between the curly braces as a string.

## Animation Examples

### Animate the Position of an Element

```slint
import { CheckBox } from "std-widgets.slint";
export component Recipe inherits Window {
    width: 200px;
    height: 100px;

    rect := Rectangle {
        x:0;
        y: 5px;
        width: 40px;
        height: 40px;
        background: blue;
        animate x {
            duration: 500ms;
            easing: ease-in-out;
        }
    }

    CheckBox {
        y: 25px;
        text: "Align rect to the right";
        toggled => {
            if (self.checked) {
                 rect.x = parent.width - rect.width;
            } else {
                 rect.x = 0px;
            }
        }
    }
}
```

Layouts position elements automatically. In this example we manually position elements instead, using the `x`, `y`, `width`, `height` properties.

Notice the `animate x` block that specifies an animation. It’s run whenever the property changes: Either because a callback sets the property, or because its binding value changes.

### Animation Sequence

```slint
import { CheckBox } from "std-widgets.slint";
export component Recipe inherits Window {
    width: 200px;
    height: 100px;

    rect := Rectangle {
        x:0;
        y: 5px;
        width: 40px;
        height: 40px;
        background: blue;
        animate x {
            duration: 500ms;
            easing: ease-in-out;
        }
        animate y {
            duration: 250ms;
            delay: 500ms;
            easing: ease-in;
        }
    }

    CheckBox {
        y: 25px;
        text: "Align rect bottom right";
        toggled => {
            if (self.checked) {
                 rect.x = parent.width - rect.width;
                 rect.y = parent.height - rect.height;
            } else {
                 rect.x = 0px;
                 rect.y = 0px;
            }
        }
    }
}
```

This example uses the `delay` property to make one animation run after another.

## States Examples

### Associate Property Values With States

```slint
import { HorizontalBox, VerticalBox, Button } from "std-widgets.slint";

component Circle inherits Rectangle {
    width: 30px;
    height: 30px;
    border-radius: root.width / 2;
    animate x { duration: 250ms; easing: ease-in; }
    animate y { duration: 250ms; easing: ease-in-out; }
    animate background { duration: 250ms; }
}

export component Recipe inherits Window {
    states [
        left-aligned when b1.pressed: {
            circle1.x: 0px; circle1.y: 40px; circle1.background: green;
            circle2.x: 0px; circle2.y: 0px; circle2.background: blue;
        }
        right-aligned when b2.pressed: {
            circle1.x: 170px; circle1.y: 70px; circle1.background: green;
            circle2.x: 170px; circle2.y: 00px; circle2.background: blue;
        }
    ]

    VerticalBox {
        HorizontalBox {
            max-height: self.min-height;
            b1 := Button {
                text: "State 1";
            }
            b2 := Button {
                text: "State 2";
            }
        }
        Rectangle {
            background: root.background.darker(20%);
            width: 200px;
            height: 100px;

            circle1 := Circle { y:0; background: green; x: 85px; }
            circle2 := Circle { background: green; x: 85px; y: 40px; }
        }
    }
}
```

### Transitions

```slint
import { HorizontalBox, VerticalBox, Button } from "std-widgets.slint";

component Circle inherits Rectangle {
    width: 30px;
    height: 30px;
    border-radius: root.width / 2;
}

export component Recipe inherits Window {
    states [
        left-aligned when b1.pressed: {
            circle1.x: 0px; circle1.y: 40px;
            circle2.x: 0px; circle2.y: 0px;
            in {
                animate circle1.x, circle2.x { duration: 250ms; }
            }
            out {
                animate circle1.x, circle2.x { duration: 500ms; }
            }
        }
        right-aligned when !b1.pressed: {
            circle1.x: 170px; circle1.y: 70px;
            circle2.x: 170px; circle2.y: 00px;
        }
    ]

    VerticalBox {
        HorizontalBox {
            max-height: self.min-height;
            b1 := Button {
                text: "Press and hold to change state";
            }
        }
        Rectangle {
            background: root.background.darker(20%);
            width: 250px;
            height: 100px;

            circle1 := Circle { y:0; background: green; x: 85px; }
            circle2 := Circle { background: blue; x: 85px; y: 40px; }
        }
    }
}
```

## Layout Examples

### Vertical

```slint
import { VerticalBox, Button } from "std-widgets.slint";
export component Recipe inherits Window {
    VerticalBox {
        Button { text: "First"; }
        Button { text: "Second"; }
        Button { text: "Third"; }
    }
}
```

### Horizontal

```slint
import { HorizontalBox, Button } from "std-widgets.slint";
export component Recipe inherits Window {
    HorizontalBox {
        Button { text: "First"; }
        Button { text: "Second"; }
        Button { text: "Third"; }
    }
}
```

### Grid

```slint
import { GridBox, Button, Slider } from "std-widgets.slint";
export component Recipe inherits Window {
    GridBox {
        Row {
            Button { text: "First"; }
            Button { text: "Second"; }
        }
        Row {
            Button { text: "Third"; }
            Button { text: "Fourth"; }
        }
        Row {
            Slider {
                colspan: 2;
            }
        }
    }
}
```

## Global Callbacks

### Invoke a Globally Registered Native Callback from Slint

This example uses a global singleton to implement common logic in native code. This singleton may also store properties that are accessible to native code.

Note: The preview visualize the Slint code only. It’s not connected to the native code.

```slint
import { HorizontalBox, VerticalBox, LineEdit } from "std-widgets.slint";

export global Logic  {
    pure callback to-upper-case(string) -> string;
    // You can collect other global properties here
}

export component Recipe inherits Window {
    VerticalBox {
        input := LineEdit {
            text: "Text to be transformed";
        }
        HorizontalBox {
            Text { text: "Transformed:"; }
            // Callback invoked in binding expression
            Text {
                text: {
                    Logic.to-upper-case(input.text);
                }
            }
        }
    }
}
```

**Rust**

In Rust you can set the callback like this:

```rust
fn main() {
    let recipe = Recipe::new().unwrap();
    recipe.global::<Logic>().on_to_upper_case(|string| {
        string.as_str().to_uppercase().into()
    });
    // ...
}
```

**C++**

C++ code

In C++ you can set the callback like this:

```cpp
int main(int argc, char **argv)
{
    auto recipe = Recipe::create();
    recipe->global<Logic>().on_to_upper_case([](slint::SharedString str) -> slint::SharedString {
        std::string arg(str);
        std::transform(arg.begin(), arg.end(), arg.begin(), toupper);
        return slint::SharedString(arg);
    });
    // ...
}
```

**NodeJS**

In JavaScript you can set the callback like this:

```js
let slint = require("slint-ui");
let file = slint.loadFile("recipe.slint");
let recipe = new file.Recipe();
recipe.Logic.to_upper_case = (str) => {
    return str.toUpperCase();
};
// ...
```

**Python**

In Python, the callback is associated with the `global_name` parameter of the `@slint.callback` decorator:

```python
import slint

class App(slint.loader.recipe.Recipe):
    @slint.callback(global_name="Logic")
    def to_upper_case(&self, value: str) -> str:
        return value.upper()

# ...
```

## Custom Widgets

### Custom Button

```slint
component Button inherits Rectangle {
    in-out property text <=> txt.text;
    callback clicked <=> touch.clicked;
    border-radius: root.height / 2;
    border-width: 1px;
    border-color: root.background.darker(25%);
    background: touch.pressed ? #6b8282 : touch.has-hover ? #6c616c :  #456;
    height: txt.preferred-height * 1.33;
    min-width: txt.preferred-width + 20px;
    txt := Text {
        x: (parent.width - self.width)/2 + (touch.pressed ? 2px : 0);
        y: (parent.height - self.height)/2 + (touch.pressed ? 1px : 0);
        color: touch.pressed ? #fff : #eee;
    }
    touch := TouchArea { }
}

export component Recipe inherits Window {
    VerticalLayout {
        alignment: start;
        Button { text: "Button"; }
    }
}
```

### ToggleSwitch

```slint
export component ToggleSwitch inherits Rectangle {
    callback toggled;
    in-out property <string> text;
    in-out property <bool> checked;
    in-out property<bool> enabled <=> touch-area.enabled;
    height: 20px;
    horizontal-stretch: 0;
    vertical-stretch: 0;

    HorizontalLayout {
        spacing: 8px;
        indicator := Rectangle {
            width: 40px;
            border-width: 1px;
            border-radius: root.height / 2;
            border-color: self.background.darker(25%);
            background: root.enabled ? (root.checked ? blue: white)  : white;
            animate background { duration: 100ms; }

            bubble := Rectangle {
                width: root.height - 8px;
                height: bubble.width;
                border-radius: bubble.height / 2;
                y: 4px;
                x: 4px + self.a * (indicator.width - bubble.width - 8px);
                property <float> a: root.checked ? 1 : 0;
                background: root.checked ? white : (root.enabled ? blue : gray);
                animate a, background { duration: 200ms; easing: ease;}
            }
        }

        Text {
            min-width: max(100px, self.preferred-width);
            text: root.text;
            vertical-alignment: center;
            color: root.enabled ? black : gray;
        }

    }

    touch-area := TouchArea {
        width: root.width;
        height: root.height;
        clicked => {
            if (root.enabled) {
                root.checked = !root.checked;
                root.toggled();
            }
        }
    }
}

export component Recipe inherits Window {
    VerticalLayout {
        alignment: start;
        ToggleSwitch { text: "Toggle me"; }
        ToggleSwitch { text: "Disabled"; enabled: false; }
    }
}
```

### CustomSlider

The `TouchArea` is covering the entire widget, so you can drag this slider from any point within itself.

```slint
import { VerticalBox } from "std-widgets.slint";

export component MySlider inherits Rectangle {
    in-out property<float> maximum: 100;
    in-out property<float> minimum: 0;
    in-out property<float> value;

    min-height: 24px;
    min-width: 100px;
    horizontal-stretch: 1;
    vertical-stretch: 0;

    border-radius: root.height/2;
    background: touch.pressed ? #eee: #ddd;
    border-width: 1px;
    border-color: root.background.darker(25%);

    handle := Rectangle {
        width: self.height;
        height: parent.height;
        border-width: 3px;
        border-radius: self.height / 2;
        background: touch.pressed ? #f8f: touch.has-hover ? #66f : #0000ff;
        border-color: self.background.darker(15%);
        x: (root.width - handle.width) * (root.value - root.minimum)/(root.maximum - root.minimum);
    }
    touch := TouchArea {
        property <float> pressed-value;
        pointer-event(event) => {
            if (event.button == PointerEventButton.left && event.kind == PointerEventKind.down) {
                self.pressed-value = root.value;
            }
        }
        moved => {
            if (self.enabled && self.pressed) {
                root.value = max(root.minimum, min(root.maximum,
                    self.pressed-value + (touch.mouse-x - touch.pressed-x) * (root.maximum - root.minimum) / (root.width - handle.width)));

            }
        }
    }
}

export component Recipe inherits Window {
    VerticalBox {
        alignment: start;
        slider := MySlider {
            maximum: 100;
        }
        Text {
            text: "Value: \{round(slider.value)}";
        }
    }
}
```

This example show another implementation that has a drag-able handle: The handle only moves when we click on that handle. The TouchArea is within the handle and moves with the handle.

```slint
import { VerticalBox } from "std-widgets.slint";

export component MySlider inherits Rectangle {
    in-out property<float> maximum: 100;
    in-out property<float> minimum: 0;
    in-out property<float> value;

    min-height: 24px;
    min-width: 100px;
    horizontal-stretch: 1;
    vertical-stretch: 0;

    border-radius: root.height/2;
    background: touch.pressed ? #eee: #ddd;
    border-width: 1px;
    border-color: root.background.darker(25%);

    handle := Rectangle {
        width: self.height;
        height: parent.height;
        border-width: 3px;
        border-radius: self.height / 2;
        background: touch.pressed ? #f8f: touch.has-hover ? #66f : #0000ff;
        border-color: self.background.darker(15%);
        x: (root.width - handle.width) * (root.value - root.minimum)/(root.maximum - root.minimum);

        touch := TouchArea {
            moved => {
                if (self.enabled && self.pressed) {
                    root.value = max(root.minimum, min(root.maximum,
                        root.value + (self.mouse-x - self.pressed-x) * (root.maximum - root.minimum) / root.width));
                }
            }
        }
    }
}

export component Recipe inherits Window {
    VerticalBox {
        alignment: start;
        slider := MySlider {
            maximum: 100;
        }
        Text {
            text: "Value: \{round(slider.value)}";
        }
    }
}
```

### Custom Tabs

Use this recipe as a basis to when you want to create your own custom tab widget.

```slint
import { Button } from "std-widgets.slint";

export component Recipe inherits Window {
    preferred-height: 200px;
    in-out property <int> active-tab;
    VerticalLayout {
        tab_bar := HorizontalLayout {
            spacing: 3px;
            Button {
                text: "Red";
                clicked => { root.active-tab = 0; }
            }
            Button {
                text: "Blue";
                clicked => { root.active-tab = 1; }
            }
            Button {
                text: "Green";
                clicked => { root.active-tab = 2; }
            }
        }
        Rectangle {
            clip: true;
            Rectangle {
                background: red;
                x: root.active-tab == 0 ? 0 : root.active-tab < 0 ? - self.width - 1px : parent.width + 1px;
                animate x { duration: 125ms; easing: ease; }
            }
            Rectangle {
                background: blue;
                x: root.active-tab == 1 ? 0 : root.active-tab < 1 ? - self.width - 1px : parent.width + 1px;
                animate x { duration: 125ms; easing: ease; }
            }
            Rectangle {
                background: green;
                x: root.active-tab == 2 ? 0 : root.active-tab < 2 ? - self.width - 1px : parent.width + 1px;
                animate x { duration: 125ms; easing: ease; }
            }
        }
    }
}
```

### Custom Table View

Slint provides a table widget, but you can also do something custom based on a `ListView`.

```slint
import { VerticalBox, ListView } from "std-widgets.slint";

component TableView inherits Rectangle {
    in property <[string]> columns;
    in property <[[string]]> values;

    private property <length> e: self.width / root.columns.length;
    private property <[length]> column_sizes: [
        root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e,
        root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e,
        root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e, root.e,
    ];

    VerticalBox {
        padding: 5px;
        HorizontalLayout {
            padding: 5px; spacing: 5px;
            vertical-stretch: 0;
            for title[idx] in root.columns : HorizontalLayout {
                width: root.column_sizes[idx];
                Text { overflow: elide; text: title; }
                Rectangle {
                    width: 1px;
                    background: gray;
                    TouchArea {
                        width: 10px;
                        x: (parent.width - self.width) / 2;
                        property <length> cached;
                        pointer-event(event) => {
                            if (event.button == PointerEventButton.left && event.kind == PointerEventKind.down) {
                                self.cached = root.column_sizes[idx];
                            }
                        }
                        moved => {
                            if (self.pressed) {
                                root.column_sizes[idx] += (self.mouse-x - self.pressed-x);
                                if (root.column_sizes[idx] < 0) {
                                    root.column_sizes[idx] = 0;
                                }
                            }
                        }
                        mouse-cursor: ew-resize;
                    }
                }
            }
        }
        ListView {
            for r in root.values : HorizontalLayout {
                padding: 5px;
                spacing: 5px;
                for t[idx] in r : HorizontalLayout {
                    width: root.column_sizes[idx];
                    Text { overflow: elide; text: t; }
                }
            }
        }
    }
}

export component Example inherits Window {
   TableView {
       columns: ["Device", "Mount Point", "Total", "Free"];
       values: [
            ["/dev/sda1", "/", "255GB", "82.2GB"] ,
            ["/dev/sda2", "/tmp", "60.5GB", "44.5GB"] ,
            ["/dev/sdb1", "/home", "255GB", "32.2GB"] ,
       ];
   }
}
```

### Breakpoints for Responsive User Interfaces

Use recipe implements a responsive SideBar that collapses when the parent width is smaller than the given break-point. When clicking the Button, the SideBar expands again. Use the blue Splitter to resize the container and test the responsive behavior.

```slint
import { Button, Palette } from "std-widgets.slint";

export component SideBar inherits Rectangle {
    private property <bool> collapsed: root.reference-width < root.break-point;

    /// Defines the reference width to check `break-point`.
    in-out property <length> reference-width;

    /// If `reference-width` is less `break-point` the `SideBar` collapses.
    in-out property <length> break-point: 600px;

    /// Set the text of the expand button.
    in-out property <string> expand-button-text;

    width: 160px;

    container := Rectangle {
        private property <bool> expanded;

        width: parent.width;
        background: Palette.background.darker(0.2);

        VerticalLayout {
            padding: 2px;
            alignment: start;

            HorizontalLayout {
                alignment: start;

                if (root.collapsed) : Button {
                    checked: container.expanded;
                    text: root.expand-button-text;

                    clicked => {
                        container.expanded = !container.expanded;
                    }
                }
            }

            @children
        }

        states [
            expanded when container.expanded && root.collapsed : {
                width: 160px;

                in {
                    animate width { duration: 200ms; }
                }
                out {
                    animate width { duration: 200ms; }
                }
                in {
                        animate width { duration: 200ms; }
                }
                out {
                        animate width { duration: 200ms; }
                }
            }
        ]
    }

    states [
        collapsed when root.collapsed : {
            width: 62px;
        }
    ]
}

component Splitter inherits TouchArea {
    width: 4px;
    mouse-cursor: ew-resize;

    Rectangle {
        width: 100%;
        height: 100%;
        background: blue;
    }
}

export component SideBarTest inherits Window {
    preferred-width: 700px;
    min-height: 400px;
    background: gray;

    GridLayout {
        x: 0;
        width: splitter.x;

        Rectangle {
            height: 100%;
            col: 1;
            background: white;

            HorizontalLayout {
                padding: 8px;

                Text {
                    color: black;
                    text: "Content";
                }
            }
        }
        SideBar {
            col: 0;
            reference-width: parent.width;
            expand-button-text: "E";
        }
    }

    splitter := Splitter {
        x: root.width - self.width;
        height: 100%;

        moved => {
            self.x = min(root.width - self.width, max(400px, self.x + self.mouse-x - self.pressed-x));
        }
    }
}
```

## Debugging Techniques

Source: `guide/development/debugging_techniques/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/debugging_techniques/

On this page we share different techniques and tools we’ve built into Slint that help you track down different issues you may be running into, during the design and development.

### Debugging Property Values

Use the [debug()](https://docs.slint.dev/latest/docs/slint/reference/global-functions/builtinfunctions/index.html#debug) function to print the values of properties to stderr. It supports multiple arguments and different types. Non-scalar types (e.g. `struct` types) are usually represented as a single line `json` string.

**Example**

### Slow Motion Animations

Animations in the user interface need to be carefully designed to have the correct duration and changes in element positioning or size need to follow an easing curve.

To inspect the animations in your application, set the `SLINT_SLOW_ANIMATIONS` environment variable before running the program. This variable accepts an unsigned integer value that is the factor by which to globally slow down the steps of all animations, automatically. This means that you don’t have to make any manual changes to the `.slint` markup and recompile. For example,`SLINT_SLOW_ANIMATIONS=4` slows down animations by a factor of four.

### User Interface Scaling

The use of logical pixel lengths throughout `.slint` files lets Slint compute the number of physical pixels, dynamically, depending on the device-pixel ratio of the screen. To get an impression of how the individual elements look like when rendered on a screen with a different device-pixel ratio, set the `SLINT_SCALE_FACTOR` environment variable before running the program. This variable accepts a floating pointer number that is used to convert logical pixel lengths to physical pixel lengths. For example, `SLINT_SCALE_FACTOR=2` renders the user interface in a way where every logical pixel has twice the width and height.

*Note*: Currently, only the FemtoVG and Skia renderers support this environment variable.

### Debugging for Performance Improvements

Slint attempts to use hardware-acceleration to ensure that rendering the user interface consumes a minimal amount of CPU resources while maintaining smooth animations. However, depending on the complexity of the user interface, quality of the graphics drivers, or the power of the GPU in your system, you may hit limits and experience slowness. To address this issue, set the `SLINT_DEBUG_PERFORMANCE` environment variable before running the program, to inspect the frame rate. The following options affect the frame rate inspection and reporting:

- `refresh_lazy` : The frame rate is measured only when an actual frame is rendered, for example due to a running animation, user interaction, or some other state change that results in a visual difference in the user interface. If there is no change, a low frame rate is reported. Use this option to verify that no unnecessary repainting happens when there are no visual changes. For example, in a user interface that shows a text input field with a cursor that blinks once per second, the reported frame rate should be two.
- `refresh_full_speed` : The user interface is continuously refreshed, even if nothing is changed. This continuous refresh results in a higher load on the system. Use this option to identify any bottlenecks that prevent you from achieving smooth animations. Also disables partial rendering with the software renderer.
- `console` : The frame rate is printed to `stderr` on the console.
- `overlay` : The frame rate is as an overlay text label on top of the user interface in each window.

Use these options in combination, separated by a comma. You must select a combination of one frame rate measurement method and a reporting method. For example, `SLINT_DEBUG_PERFORMANCE=refresh_full_speed,overlay` repeatedly re-renders the entire user interface in each window and prints the achieved frame rate in the top-left corner. In comparison, `SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay` measures the frame rate only when something in the user interface changes and the measured value is printed to `stderr` as well as rendered as an overlay text label.

The environment variable must be set before running the program. If the application runs on a microcontroller without the standard library, the environment variable must be set during compilation.

### Tuning Rendering Performance

If you’re not satisfied with the performance, it might be worthwhile to descend into a low-level investigation. Tools such as [RenderDoc↗](https://renderdoc.org/) permit recording the rendering output of your application and can give you detailed insight into the OpenGL/Vulkan commands Slint’s renderers produce for your user interface.

As a general rule of thumb, it’s best to minimize the number of commands per frame. With OpenGL you’ll see `glDrawArrays()` and `glDrawElementsInstanced*` calls filling the color buffers. Reducing the number of calls tends to improve performance.

For example, if you draw a series of bar charts by filling rectangles with a gradient, you’ll observe that each rectangle is a draw call on its own:

```slint
component BarChart inherits Rectangle {
    background: black;
    height: 150px;
    HorizontalLayout {
        spacing: 10px;
        for i in 5: Rectangle {
            height: 10px + i * 20px;
            width: 20px;
            background: @linear-gradient(180deg, #f00 0%, #0f0);
        }
    }
}
```

For example when using the Skia renderer, if you replace the gradient with a plain color, the calls will all be batched together into one call. But when the UI design requires the use of a gradient, that’s not possible. But there might be another way. If the underlying data for your bar chart rarely changes, it might be worthwhile to render the entire chart once into a texture and therefore replace multiple calls with just one. This can be done by setting the [cache-rendering-hint](https://docs.slint.dev/latest/docs/slint/reference/common/index.html#cache-rendering-hint) to `true` on the `BarChar` itself, which surrounds and captures the individual bar charts.

Note that extensive use of this technique comes at the expense of increased GPU memory usage and memory throughput.

## Focus Handling

Source: `guide/development/focus/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/focus/

Certain elements such as `TextInput` accept input from the mouse/finger and also key events originating from (virtual) keyboards. In order for an item to receive these events, it must have focus. This is visible through the `has-focus` (out) property.

You can manually activate the focus on an element by calling `focus()`:

Similarly, you can manually clear the focus on an element that’s currently focused, by calling `clear-focus()`:

```slint
import { Button } from "std-widgets.slint";

export component App inherits Window {
    VerticalLayout {
        alignment: start;
        Button {
            text: "press me";
            clicked => { input.clear-focus(); }
        }
        input := TextInput {
            text: "I am a text input field";
        }
    }
}
```

After clearing the focus, keyboard input to the window is discarded until another element is explicitly focused. For example by calling `focus()`, an element acquiring focus when the user clicks on it, or when pressing tab and the first focusable element is found.

If you have wrapped the `TextInput` in a component, then you can forward such a focus activation using the `forward-focus` property to refer to the element that should receive it:

```slint
import { Button } from "std-widgets.slint";

component LabeledInput inherits GridLayout {
    forward-focus: input;
    Row {
        Text {
            text: "Input Label:";
        }
        input := TextInput {}
    }
}

export component App inherits Window {
    GridLayout {
        Button {
            text: "press me";
            clicked => { label.focus(); }
        }
        label := LabeledInput {
        }
    }
}
```

If you use the `forward-focus` property on a `Window` or a `PopupWindow`, then the specified element will receive the focus the first time the window receives the focus - it becomes the initial focus element.

## Font Handling

Source: `guide/development/fonts/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/fonts/

Elements such as `Text` and `TextInput` can render text and allow customizing the appearance of the text through different properties. The properties prefixed with `font-`, such as `font-family`, `font-size` and `font-weight` affect the choice of font used for rendering to the screen. If any of these properties isn’t specified, the `default-font-` values in the surrounding `Window` element apply, such as `default-font-family`.

The fonts chosen for rendering are automatically picked up from the system running the application. It’s also possible to include custom fonts in your design. A custom font must be a TrueType font (`.ttf`), a TrueType font collection (`.ttc`) or an OpenType font (`.otf`). You can select a custom font with the `import` statement: `import "./my_custom_font.ttf"` in a .slint file. This instructs the Slint compiler to include the font and makes the font families globally available for use with `font-family` properties.

For example:

## Third Party Libraries

Source: `guide/development/third-party-libraries/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/third-party-libraries/

This page is a selection of third party libraries and addons that can be useful to develop with Slint.

If you have a library that you think should be listed here, please open let us know on our [Bug tracker↗](https://github.com/slint-ui/slint/issues), or open a pull request for [this page↗](https://github.com/slint-ui/slint/blob/master/docs/astro/src/content/docs/guide/development/third-party-libraries.mdx).

### Component Sets

#### Material

The [Material component set↗](https://material.slint.dev/) is a set of components that are based on the [Material Design 3 specification↗](https://m3.material.io/).

It is a set of components that can be used to build a Material Design 3 application.

The component set is developed in the Slint repository in [this folder↗](https://github.com/slint-ui/slint/tree/master/ui-libraries/material).

[https://material.slint.dev/↗](https://material.slint.dev/)

#### SurrealismUI

[SurrealismUI↗](https://github.com/Surrealism-All/SurrealismUI) is a third-party UI library using Slint

[https://github.com/Surrealism-All/SurrealismUI↗](https://github.com/Surrealism-All/SurrealismUI)

#### Vivi

[Vivi↗](https://app.radicle.xyz/nodes/seed.radicle.garden/rad:z3oxAZSLcyXgpa7fcvgtueF49jHpH) is a third-party custom component library for Slint. The goal of this project is to provide a full set of components that can be used to create user interfaces from small apps to complex desktop applications.

[https://app.radicle.xyz/nodes/seed.radicle.garden/rad:z3oxAZSLcyXgpa7fcvgtueF49jHpH↗](https://app.radicle.xyz/nodes/seed.radicle.garden/rad:z3oxAZSLcyXgpa7fcvgtueF49jHpH)

#### Sleek-ui

[Sleek-ui↗](https://github.com/uAtomicBoolean/sleek-ui) is a third-party UI components library built with/for Slint based on [ant design↗](https://ant.design/).

[https://github.com/uAtomicBoolean/sleek-ui/↗](https://github.com/uAtomicBoolean/sleek-ui/)

### Templates

#### Heng30’s Slint Template

It’s a Rust template project for Slint GUI. It contains frequently-used components, setting panel, configure, simple database feature and other small feature. This project can be compiled to Desktop (Windows, Linux, Macos), Android and Web platform.

[https://github.com/heng30/slint-template↗](https://github.com/heng30/slint-template)

## Translations

Source: `guide/development/translations/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/development/translations/

Slint’s translation infrastructure makes your application available in different languages.

> **Prerequisite**
> Install the [`slint-tr-extractor`↗](https://crates.io/crates/slint-tr-extractor) tool to extract translatable strings from `.slint` files:

You can choose between runtime translations using `gettext` or bundling translations directly into your executable for platforms where `gettext` is unavailable or impractical.

To translate your application, follow these steps:

1. Identify all user-visible strings that need translation and annotate them with the `@tr()` macro.
2. Extract annotated strings by running the `slint-tr-extractor` tool to generate `.pot` files.
3. Use a third-party tool to translate the strings into `.po` files for each target language.
4. **(For runtime `gettext` translations only)** Convert `.po` files into `.mo` files using [gettext’s `msgfmt`↗](https://www.gnu.org/software/gettext/manual/gettext.html) .
5. **(For bundled translations only)** Configure bundling during the build process to embed translations into your application.
6. Use Slint’s API to select the appropriate translation based on the user’s locale.

At this point, all strings marked for translation will be automatically rendered in the selected language.

### Annotating Translatable Strings

Use the `@tr` macro in `.slint` files to mark strings for translation. This macro supports formatting and pluralization, and can include contextual information.

The first argument must be a plain string literal, followed by the arguments:

#### Basic Example

```slint
export component Example {
    property <string> name;
    Text {
        text: @tr("Hello, {}", name);
    }
}
```

#### Formatting

The `@tr` macro replaces each `{}` placeholder in the string marked for translation with the corresponding argument. It’s also possible to re-order the arguments using `{0}`, `{1}`, and so on. Translators can use ordered placeholders even if the original string did not.

You can include the literal characters `{` and `}` in a string by preceding them with the same character. For example, escaping the `{` character with `{{` and the `}` character with `}}`.

#### Plurals

Use plural formatting when the translation of text involving a variable number of elements should change depending on whether there is a single element or multiple.

Given `count` and an expression that represents the count of something, form the plural with the `|` and `%` symbols like so:

`@tr("I have {n} item" | "I have {n} items" % count)`.

Use `{n}` in the format string to access the expression after the `%`.

```slint
export component Example inherits Text {
    in property <int> score;
    in property <int> name;
    text: @tr("Hello {0}, you have one point" | "Hello {0}, you have {n} points" % score, name);
}
```

#### Context

Disambiguate translations for strings with the same source text but different contextual meanings by adding a context to the `@tr(...)` macro using the `"..." =>` syntax.

Use the context to provide additional context information to translators, ensuring accurate and contextually appropriate translations.

The context must be a plain string literal and it appears as `msgctx` in the `.pot` files. If not specified, the context defaults to the name of the surrounding component.

```slint
export component MenuItem {
    property <string> name : @tr("Default Name"); // Default: `MenuItem` will be the context.
    property <string> tooltip : @tr("ToolTip" => "ToolTip for {}", name); // Specified: The context will be `ToolTip`.
}
```

Pass the `--no-default-translation-context` flag to `slint-tr-extractor` if you don’t want the component name to be the default context. The same option needs to be passed to the Slint compiler:

**C++**

Set the [`SLINT_NO_DEFAULT_TRANSLATION_CONTEXT`](https://docs.slint.dev/latest/docs/cpp/cmake_reference.html#bundle-translations) target property on your CMake target.

**Rust**

With [`slint_build::CompilerConfiguration::set_default_translation_context(slint_build::DefaultTranslationContext::None)`](https://docs.slint.dev/latest/docs/rust/slint_build/struct.compilerconfiguration#method.set_default_translation_context) when using a build script. With [`slint_interpreter::Compiler::set_default_translation_context(slint_interpreter::DefaultTranslationContext::None)`](https://docs.slint.dev/latest/docs/rust/slint_interpreter/struct.compiler#method.set_default_translation_context) when using the `slint-interpreter` crate.

### Extracting Translatable Strings

Use [`slint-tr-extractor`↗](https://crates.io/crates/slint-tr-extractor) to generate a `.pot` file with all strings marked for translation:

```sh
find -name \*.slint | xargs slint-tr-extractor -o MY_PROJECT.pot
```

This creates a file called `MY_PROJECT.pot`. Replace “MY_PROJECT” with your actual project name. To learn how the project name affects the lookup of translations, read the sections below.

> **Tip**
> `.pot` files are [Gettext↗](https://www.gnu.org/software/gettext/) template files.

### Translating Strings

Start a new translation by creating a `.po` file from a `.pot` file. Both file formats are identical. You can either copy the file manually or use a tool like Gettext’s `msginit` to start a new `.po` file.

The `.po` file contains the strings in a target language.

`.po` and `.pot` files are plain text files that you can edit with a text editor. We recommend using a dedicated translation tool for working with them, such as the following:

- [poedit↗](https://poedit.net/)
- [OmegaT↗](https://omegat.org/)
- [Lokalize↗](https://userbase.kde.org/Lokalize)
- [Transifex↗](https://www.transifex.com/) (web interface)

### Runtime Translations with Gettext

Slint can use the [Gettext↗](https://www.gnu.org/software/gettext/) library to load translations at run-time.

Gettext expects translation files - called message catalogs - in following directory hierarchy:

- **dir_name/** - **locale/e.g.fr,en,de, etc** - **LC_MESSAGES/** - domain_name.mo

- `dir_name`: the base directory that you can choose freely.
- `locale`: The name of the user’s locale for a given target language, such as `fr` for French, or `de` for German. The locale is typically determined using environment variables that your operating system sets.
- `domain_name`: Selected based on the programming language you’re using Slint with.

> **Tip**
> Read the [Gettext documentation↗](https://www.gnu.org/software/gettext/manual/gettext.html#Locating-Catalogs) for more information.

#### Convert `.po` Files to `.mo` Files

Convert the human readable `.po` files into machine-friendly `.mo` files, which are a binary representation that is more efficient to read by code.

Use [Gettext↗](https://www.gnu.org/software/gettext/)’s `msgfmt` command line tool to convert `.po` files to `.mo` files:

```sh
msgfmt translation.po -o translation.mo
```

#### Select and Load Translations

**C++**

First, enable the `SLINT_FEATURE_GETTEXT` cmake option when compiling Slint to gain access to the translations API and activate run-time translation support.

In C++ applications using cmake, the `domain_name` is the CMake target name.

Next, bind the text domain to a path using the standard gettext library.

To do so, add this in your `CMakeLists.txt` file:

```cmake
find_package(Intl)
if(Intl_FOUND)
    target_compile_definitions(my_application PRIVATE HAVE_GETTEXT SRC_DIR="${CMAKE_CURRENT_SOURCE_DIR}")
    target_link_libraries(my_application PRIVATE Intl::Intl)
endif()
```

You can then setup the locale and the text domain

```c++
#ifdef HAVE_GETTEXT
#    include <locale>
#    include <libintl.h>
#endif

int main()
{
#ifdef HAVE_GETTEXT
    bindtextdomain("my_application", SRC_DIR "/lang/");
    std::locale::global(std::locale(""));
#endif
   //...
}
```

For example, if you’re using the above and the user’s locale is `fr`, Slint looks for `my_application.mo` in the `lang/fr/LC_MESSAGES/` directory.

**Rust**

First, enable the `gettext` feature of the `slint` crate in the `features` section to gain access to the translations API and activate run-time translation support.

Next, use the [`slint::init_translations!`](https://docs.slint.dev/latest/docs/rust/slint/macro.init_translations) macro to specify the base location of your `.mo` files. This is the `dir_name` in the scheme of the previous section. Slint expects the `.mo` files to be in the corresponding sub-directories and their file name - `domain_name` - must match the package name in your `Cargo.toml`. This is often the same as the crate name.

For example:

```rust
slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
```

For example, if your `Cargo.toml` contains the following lines and the user’s locale is `fr`:

```toml
[package]
name = "gallery"
```

With these settings, Slint looks for `gallery.mo` in the `lang/fr/LC_MESSAGES/gallery.mo`.

### Bundled Translations

Bundled translations embed the translated strings directly into your application binary. This approach is ideal for platforms like WASM or microcontrollers where `gettext` is unavailable.

Configure the Slint compiler to bundle translations by providing a path to the translations. Translation files should be organized in the following hierarchy:

```plaintext
path/<lang>/LC_MESSAGES/<domain>.po
```

#### Bundling

**C++**

Set the [`SLINT_BUNDLE_TRANSLATIONS`](https://docs.slint.dev/latest/docs/cpp/cmake_reference.html#bundle-translations) property in CMake:

```cmake
set_property(TARGET my_application PROPERTY SLINT_BUNDLE_TRANSLATIONS "${CMAKE_CURRENT_SOURCE_DIR}/lang")
```

the `<domain>` is the cmake target name.

**Rust**

Use `slint_build::CompilerConfiguration`’s [`with_bundled_translations()`](https://docs.slint.dev/latest/docs/rust/slint_build/struct.compilerconfiguration#method.with_bundled_translations) function to set up bundling in `build.rs`:

```rust
let config = slint_build::CompilerConfiguration::new()
    .with_bundled_translations("path/to/translations");
slint_build::compile_with_config("path/to/main-ui.slint", config).unwrap();
```

The `<domain>` is the crate name.

#### Selecting a Translation

If you enable the `std` feature with Slint, language for translations is detected based on the locale: if one of the bundled language matches the selected locale, it will be used.

**C++**

Use the [`slint::select_bundled_translation`](https://docs.slint.dev/latest/docs/cpp/api/function_namespaceslint_1a18aba736373254f5be3362941f3ddbcd.html#_CPPv4N5slint26select_bundled_translationENSt11string_viewE) function to change translations at runtime.

**Rust**

Use the [`slint::select_bundled_translation`](https://docs.slint.dev/latest/docs/rust/slint/fn.select_bundled_translation.html) function to change translations at runtime.

**Python**

Use the [`slint.init_translations()`](https://docs.slint.dev/latest/docs/python/slint#init_translations) function to change translations at runtime.

### Previewing Translations with `slint-viewer`

Make sure the `gettext` feature was enabled when building slint-viewer. Use the `--translation-domain` and `--translation-dir` command line options to load translations for preview.

## Animations

Source: `guide/language/coding/animation/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/animation/

Declare animations for properties with the `animate` keyword like this:

This will animate the color property for 250ms whenever it changes.

It’s also possible to animate several properties with the same animation, so:

```slint
animate x, y { duration: 100ms; easing: ease-out-bounce; }
```

is the same as:

```slint
animate x { duration: 100ms; easing: ease-out-bounce; }
animate y { duration: 100ms; easing: ease-out-bounce; }
```

Fine-tune animations using the following parameters:

#### delay

[duration](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#duration) default: `0ms`

The amount of time to wait before starting the animation.

#### duration

[duration](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#duration) default: `0ms`

The amount of time it takes for the animation to complete.

#### iteration-count

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The number of times an animation should run. A negative value specifies infinite reruns. Fractional values are possible. For permanently running animations, see [`animation-tick()`](https://docs.slint.dev/latest/docs/slint/reference/global-functions/builtinfunctions/index.html#animation-tick---duration).

#### easing

[easing](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#easing) default: `linear`

Can be any of the following. See [`easings.net`↗](https://easings.net/) for a visual reference:

#### direction

[enum AnimationDirection](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#animationdirection) default: `the first enum value`

Use this to set or change the direction of the animation.

`AnimationDirection`

This enum describes the direction of an animation.

- **`normal`** : The [“normal” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#normal) .
- **`reverse`** : The [“reverse” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#reverse) .
- **`alternate`** : The [“alternate” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#alternate) .
- **`alternate-reverse`** : The [“alternate reverse” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#alternate-reverse) .

## Expressions

Source: `guide/language/coding/expressions-and-statements/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/expressions-and-statements/

Expressions are a powerful way to declare relationships and connections in your user interface. They´re typically used to combine basic arithmetic with access to properties of other elements. When these properties change, the expression is automatically re-evaluated and a new value is assigned to the property the expression is associated with:

When `my-property` changes, the width changes automatically, too.

Arithmetic in expression with numbers works like in most programming languages with the operators `*`, `+`, `-`, `/`:

```slint
export component Example {
    in-out property <int> p: 1 * 2 + 3 * 4; // same as (1 * 2) + (3 * 4)
}
```

Concatenate strings with `+`.

The operators `&&` and `||` express logical *and* and *or* between boolean values. The operators `==`, `!=`, `>`, `<`, `>=` and `<=` compare values of the same type.

Access an element’s properties by using its name, followed by a `.` and the property name:

```slint
export component Example {
    foo := Rectangle {
        x: 42px;
    }
    x: foo.x;
}
```

The ternary operator `... ? ... : ...` is also supported, like in C or JavaScript:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;

    Rectangle {
        touch := TouchArea {}
        background: touch.pressed ? #111 : #eee;
        border-width: 5px;
        border-color: !touch.enabled ? #888
            : touch.pressed ? #aaa
            : #555;
    }
}
```

### Statements

#### Let statements (local variables)

The `let` keyword can be used to create local variables. Local variables are immutable and cannot be redeclared (even in other scopes). They optionally have a type annotation.

```slint
clicked => {
    let foo = "hello world"; // no type annotation, inferred type
    debug(foo); // prints "hello world"

    let bar: int = 2; // explicit type annotation
    debug(bar); // prints "2"
}
```

#### Assignment

```slint
clicked => { some-property = 42; }
```

#### Self-assignment with `+=` `-=` `*=` `/=`

```slint
clicked => { some-property += 42; }
```

#### Calling a callback

```slint
clicked => { root.some-callback(); }
```

#### Conditional statements

```slint
clicked => {
    if (condition) {
        foo = 42;
    } else if (other-condition) {
        bar = 28;
    } else {
        foo = 4;
    }
}
```

#### Empty expression

```slint
clicked => { }
// or
clicked => { ; }
```

## The `.slint` File

Source: `guide/language/coding/file/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/

You write user interfaces in the Slint language and saved in files with the `.slint` extension.

Each `.slint` file defines one or several components. These components declare a tree of elements. Components form the basis of composition in Slint. Use them to build your own re-usable set of UI controls. You can use each declared component under its name as an element in another component.

Below is an example of components and elements:

Both `MyButton` and `MyApp` are components. `Window` and `Rectangle` are built-in elements used by `MyApp`. `MyApp` also re-uses the `MyButton` component as two separate elements.

Elements have properties, which you can assign values to. The example above assigns a string constant “hello” to the first `MyButton`’s `text` property. You can also assign entire expressions. Slint re-evaluates the expressions when any of the properties they depend on change, which makes the user-interface reactive.

You can name elements using the `:=` syntax:

```slint
component MyButton inherits Text {
    // ...
}

export component MyApp inherits Window {
    preferred-width: 200px;
    preferred-height: 100px;

    hello := MyButton {
        x:0;y:0;
        text: "hello";
    }
    world := MyButton {
        y:0;
        text: "world";
        x: 50px;
    }
}
```

> **Note**
> Names have to be valid [identifiers](https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/#identifiers).

Some elements are also accessible under pre-defined names:

- `root` refers to the outermost element of a component.
- `self` refers to the current element.
- `parent` refers to the parent element of the current element.

These names are reserved and you can’t re-define them.

### Comments

Comments are lines of code that are ignored by the Slint compiler. They are used to explain the code or to temporarily disable code.

#### Single Line Comments

Single line comments are denoted by `//` and are terminated by a new line.

```slint
// Amazing text! This is a comment
```

#### Multi Line Comments

Multi line comments are denoted by `/*` and `*/` and are terminated by a new line.

```slint
/*
    This is a multi line comment.
    It can span multiple lines.
*/
```

### Elements and Components

The core part of the Slint language are elements and components. Technically they are the same thing so once you know how to declare and use one you know the other. Elements are the basic building blocks that are part of the Slint Language, while components (also know as widgets) are larger items that are built up from multiple elements and properties.

If you have come from other languages such as HTML or React you might be used to opening and closing tags as well as self closing tags.

```html
<!-- opening and closing tag -->
<Button>Hello World</Button>
 <!-- self closing tag -->
<img/>
```

Slint simply has a single way to declare an item the `element-name` followed by a set of curly braces `{}` that contain the properties of the element.

```slint
// valid
Text {}

Text {
}
// Valid, but considered bad Slint practice
Text
{
}

// Not valid due to terminating semicolon
Text {};
```

> **Note**
> The Slint tooling provides a code formatter that enforces what is considered good practice.
>
> If you are new to coding then you can make friends with fellow developers by discussing aspects of code formatting you don’t like. It’s a type of small talk developers love and appreciate.

### The Root Element

```slint
component MyApp {
    Text {
        text: "Hello World";
        font-size: 24px;
    }
}
```

To be a valid Slint file the root element must be a component. Then inside the component you can declare any number of elements. This is explained in more detail later, it’s not important to understand at this point.

### Properties

Properties are the values that are assigned to an element. They are set using the `property-name: value;` syntax.

### Identifiers

Identifiers can be composed of letter (`a-zA-Z`), of numbers (`0-9`), or of the underscore (`_`) or the dash (`-`). They can’t start with a number or a dash (but they can start with underscore) The underscores are normalized to dashes. Which means that these two identifiers are the same: `foo_bar` and `foo-bar`.

### Conditional Elements

The `if` construct instantiates an element only if a given condition is true. The syntax is `if condition : id := Element { ... }`

```slint
export component Example inherits Window {
    preferred-width: 50px;
    preferred-height: 50px;
    if area.pressed : foo := Rectangle { background: blue; }
    if !area.pressed : Rectangle { background: red; }
    area := TouchArea {}
}
```

### Modules

Components declared in a `.slint` file can be used as elements in other `.slint` files, by means of exporting and importing them.

By default, every type declared in a `.slint` file is private. The `export` keyword changes this.

```slint
component ButtonHelper inherits Rectangle {
    // ...
}

component Button inherits Rectangle {
    // ...
    ButtonHelper {
        // ...
    }
}

export { Button }
```

In the above example, `Button` is accessible from other `.slint` files, but `ButtonHelper` isn’t.

It’s also possible to change the name just for the purpose of exporting, without affecting its internal use:

```slint
component Button inherits Rectangle {
    // ...
}

export { Button as ColorButton }
```

In the above example, `Button` isn’t accessible from the outside, but is available under the name `ColorButton` instead.

For convenience, a third way of exporting a component is to declare it exported right away:

```slint
export component Button inherits Rectangle {
    // ...
}
```

Similarly, components exported from other files may be imported:

```slint
import { Button } from "./button.slint";

export component App inherits Rectangle {
    // ...
    Button {
        // ...
    }
}
```

In the event that two files export a type under the same name, then you have the option of assigning a different name at import time:

```slint
import { Button } from "./button.slint";
import { Button as CoolButton } from "../other_theme/button.slint";

export component App inherits Rectangle {
    // ...
    CoolButton {} // from other_theme/button.slint
    Button {} // from button.slint
}
```

Elements, globals and structs can be exported and imported.

It’s also possible to export globals (see [Global Singletons](https://docs.slint.dev/latest/docs/slint/guide/language/coding/globals/index.html)) imported from other files:

```slint
import { Logic as MathLogic } from "math.slint";
export { MathLogic } // known as "MathLogic" when using native APIs to access globals
```

### Module Syntax

The following syntax is supported for importing types:

```slint
import { MyButton } from "module.slint";
import { MyButton, MySwitch } from "module.slint";
import { MyButton as OtherButton } from "module.slint";
import {
    MyButton,
    /* ... */,
    MySwitch as OtherSwitch,
} from "module.slint";
```

The following syntax is supported for exporting types:

```slint
// Export declarations
export component MyButton inherits Rectangle { /* ... */ }

// Export lists
component MySwitch inherits Rectangle { /* ... */ }
export { MySwitch }
export { MySwitch as Alias1, MyButton as Alias2 }

// Re-export types from other module
export { MyCheckBox, MyButton as OtherButton } from "other_module.slint";

// Re-export all types from other module (only possible once per file)
export * from "other_module.slint";
```

### Component Libraries

Splitting your code base into separate module files promotes re-use and improves encapsulation by allow you to hide helper components. This works well within a project’s own directory structure. To share libraries of components between projects without hardcoding their relative paths, use the component library syntax:

```slint
import { MySwitch } from "@mylibrary/switch.slint";
import { MyButton } from "@otherlibrary";
```

In the above example, the `MySwitch` component will be imported from a component library called `mylibrary`, in which Slint looks for the `switch.slint` file. Therefore `mylibrary` must be declared to refer to a directory, so that the subsequent search for `switch.slint` succeeds. `MyButton` will be imported from `otherlibrary`. Therefore `otherlibrary` must be declared to refer to a `.slint` file that exports `MyButton`.

The path to each library, as file or directory, must be defined separately at compilation time. Use one of the following methods to help the Slint compiler resolve libraries to the correct path on disk:

**C++**

- Specify `LIBRARY_PATHS` with [`slint_target_sources`](https://docs.slint.dev/latest/docs/cpp/cmake_reference#slint-target-sources) . For example:

```cmake
slint_target_sources(my_application
    ui/main.slint
    LIBRARY_PATHS
        material=${CMAKE_CURRENT_SOURCE_DIR}/material-1.0/material.slint
)
```

**Rust**

- In `build.rs` , call [`with_library_paths`](https://docs.slint.dev/latest/docs/rust/slint_build/struct.CompilerConfiguration#method.with_library_paths) to provide a mapping from library name to path. For example:

```rust
// build.rs
fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_paths = std::collections::HashMap::from([(
        "example".to_string(),
        manifest_dir.join("third_party/example/ui/lib.slint"),
    )]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
```

**NodeJS**

- Provide the `libraryPaths` map with [`loadFile`](https://docs.slint.dev/latest/docs/node/functions/loadFile) in `LoadFileOptions` . For example:

```javascript
let ui = slint.loadFile("/path/to/main.slint", {
    libraryPaths: {
        "material": "/path/to/material-1.0/material.slint"
    }
});
```

**Python**

- Provide the `library_paths` dict with [`load_file`](https://docs.slint.dev/latest/docs/python/slint#load_file) . For example:

```python
ui = slint.load_file(
    "/path/to/main.slint",
    library_paths={
        "material": "/path/to/material-1.0/material.slint"
    },
)
```

- When invoking the `slint-viewer` from the command line, pass `-Lmylibrary=/path/to/my/library` for each component library.
- When using the VS Code extension, configure the Slint extension’s library path using the `Slint: Library Paths` setting. Example below: ```json "slint.libraryPaths": { "mylibrary": "/path/to/my/library", "otherlibrary": "/path/to/otherlib/index.slint", }, ``` This can also be edited in the `.vscode/settings.json` file committed to your repository. Relative paths are resolved against the workspace root.
- With other editors, you can configure them to pass the `-L` argument to the `slint-lsp` just like for the slint-viewer.

## Functions

Source: `guide/language/coding/functions-and-callbacks/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/functions-and-callbacks/

Similar to other programming languages, functions in Slint are way to name, organize and reuse a piece of logic/code.

Functions can be defined as part of a component, or as part of an element within a component. It is not possible to declare global (top-level) functions, or to declare them as part of a struct or enum. It is also not possible to nest functions within other functions.

### Declaring Functions

Functions in Slint are declared using the `function` keyword. For example:

Functions can have parameters which are declared within parentheses, following the format `name: type`. These parameters can be referenced by their names within the function body. Parameters are passed by value.

Functions can also return a value. The return type is specified after `->` in the function signature. The return type is `void` if no return type is specified. The `return` keyword is used within the function body to return an expression of the declared type. If a function expects a return value and does not have an explicit return statement then the value of the last statement is returned by default.

Functions can be annotated with the `pure` keyword. This indicates that the function does not cause any side effects. More details can be found in the [Purity](https://docs.slint.dev/latest/docs/slint/guide/language/concepts/reactivity/index.html) chapter.

### Calling Functions

A function can be called without an element name (like a function call in other languages) or with an element name (like a method call in other languages):

```slint
import { Button, VerticalBox } from "std-widgets.slint";

export component Example {
    // Call without an element name:
    property <string> my-property: my-function();
    // Call with an element name:
    property <int> my-other-property: my_button.my-other-function();

    pure function my-function() -> string {
        return "result";
    }

    VerticalBox {
        Text {
            // Called with a pre-defined element:
            text: root.my-function();
        }

        my_button := Button {
            text: "Click me";
            clicked => { self.text = root.my-other-property; }
            pure function my-other-function() -> int {
                return 42;
            }
        }
    }
}
```

### Function Visibility

By default, functions are private and cannot be accessed from other components.

However, their accessibility can be modified using the `public` or `protected` keywords.

- A root-level function annotated with `public` can be accessed by any component.

To access such a function from a different component, you always need a target, which in practice means the calling component must declare the called component as one of its child elements.

```slint
export component HasFunction {
    public pure function double(x: int) -> int {
        return x * 2;
    }
}

export component CallsFunction {
    property <int> test: my-friend.double(1);

    my-friend := HasFunction {
    }
}
```

If a function is declared in a child element, even if marked public, it is not possible to call it from another component, as the child elements themselves are not public and a valid target for the function does not exist:

```slint
export component HasFunction {
    t := Text {
        public pure function double(x: int) -> int {
            return x * 2;
        }
    }
}

export component CallsFunction {
    // Compiler error!
    // property <int> test: my-friend.t.double(1);

    my-friend := HasFunction {
    }
}
```

Functions marked `public` in an exported component can also be invoked from backend code (Rust, C++, JS). See the language-specific documentation for the generated code to use.

- A function annotated with `protected` can only be accessed by components that directly inherit from it.

### Functions vs. Callbacks

There are a lot of similarities between functions and [callbacks](https://docs.slint.dev/latest/docs/slint/guide/language/coding/functions-and-callbacks/#callbacks):

- They are both callable blocks of logic/code
- They are invoked in the same way
- They can both have parameters and return values
- They can both be declared `pure`

But there are also differences:

- The code/logic in the callback can be set in the backend code and implemented in the backend language (Rust, C++, JS), while functions must be defined entirely in slint
- The syntax for defining a callback is different
- Callbacks can be declared without assigning a block of code to them
- Callbacks have a special syntax for declaring aliases using the two-way binding operator `<=>`
- Callback visibility is always similar to `public` functions

In general, the biggest reason to use callbacks is to be able to handle them from the backend code. Use a function if that is not needed.

### Callbacks

Components may declare callbacks, that communicate changes of state to the outside. Callbacks are invoked by “calling” them like you would call a function.

You react to callback invocation by declaring a handler using the `=>` arrow syntax. The built-in `TouchArea` element declares a `clicked` callback, that’s invoked when the user touches the rectangular area covered by the element, or clicks into it with the mouse. In the example below, the invocation of that callback is forwarded to another custom callback (`hello`) by declaring a handler and invoking our custom callback:

```slint
export component Example inherits Rectangle {
    // declare a callback
    callback hello;

    area := TouchArea {
        // sets a handler with `=>`
        clicked => {
            // emit the callback
            root.hello()
        }
    }
}
```

It’s possible to add parameters to a callback:

```slint
export component Example inherits Rectangle {
    // declares a callback
    callback hello(int, string);
    hello(aa, bb) => { /* ... */ }
}
```

Callbacks may also return a value:

```slint
export component Example inherits Rectangle {
    // declares a callback with a return value
    callback hello(int, int) -> int;
    hello(aa, bb) => { aa + bb }
}
```

Callback arguments can also have names. The names of arguments have currently no semantic value, but they improve readability of your code.

```slint
export component Example inherits Rectangle {
    // Declare a callback with named argument
    callback hello(foo: int, bar: string);
    // The names can be overridden with
    // anything when setting a handler
    hello(aa, bb) => { /* ... */ }
}
```

### Aliases

It’s possible to declare callback aliases in a similar way to two-way bindings:

```slint
export component Example inherits Rectangle {
    callback clicked <=> area.clicked;
    area := TouchArea {}
}
```

## Globals

Source: `guide/language/coding/globals/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/globals/

Declare a global singleton with `global Name { /* .. properties or callbacks .. */ }` to make properties and callbacks available throughout the entire project. Access them using `Name.property`.

For example, this can be useful for a common color palette:

Export a global to make it accessible from other files (see [Modules](https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/index.html#modules)). To make your global visible to native code with the business logic, re-export a global from the file also exporting the main application component.

```slint
export global Logic  {
    in-out property <int> the-value;
    pure callback magic-operation(int) -> int;
}
// ...
```

**Rust**

```rust
slint::slint!{
export global Logic {
    in-out property <int> the-value;
    pure callback magic-operation(int) -> int;
}

export component App inherits Window {
    // ...
}
}

fn main() {
    let app = App::new();
    app.global::<Logic>().on_magic_operation(|value| {
        eprintln!("magic operation input: {}", value);
        value * 2
    });
    app.global::<Logic>().set_the_value(42);
    // ...
}
```

**C++**

```cpp
#include "app.h"

int main() {
    auto app = App::create();
    app->global<Logic>().on_magic_operation([](int value) -> int {
        return value * 2;
    });
    app->global<Logic>().set_the_value(42);
    // ...
}
```

**NodeJS**

```js
let slint = require("slint-ui");
let file = slint.loadFile("app.slint");
let app = new file.App();
app.Logic.magic_operation = (value) => {
    return value * 2;
};
app.Logic.the_value = 42;
// ...
```

**Python**

```python
import slint

class App(slint.loader.app.App):
    @slint.callback(global_name="Logic")
    def magic_operation(self, value: int) -> int:
        return value * 2

app = new App()
app.Logic.the_value = 42

# ...
```

> **Note**
> Global singletons are not shared between windows. This means you may need to initialize the global callback and properties for each window instance you create in your application.

It’s possible to re-expose a callback or properties from a global using the two way binding syntax.

```slint
global Logic  {
    in-out property <int> the-value;
    pure callback magic-operation(int) -> int;
}

component SomeComponent inherits Text {
    // use the global in any component
    text: "The magic value is:" + Logic.magic-operation(42);
}

export component MainWindow inherits Window {
    // re-expose the global properties such that the native code
    // can access or modify them
    in-out property the-value <=> Logic.the-value;
    pure callback magic-operation <=> Logic.magic-operation;

    SomeComponent {}
}
```

## Name Resolution (Scope)

Source: `guide/language/coding/name-resolution/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/name-resolution/

Function calls have the same name resolution rules as properties and callbacks. When called without an element name:

- If the element that the function is called in ( `self` ) defines a function with that name, it is chosen.
- If not, name resolution continues to its parent element, and so on, until the root component.

When called with an element name (or `self`, `parent` or `root`), the function must be defined on that element. Name resolution does not look at ancestor elements in this case. Note that this means calling a function without an element name is *not* equivalent to calling it with `self` (which is how methods work in many languages).

Multiple functions with the same name are allowed in the same component, as long as they are defined on different elements. Therefore it is possible for a function to shadow another function from an ancestor element.

In the example above, the property `secret_number` will be set to 1, and the text labels will say “The secret number is 3” and “The other secret number is 2”.

## Positioning and Layouts

Source: `guide/language/coding/positioning-and-layouts/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/positioning-and-layouts/

All visual elements are shown in a window. The `x` and `y` properties store the elements coordinates relative to their parent element. Slint determines the absolute position of an element by adding the parent’s position to the element’s position. If the parent has a parent element itself, then that one is added as well. This calculation continues until the top-level element is reached.

The `width` and `height` properties store the size of visual elements.

You can create an entire graphical user interface by placing the elements in two ways:

- Explicitly - by setting the `x` , `y` , `width` , and `height` properties.
- Automatically - by using layout elements.

Explicit placement is great for static scenes with few elements. Layouts are suitable for complex user interfaces and help create scalable user interfaces. Layout elements express geometric relationships between elements.

### Explicit Placement

The following example places two rectangles into a window, a blue one and a green one. The green rectangle is a child of the blue:

![Explicit Placement](https://docs.slint.dev/latest/docs/slint/_astro/layouts-explicit-placement.CjNoX7zr_9kM8A.webp)

The positions of both rectangles and the size of the inner green one are fixed. The outer blue rectangle has a size that’s automatically calculated using binding expressions for the `width` and `height` properties. The calculation results in the bottom left corner aligning with the corner of the window - it updates whenever the `width` and `height` of the window changes.

When specifying explicit values for any of the geometric properties, Slint requires you to attach a unit to the number. You can choose between two different units:

- Logical pixels, using the `px` unit suffix. This is the recommended unit.
- Physical pixels, using the `phx` unit suffix

Logical pixels scale automatically with the device pixel ratio that your system is configured with. For example, on a modern High-DPI display the device pixel ratio can be 2, so every logical pixel occupies 2 physical pixels. On an older screen the user interface scales without any adaptations.

You can also specify the `width` and `height` properties as a `%` percentage unit, which applies relative to the parent element. For example a `width: 50%` means half of the parent’s `width`.

The default values for `x` and `y` properties center elements within their parent.

The default values for `width` and `height` depend on the type of element. Elements such as `Image`, `Text`, as well as most widgets are sized automatically based on their content. The following elements don’t have content and default to fill their parent element when they do not have children:

- `Rectangle`
- `TouchArea`
- `FocusScope`
- `Flickable`
- `SwipeGestureHandler`
- `ScaleRotateGestureHandler`

Layouts are also defaulting to fill the parent, regardless of their own preferred size. Other elements, including custom ones that don’t inherit from a base, default to using their preferred size.

#### Preferred Size

You can specify the preferred size of elements with the `preferred-width` and `preferred-height` properties.

When not explicitly set, the preferred size depends on the children, and is the preferred size of the child that has the bigger preferred size, whose `x` and `y` property are not set. The preferred size is therefore computed from the child to the parent, just like other constraints (maximum and minimum size), unless explicitly overwritten.

A special case is to set the preferred size to be the size of the parent using `100%` as value. For example, this component uses the size of the parent by default:

```slint
export component MyComponent {
    preferred-width: 100%;
    preferred-height: 100%;
    // ...
}
```

### Automatic Placement Using Layouts

Slint comes with different layout elements that automatically calculate the position and size of their children:

- `VerticalLayout` / `HorizontalLayout` : The children are placed along the vertical or horizontal axis.
- `GridLayout` : The children are placed in a grid of columns and rows.

You can also nest layouts to create complex user interfaces.

You can tune the automatic placement using different constraints, to accommodate the design of your user interface. Each element has a minimum, a maximum size, and a preferred size. Set these explicitly using the following properties:

- `min-width`
- `min-height`
- `max-width`
- `max-height`
- `preferred-width`
- `preferred-height`

Any element with a specified `width` and `height` has a fixed size in a layout.

When there is extra space in a layout, elements can stretch along the layout axis. You can control this stretch factor between the element and its siblings with these properties:

- `horizontal-stretch`
- `vertical-stretch`

A value of `0` means that the element won’t stretch at all. All elements stretch equally if they all have a stretch factor of `1`.

The default value of these constraint properties may depends on the content of the element. If the element’s `x` or `y` isn’t set, these constraints are also automatically applied to the parent element.

### Common Properties on Layout Elements

All layout elements have the following properties in common:

- `spacing` : This controls the spacing between the children.
- `padding` : This specifies the padding within the layout, the space between the elements and the border of the layout.

For more fine grained control, you can split the `padding` property into properties for each side of the layout:

- `padding-left`
- `padding-right`
- `padding-top`
- `padding-bottom`

### `VerticalLayout` and `HorizontalLayout`

The `VerticalLayout` and `HorizontalLayout` elements place their children in a column or a row. By default, they stretch or shrink to take the whole space. You can adjust the element’s alignment as needed.

The following example places the blue and yellow rectangle in a row and evenly stretched across the 200 logical pixels of `width`:

```slint
// Stretch by default
export component Example inherits Window {
    width: 200px;
    height: 200px;
    HorizontalLayout {
        Rectangle { background: blue; min-width: 20px; }
        Rectangle { background: yellow; min-width: 30px; }
    }
}
```

![Horizontal Layout](https://docs.slint.dev/latest/docs/slint/_astro/layouts-horizontal-layout.DymT5GaR_29kK90.webp)

The example below, on the other hand, specifies that the rectangles align to the start of the layout (the visual left). That results in no stretching but instead the rectangles retain their specified minimum width:

```slint
// Unless an alignment is specified
export component Example inherits Window {
    width: 200px;
    height: 200px;
    HorizontalLayout {
        alignment: start;
        Rectangle { background: blue; min-width: 20px; }
        Rectangle { background: yellow; min-width: 30px; }
    }
}
```

![Horizontal Layout with Alignment](https://docs.slint.dev/latest/docs/slint/_astro/layouts-horizontal-layout-align.Cnn2MHBi_Zsa20w.webp)

The example below nests two layouts for a more complex scene:

```slint
export component Example inherits Window {
    width: 200px;
    height: 200px;
    HorizontalLayout {
        // Side panel
        Rectangle { background: green; width: 10px; }

        VerticalLayout {
            padding: 0px;
            //toolbar
            Rectangle { background: blue; height: 7px; }

            Rectangle {
                border-color: red; border-width: 2px;
                HorizontalLayout {
                    Rectangle { border-color: blue; border-width: 2px; }
                    Rectangle { border-color: green; border-width: 2px; }
                }
            }
            Rectangle {
                border-color: orange; border-width: 2px;
                HorizontalLayout {
                    Rectangle { border-color: black; border-width: 2px; }
                    Rectangle { border-color: pink; border-width: 2px; }
                }
            }
        }
    }
}
```

![Nested Layouts](https://docs.slint.dev/latest/docs/slint/_astro/layouts-nested.Ccmubu0W_VP8JU.webp)

#### Relative Lengths

Sometimes it’s convenient to express the relationships of length properties in terms of relative percentages. For example the following inner blue rectangle has half the size of the outer green window:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;

    background: green;
    Rectangle {
        background: blue;
        width: parent.width * 50%;
        height: parent.height * 50%;
    }
}
```

This pattern of expressing the `width` or `height` in percent of the parent’s property with the same name is common. For convenience, a short-hand syntax exists for this scenario:

- The property is `width` or `height`
- A binding expression evaluates to a percentage.

If these conditions are met, then it’s not necessary to specify the parent property, instead you can simply use the percentage. The earlier example then looks like this:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;

    background: green;
    Rectangle {
        background: blue;
        width: 50%;
        height: 50%;
    }
}
```

#### Alignment

Each element is sized according to their `width` or `height` if specified, otherwise it’s set to the minimum size which is set with the min-width or min-height property, or the minimum size of an inner layout, whatever is bigger.

The elements are placed according to the alignment. The size of elements is bigger than the minimum size only if the `alignment` property of the layout is `LayoutAlignment.stretch` (the default)

This example show the different alignment possibilities:

```slint
export component Example inherits Window {
    width: 300px;
    height: 200px;
    VerticalLayout {
        HorizontalLayout {
            alignment: stretch;
            Text { text: "stretch (default)"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
        HorizontalLayout {
            alignment: start;
            Text { text: "start"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
        HorizontalLayout {
            alignment: end;
            Text { text: "end"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
        HorizontalLayout {
            alignment: start;
            Text { text: "start"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
        HorizontalLayout {
            alignment: center;
            Text { text: "center"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
        HorizontalLayout {
            alignment: space-between;
            Text { text: "space-between"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
        HorizontalLayout {
            alignment: space-around;
            Text { text: "space-around"; }
            Rectangle { background: blue; min-width: 20px; }
            Rectangle { background: yellow; min-width: 30px; }
        }
    }
}
```

![Layouts with different alignments](https://docs.slint.dev/latest/docs/slint/_astro/layouts-alignment.DFnpm4AY_Z20jPM4.webp)

#### Stretch algorithm

When the `alignment` is set to stretch (the default), the elements are sized to their minimum size, then the extra space is shared amongst element proportional to their stretch factor set with the `horizontal-stretch` and `vertical-stretch` properties. The stretched size won’t exceed the maximum size. The stretch factor is a floating point number. The elements that have a default content size usually defaults to 0 while elements that default to the size of their parents defaults to 1. An element of a stretch factor of 0 keep its minimum size, unless all the other elements also have a stretch factor of 0 or reached their maximum size.

Examples:

```slint
export component Example inherits Window {
    width: 300px;
    height: 200px;
    VerticalLayout {
        // Same stretch factor (1 by default): the size is divided equally
        HorizontalLayout {
            Rectangle { background: blue; }
            Rectangle { background: yellow;}
            Rectangle { background: green;}
        }
        // Elements with a bigger min-width are given a bigger size before they expand
        HorizontalLayout {
            Rectangle { background: cyan; min-width: 100px;}
            Rectangle { background: magenta; min-width: 50px;}
            Rectangle { background: gold;}
        }
        // Stretch factor twice as big:  grows twice as much
        HorizontalLayout {
            Rectangle { background: navy; horizontal-stretch: 2;}
            Rectangle { background: gray; }
        }
        // All elements not having a maximum width have a stretch factor of 0 so they grow
        HorizontalLayout {
            Rectangle { background: red; max-width: 20px; }
            Rectangle { background: orange; horizontal-stretch: 0; }
            Rectangle { background: pink; horizontal-stretch: 0; }
        }
    }
}
```

![Explicit Placement](https://docs.slint.dev/latest/docs/slint/_astro/layouts-stretch.D9hUJ5o7_wkF8r.webp)

#### `for`

The VerticalLayout and HorizontalLayout can also contain `for` or `if` expressions:

```slint
export component Example inherits Window {
    width: 200px;
    height: 50px;
    HorizontalLayout {
        Rectangle { background: green; }
        for t in [ "Hello", "World", "!" ] : Text {
            text: t;
        }
        Rectangle { background: blue; }
    }
}
```

![Explicit Placement](https://docs.slint.dev/latest/docs/slint/_astro/layouts-for-loop.pOSOlrdq_Z1AXGEw.webp)

### GridLayout

The GridLayout lays the element in a grid. Each element gains the properties `row`, `col`, `rowspan`, and `colspan`. You can either use a `Row` sub-element, or set the `row` property explicitly. These properties must be statically known at compile time, so it’s impossible to use arithmetic or depend on properties. As of now, the use of `for` or `if` isn’t allowed in a grid layout.

This example use the `Row` element

```slint
export component Foo inherits Window {
    width: 200px;
    height: 200px;
    GridLayout {
        spacing: 5px;
        Row {
            Rectangle { background: red; }
            Rectangle { background: blue; }
        }
        Row {
            Rectangle { background: yellow; }
            Rectangle { background: green; }
        }
    }
}
```

![Explicit Placement](https://docs.slint.dev/latest/docs/slint/_astro/layouts-grid.CPyFvTOY_8LHYe.webp)

This example use the `col` and `row` property:

```slint
export component Foo inherits Window {
    width: 200px;
    height: 150px;
    GridLayout {
        spacing: 0px;
        Rectangle { background: red; }
        Rectangle { background: blue; }
        Rectangle { background: yellow; row: 1; }
        Rectangle { background: green; }
        Rectangle { background: black; col: 2; row: 0; }
    }
}
```

![Explicit Placement](https://docs.slint.dev/latest/docs/slint/_astro/layouts-grid-col-row.CmMo3FAp_L1yMu.webp)

### Container Components (@children)

When creating components, it’s sometimes useful to influence where child elements are placed when used. For example, a component that draws a label above an element inside:

```slint
export component MyApp inherits Window {

    BoxWithLabel {
        Text {
            // ...
        }
    }

    // ...
}
```

You can implement such a `BoxWithLabel` using a layout. By default child elements like the `Text` element become direct children of the `BoxWithLabel`, but for this example they need to become children of the layout instead. To do this can change the default child placement by using the `@children` expression inside the element hierarchy of a component:

```slint
component BoxWithLabel inherits GridLayout {
    Row {
        Text { text: "label text here"; }
    }
    Row {
        @children
    }
}

export component MyApp inherits Window {
    preferred-height: 100px;
    BoxWithLabel {
        Rectangle { background: blue; }
        Rectangle { background: yellow; }
    }
}
```

## Properties

Source: `guide/language/coding/properties/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/properties/

All elements have properties. Built-in elements come with common properties such as color or dimensional properties.

### Assigning bindings

You can assign values or entire [Expressions](https://docs.slint.dev/latest/docs/slint/guide/language/coding/expressions-and-statements/index.html) to them:

Properties in Slint can be assigned using either a simple **expression**, which ends with a semicolon (`;`), or a **code block**, enclosed in `{ ... }`, which does not require a semicolon. These two forms are interchangeable for simple values, but a block is useful when you need multiple statements.

For example, these two bindings are equivalent:

```slint
background: touch-area.is-pressed ? red : blue;

background: {
    if (touch-area.is-pressed) {
        return red;
    } else {
        return blue;
    }
}
```

Both forms are reactive, and Slint will automatically track dependencies used within the expression or block.

The default value of a property is the default value of the type. For example a boolean property defaults to `false`, an `int` property to zero, etc.

### Declaring Properties

In addition to the existing properties, define extra properties by specifying the type, the name, and optionally a default value:

```slint
export component Example {
    // declare a property of type int with the name `my-property`
    property<int> my-property;

    // declare a property with a default value
    property<int> my-second-property: 42;
}
```

Annotate extra properties with a qualifier that specifies how the property can be read and written:

- **`private`** (the default): The property can only be accessed from within the component.
- **`in`** : The property is an input. It can be set and modified by the user of this component, for example through bindings or by assignment in callbacks. The component can provide a default binding, but it can’t overwrite it by assignment
- **`out`** : An output property that can only be set by the component. It’s read-only for the users of the components.
- **`in-out`** : The property can be read and modified by everyone.

```slint
export component Button {
    // This is meant to be set by the user of the component.
    in property <string> text;
    // This property is meant to be read by the user of the component.
    out property <bool> pressed;
    // This property is meant to both be changed by the user and the component itself.
    in-out property <bool> checked;

    // This property is internal to this component.
    private property <bool> has-mouse;
}
```

All properties declared at the top level of a component that aren’t `private` are accessible from the outside when using a component as an element, or via the language bindings from the business logic.

### Change Callbacks

In Slint, it’s possible to define a callback that is invoked when a property’s value changes.

```slint
import { LineEdit } from "std-widgets.slint";
export component Example inherits Window  {
    VerticalLayout {
        LineEdit {
            // This callback is invoked when the `text` property of the LineEdit changes
            changed text => { t.text = self.text; }
        }
        t := Text {}
    }
}
```

Note that these callbacks aren’t invoked immediately. Instead, they’re queued for invocation in the subsequent iteration of the event loop. A callback is invoked only if the property’s value has indeed changed. If a property’s value changes multiple times within the same event loop cycle, the callback is invoked only once. Additionally, if a property’s value changes and then reverts to its original state before the callback is executed, the callback won’t be invoked.

**Warning:** Altering properties during a change event in a way that could lead to the same property being affected is undefined behavior.

```slint
export component Example {
    in-out property <int> foo;
    property <int> bar: foo + 1;
    // This setup creates a potential loop between `foo` and `bar`, and the outcome is undefined.
    changed bar => { foo += 1; }
}
```

The above represents an infinite loop. Slint will break the loop after a few iterations. Consequently, if there’s a sequence of changed callbacks where one callback triggers another change callback, this sequence might break, and further callbacks won’t be invoked.

Therefore, it’s crucial not to overuse changed callbacks.

**Warning:** Utilize changed callbacks only when an alternative through binding isn’t feasible.

For instance, avoid doing this:

```slint
changed bar => { foo = bar + 1; }
```

Instead, opt for:

```slint
foo: bar + 1;
```

Declarative bindings automatically manage dependencies. Using a changed callback forces immediate evaluation of bindings, which are typically evaluated lazily. This practice also compromises the purity of bindings, complicating edits via graphical editors. Accumulating excessive changed events can introduce issues and bugs, especially in scenarios involving loops, where a change callback modifies a property, potentially triggering changes to the same property.

## Repetition

Source: `guide/language/coding/repetition-and-data-models/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/repetition-and-data-models/

Use the `for`-`in` syntax to create an element multiple times.

The syntax looks like this: `for name[index] in model : id := Element { ... }`

The *model* can be of the following type:

- an integer, in which case the element will be repeated that amount of time
- an [array type or a model](https://docs.slint.dev/latest/docs/slint/guide/language/coding/repetition-and-data-models/#arrays-and-models) declared natively, in which case the element will be instantiated for each element in the array or model.

The *name* will be available for lookup within the element and is going to be like a pseudo-property set to the value of the model. The *index* is optional and will be set to the index of this element in the model. The *id* is also optional.

### Examples

```slint
export component Example inherits Window {
    preferred-width: 50px;
    preferred-height: 50px;
    in property <[{foo: string, col: color}]> model: [
        {foo: "abc", col: #f00 },
        {foo: "def", col: #00f },
    ];
    VerticalLayout {
        for data in root.model: my-repeated-text := Text {
            color: data.col;
            text: data.foo;
        }
    }
}
```

### Arrays and Models

Arrays are declared by wrapping `[` and `]` square brackets around the type of the array elements.

Array literals as well as properties holding arrays act as models in `for` expressions.

```slint
export component Example {
    in-out property<[int]> list-of-int: [1,2,3];
    in-out property<[{a: int, b: string}]> list-of-structs: [{ a: 1, b: "hello" }, {a: 2, b: "world"}];
}
```

Arrays define the following operations:

- **`array.length`** : One can query the length of an array and model using the builtin `.length` property.
- **`array[index]`** : The index operator retrieves individual elements of an array.

Out of bound access into an array will return default-constructed values.

```slint
export component Example {
    in-out property<[int]> list-of-int: [1,2,3];

    out property <int> list-len: list-of-int.length;
    out property <int> first-int: list-of-int[0];
}
```

## States

Source: `guide/language/coding/states/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/states/

The `states` statement allows to declare states and set properties of multiple elements in one go:

In this example, the `active` and `active-hovered` states are defined depending on the value of the `active` boolean property and the `TouchArea`’s `has-hover`. When the user hovers the example with the mouse, it will toggle between a blue and a green background, and adjust the text label accordingly. Clicking toggles the `active` property and thus enters the `inactive` state.

### Transitions

Transitions bind animations to state changes.

This example defines two transitions. First the `out` keyword is used to animate all properties for 800ms when leaving the `disabled` state. The second transition uses the `in` keyword to animate the background when transitioning into the `down` state.

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;

    text := Text { text: "hello"; }
    in-out property<bool> pressed;
    in-out property<bool> is-enabled;

    states [
        disabled when !root.is-enabled : {
            background: gray; // same as root.background: gray;
            text.color: white;
            out {
                animate * { duration: 800ms; }
            }
        }
        down when pressed : {
            background: blue;
            in {
                animate background { duration: 300ms; }
            }
        }
    ]
}
```

#### Transition Types

There are three types of transitions you can define:

- **`in`** : Animates properties when entering a state
- **`out`** : Animates properties when leaving a state
- **`in-out`** : Animates properties both when entering and leaving a state

The `in-out` transition is useful when you want the same animation to play for both entering and exiting a state, avoiding the need to duplicate the animation definition.

## Structs and Enums

Source: `guide/language/coding/structs-and-enums/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/coding/structs-and-enums/

### Structs

Define named structures using the `struct` keyword:

The default value of a struct, is initialized with all its fields set to their default value.

#### Anonymous Structures

Declare anonymous structures using `{ identifier1: type1, identifier2: type2 }` syntax, and initialize them using `{ identifier1: expression1, identifier2: expression2 }`.

You may have a trailing `,` after the last expression or type.

```slint
export component Example {
    in-out property<{name: string, score: int}> player: { name: "Foo", score: 100 };
    in-out property<{a: int, }> foo: { a: 3 };
}
```

### Enums

Define an enumeration with the `enum` keyword:

```slint
export enum CardSuit { clubs, diamonds, hearts, spade }

export component Example {
    in-out property<CardSuit> card: spade;
    out property<bool> is-clubs: card == CardSuit.clubs;
}
```

Enum values can be referenced by using the name of the enum and the name of the value separated by a dot. (eg: `CardSuit.spade`)

The name of the enum can be omitted in bindings of the type of that enum, or if the return value of a callback is of that enum.

The default value of each enum type is always the first value.

## Reactivity vs React.js

Source: `guide/language/concepts/reactivity-vs-react/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/concepts/reactivity-vs-react/

### Comparison to React.js

The following sections are for those coming from or are familiar with the [React.js↗](https://react.dev/) web framework. We’re going to compare patterns from React.js based app development with Slint.

> **Note**
> There is no web browser as part of Slint to render the UI. There is no DOM or shadow DOM.

#### Component Life Cycle Management

React.js has a model where on a state change a component is destroyed and recreated. By default this will also include the destruction of all child components of an element and these then all need to be recreated. To manage performance, careful use of `useMemo()` and `useCallback()` are needed to avoid unnecessary re-renders. Even though the need for this has been reduced via the React Compiler it’s still necessary to understand this model to understand how a React app behaves.

Slint is much simpler and uses fine-grained reactivity: Components update, but they aren’t destroyed and recreated. There is no equivalent of `useMemo()` and `useCallback()` as they are unnecessary.

#### State

React.js refers to properties that update and re-render the component as state. They are opt-in and by default are not tracked.

```slint
import { Button } from "std-widgets.slint";

export component Counter {
    property <int> count: 0;
    Button {
        text: count;
        clicked => {
            count += 1;
        }
    }
}
```

The classic counter example also shows key differences. First the `count` property is declared and a `count` value and `setCount()` function are deconstructed from the useState hook. Note that ‘count’ cannot be directly accessed and must be updated via `setCount()`.

The counter button is then used to update the count property and to ensure it’s correctly updated must rely on the `currentCount` value returned by `setCount()` is used to update the values. As using `setCount(count + 1)` can cause issues in more complex scenarios where the state is updated later.

While the Slint example may not look much simpler, it does the same job and has less gotchas. As everything in Slint is reactive by default, the property is declared in one single way. The language has strong types and for numbers has both `float`s and `int`s. The property can also be safely modified directly which also in this example allows the use of the `+=` operator.

## Reactivity

Source: `guide/language/concepts/reactivity/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/concepts/reactivity/

### Reactivity

Reactivity is core concept within Slint. It allows the creation of complex dynamic user interfaces with a fraction of the code. The following examples will help you understand the basics of reactivity.

As the name suggests, Reactivity is all about parts of the user interface automatically updating or ‘reacting’ to changes. The above example looks simple, but when run it does several things:

- The `Rectangle` will follow the mouse around as you move it.
- If you `click` anywhere the `Rectangle` will change color.
- The `Text` elements will update their text to show the current position of the `Rectangle` .

The ‘magic’ here is built into the Slint language directly. There is no need to opt into this or define specific stateful items. The `Rectangle` will automatically update because its `x` and `y` properties are bound to the `mouse-x` and `mouse-y` properties of the `TouchArea` element. This was done by giving the `TouchArea` a name to identify it `ta` and then using the name in what Slint calls an `expression` to track values. It’s as simple as `x: ta.mouse-x;` and `y: ta.mouse-y;`. The mouse-x and mouse-y properties are built into the `TouchArea` and automatically update as the cursor moves over them.

The `TouchArea` also has a `pressed` property that is only `true` when the cursor is pressed or clicked down. So the ternary expression `background: ta.pressed ? orange : white;` will change the background color of the `Rectangle` to orange when `ta.pressed` is true, or white when it isn’t.

Similarly the 2 text items are updating by tracking the rectangle’s `x` and `y` position.

### Overwritten Bindings

Bindings in Slint are only active as long as the property is assigned using a binding expression (`x: other.value`) or a two-way binding (`<=>`). If a property’s value is later changed using an imperative assignment in code (e.g. `foo.bar = 42;`), the original binding is broken. From that point on, the property will no longer react to changes in the values it was previously bound to. If needed, the property can still be updated again later through another assignment, but the automatic reactivity is lost.

This behavior also applies to most built-in `in-out` properties, such as the `text` property of a `TextInput`. When the user interacts with the widget, for example by typing into the input, this is considered an imperative assignment that will break any existing binding to that property.

To maintain reactivity in such cases, you can use a two-way binding (`<=>`), or you can make use of the `changed` callback to track and respond to property updates.

### Performance

From a performance perspective, Slint works out what properties are changed. It then finds all the expressions that depend on that value. These dependencies are then re-evaluated based on the new values and the UI will update.

The re-evaluation happens lazily when the property is queried.

Internally, a dependency is registered for any property accessed while evaluating a binding. When a property changes, the dependencies are notified and all dependent bindings are marked as dirty.

### Property Expressions

Expressions can vary in complexity:

```slint
// Tracks the `x` value of an element called foo
x: foo.x;

// Tracks the value, but sets it to 0px or 400px based on if
// foo.x is greater than 400px
x: foo.x > 100px ? 0px : 400px;

// Tracks the value, but clamps it between 0px and 400px
x: clamp(foo.x, 0px, 400px);
```

As the last example shows functions can be used as part of a property expression. This can be useful for when an expression is too complex to be readable or maintained as a single line.

```slint
export component MyComponent {
    width: 400px; height: 400px;

    pure function lengthToInt(n: length) -> int {
        return (n / 1px);
    }

    Rectangle {
        background: #151515;
    }

    ta := TouchArea {}

    myRect := Rectangle {
        x: ta.mouse-x;
        y: ta.mouse-y;
        width: 60px;
        height: 60px;
        background: ta.pressed ? orange : skyblue;
        Text {
            x: 5px; y: 5px;
            text: "x: " + lengthToInt(myRect.x);
            color: white;
        }
        Text {
            x: 5px; y: 20px;
            text: "y: " + lengthToInt(myRect.y);
            color: white;
        }
    }
}
```

Here the earlier example was updated to use a function to convert the length to an integer. This also truncates the x and y values to be more readable i.e. ‘4’ instead of ‘4.124488’.

### Purity

For any reactive system to work well, evaluating a property shouldn’t change any observable state but the property itself. If this is the case, then the expression is “pure”, otherwise it’s said to have side-effects. Side-effects are problematic because it’s not always clear when they will happen: Lazy evaluation may change their order or affect whether they happen at all. In addition, changes to properties during their binding evaluation due to a side-effect may result in unexpected behavior.

For this reason, bindings in Slint **must** be pure. The Slint compiler enforces code in pure contexts to be free of side effects. Pure contexts include binding expressions, bodies of pure functions, and bodies of pure callback handlers. In such a context, it’s not allowed to change a property, or call a non-pure callback or function.

Annotate callbacks and public functions with the `pure` keyword to make them accessible from property bindings and other pure callbacks and functions.

The purity of private functions is automatically inferred. You may declare private functions explicitly “pure” to have the compiler enforce their purity.

```slint
export component Example {
    pure callback foo() -> int;
    public pure function bar(x: int) -> int
    { return x + foo(); }
}
```

### Two-Way Bindings

Create two-way bindings between properties with the `<=>` syntax. These properties will be linked together and always contain the same value. Also known as bidirectional or bi-directional bindings.

The right hand side of the `<=>` must be a reference to a property of the same type, or a field of the same type within a property of struct type. The property type is optional with two-way bindings, it will be inferred if not specified. The initial value of a linked property will be the value of the right hand side of the binding. The two linked properties must be compatible in terms of input/output.

The initial value of a linked property will be the value of the right hand side of the binding.

```slint
struct Thing { name: string, price: int }

export component Example  {
    in property<brush> rect-color <=> r.background;
    // It's allowed to omit the type to have it automatically inferred
    in property rect-color2 <=> r.background;
    in-out property<Thing> thing;
    r:= Rectangle {
        background: blue;
    }
    input := TextInput {
        // The `thing.name` field will be sync'ed with the text when the user edits
        text <=> root.thing.name;
    }
}
```

## Slint Language

Source: `guide/language/concepts/slint-language/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/language/concepts/slint-language/

This following section gives you an insight into the thinking behind the language and the core concepts it’s made up from.

[YouTube video](https://www.youtube.com/watch?v=GLloeHGWb3A)

As covered in the video above, the Slint declarative UI language is designed to be a simple, yet powerful way to create any user interface you can imagine.

This example shows the core of how Slint works. Use elements with their name followed by open and closed braces, e.g. `Text {}`. Then within the braces customize it with properties e.g. `font-size: 24px`.

To nest elements inside each other place them within the parents braces. For example the following `Rectangle` has a `Text` element as its child.

```slint
Rectangle {
    width: 150px;
    height: 60px;
    background: white;
    border-radius: 10px;

    Text {
        text: "Hello World!";
        font-size: 24px;
        color: black;
    }
}
```

![Image showing text nested inside a rectangle](https://docs.slint.dev/latest/docs/slint/_astro/intro-nesting.CNliPhF2_ZztjQC.webp)

The final core part are binding expressions:

```slint
property <int> counter: 0;

Rectangle {
    width: 150px;
    height: 60px;
    background: white;
    border-radius: 10px;

    Text {
        text: "Count: " + counter;
        font-size: 24px;
        color: black;
    }

    TouchArea {
        clicked => {
            counter += 1;
        }
    }
}
```

In this example a property called `counter` is declared. Then the `Rectangle` has a `TouchArea` inside, that automatically fills its parent and responds to clicks or taps. On click it increments `counter`. Here is where the magic of fine grained reactivity comes in.

The `Text` element’s `text` property depends on the `counter` property. When `counter` changes, the UI is automatically updated. There’s no need to opt in. In Slint every expression is automatically re-evaluated. However this is done in a performant way that only updates expressions where the dependencies have changed.

Slint makes it trivial to create your own components by composing together the built in elements or other components. `Text` and `Rectangles` can become buttons. Buttons and input fields can become forms or dialogs. Forms and dialogs can become Views. And finally Views combine to become applications.

```slint
export component MyView {
    MyDialog {
        title: "Can UI Development Be Easy?";

        MyButton {
            text: "Yes";
        }
    }
}
```

With practice you can reduce any level of complexity to simple, maintainable UI components.

### Why Slint?

The objective of the concepts section of the guide is to give you an overview of the language from a high level. If you want to dive straight in the details then check out the [coding section](https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/index.html) and the language reference sections.

The Slint language describes your application’s User Interface in a declarative way.

A User Interface is quite different from abstract code. It’s made up of text, images, colors, animations, and so on. Even though it’s a mirage, the buttons and elements do not really exist in the physical world, it’s meant to look and behave as if it does. Buttons have pressed effects, lists can be flicked and behave as if they had real inertia. When being designed they are described in terms of UI components, forms, and views.

Meanwhile the world of code is a quite different abstraction. It’s made up of functions, variables, and so on. Even when some aspects of a UI exist such as buttons and menus they also go hand in hand with a lot of code to manage the implementation details.

```js
const button = document.createElement('button');
button.textContent = 'Click me';
document.body.appendChild(button);
```

Take this simple web example. A button is created. Then a property to show ‘Click me’ text is text. At this point technically the button exists, but it won’t show up as its not attached to anything. So the final line makes it a child of the main view.

```js
const buttonWithListener = document.createElement('button');
buttonWithListener.textContent = 'Click me';
buttonWithListener.addEventListener('click', () => {
    console.log('Button clicked!');
});
document.body.appendChild(buttonWithListener);
```

In this second example a button is created and an event listener is added. Within that is a callback function to log out that the button is pressed. It’s a lot of code to do simple things.

It’s quite abstract. For example once a few more buttons and components are added its almost impossible to be able to think how the real interface would look. It’s hard to think about the design of a UI using this kind of code.

It’s also too complex to edit without understanding how to code. This means UI designers cannot work hands on to ensure all their design intent is implemented. They are forced to use other tools and frameworks to create prototypes and design guides that may look or behave differently to the actual UI implementation. It doesn’t have to be this way.

### Declarative Style

There have been attempts to make describing a UI in code more declarative. For example [React↗](https://reactjs.org/) and [SwiftUI↗](https://developer.apple.com/xcode/swiftui/).

```jsx
function ContentView() {
  return (
    <p style={{ fontSize: '2rem', color: 'green' }}>
      Hello World
    </p>
  );
}
```

```swift
struct ContentView: View {
    var body: some View {
        Text("Hello World")
            .font(.title)
            .foregroundColor(.green)
    }
}
```

These languages take normal code and let it be used in a declarative way. But it’s still functions with arguments that act as properties. It is simpler and behavior such as parent child relationships can be inferred.

For Slint, instead of tweaking a normal language to be more declarative, we created a pure declarative language from the ground up.

### The Business Logic Problem

One attempt to solve the problem of describing a UI in code has been to create a separate static markup language. Platforms such as Android and [WPF↗](https://docs.microsoft.com/en-us/dotnet/desktop/wpf/) have done this. The UI is described in an XML-like format. While the rest of the code is written in both a separate language and also separate files. Known as a code behind file. The issue here is that XML, despite claims to the contrary, is not human readable or editable. It’s also too static and rigid and it can be frustrating to jump between the UI file and the separate code behind file to describe the behavior of the UI.

Meanwhile the React web framework has solved this problem by using JSX. HTML, CSS and JavaScript can be mixed together in a single file. On one hand this is great as it means you can use the same language to describe the UI layout and behavior. On the other hand there is no limit to what code you can put in a JSX file. Logic for handling network requests, processing data and pretty much anything quickly ends up mixed in with UI code. It leads to the issue where it can be fast to create an initial application, but it becomes so hard to maintain it that it’s too slow or costly to evolve the application.

### A True Declarative UI Language

Slint provides a declarative language to describe an application’s user interface

```slint
Rectangle {
    Button {
        text: "Click me!";
        clicked => {
            debug("Button clicked!");
        }
    }
}
```

If you can understand this Slint code — you’ve already grasped a majority of how simple to use the language is. On line 2 we declare a `Button`. On line 3 we set its `text` property to `"Click me!"` and on line 4 we set a callback to print out “Button clicked!” to the console via the built in `debug` function.

The Slint compiler will take this code and see that it needs to generate a Rectangle that has a Button as a child. Similarly, when the clicked callback activates, it will run the `debug` function. There is no need to think about component and event listener life cycles.

With the Slint language you can think much more closely to how a user interface looks and behaves. Instead of describing the ‘how’, the how should the details be implemented in traditional code, you “declare” how the interface should look and behave. Hence Slint being a ‘declarative UI language’.

It’s not only simpler for software developers to use, it’s also now something designers can potentially edit or more easily contribute to.

Slint isn’t the first declarative UI language, but it takes advantage of being able to learn from earlier more complex attempts that hinted at the potential of a declarative UI language to finalize into a modern and complete system.

At first glance it has the simplicity of a static markup language, but with a modern take that removes things like angle brackets and tags. But also dynamic features such as complex property expressions, functions, callbacks, and automatic reactivity. However these can only be used in the context of what helps a UI component. If you want to make network requests, process data, or many other things that count as ‘business logic’ then those have to live in separate files written in Rust, C++, JavaScript, etc. Slint allows you to express any UI and only the UI.

It then provides a system of adapters to allow the business logic side of the app to easily communicate with the UI and visa versa.

## Desktop

Source: `guide/platforms/desktop/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/desktop/

Generally, Slint runs on Windows, macOS, and popular Linux distributions. The following tables cover versions that we specifically test. The general objective is to support the operating systems that are supported by their vendors at the time of a Slint version release.

**Windows**

| Operating System | Architecture |
| --- | --- |
| Windows 10 | x86-64 |
| Windows 11 | x86-64, aarch64 |

#### Handle the Console Window

When you running an application a console window will show by [default↗](https://learn.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-170).

Disable the console by specifying a `WINDOWS` subsystem.

When running the application from the command line, if the subsystem is set to windows it will no longer output stdout. To get it back consider using `FreeConsole()`.

See more details at [#3235↗](https://github.com/slint-ui/slint/issues/3235)

**Rust**

Add the code to the top of .rs file which contains `fn main()`:

Or if you want to keep console output in debug mode:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

**C++**

Select the `WINDOWS` subsystem by setting the [`WIN32_EXECUTABLE`↗](https://cmake.org/cmake/help/latest/prop_tgt/WIN32_EXECUTABLE.html#prop_tgt:WIN32_EXECUTABLE) target property on your executable target:

```cmake
add_executable(my_program ...)
set_property(TARGET my_program APPEND PROPERTY WIN32_EXECUTABLE TRUE)
```

**Python**

Change the extension of your Python script from `.py` to `.pyw`, the default Python interpreter associated with Python files will launch without a console window. Alternatively, use `pythonw.exe` instead of `python.exe` to launch your Python script.

#### Rust: Stack Overflows

When developing Rust applications on Windows, you might sooner or later observe the program aborting with `STATUS_STACK_OVERFLOW`, especially in debug builds. This is a known issue that’s a combination of a high demand for stack space and MSVC defaulting to a stack size for the main thread that’s significantly smaller compared to other operating systems.

This is fixed by configuring the linker. Create a [`.cargo\config.toml`↗](https://doc.rust-lang.org/cargo/reference/config.html#configuration) file in your project (note the `.cargo` sub-directory) with the following contents:

```toml
[target.x86_64-pc-windows-msvc]
# Increase default stack size to avoid running out of stack
# space in debug builds. The size matches Linux's default.
rustflags = ["-C", "link-arg=/STACK:8000000"]
[target.aarch64-pc-windows-msvc]
# Increase default stack size to avoid running out of stack
# space in debug builds. The size matches Linux's default.
rustflags = ["-C", "link-arg=/STACK:8000000"]
```

**macOS**

| Operating System | Architecture |
| --- | --- |
| macOS 14 Sonoma | aarch64 |
| macOS 15 Sequoia | aarch64 |
| macOS 26 | aarch64 |

**Linux**

Linux desktop distribution present a diverse landscape, and Slint should run on any of them, provided that they are using Wayland or X-Windows, glibc, and d-bus. If a Linux distribution provides Long Term Support (LTS), Slint should run on the most recent LTS or newer, at the time of a Slint version release.

### Other Platforms

[Contact us↗](https://slint.dev/contact) if you want to use Slint on other platforms/versions.

## Embedded

Source: `guide/platforms/embedded/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/embedded/

Slint runs on many embedded platforms.

The platform descriptions below cover what has been tested for deployment. For the development environment, we recommend using a recent desktop operating system and compiler version.

### Embedded Linux

Slint runs on a variety of embedded Linux platforms. Generally speaking, Slint requires a modern Linux userspace with working OpenGL ES 2.0 (or newer) or Vulkan drivers. We’ve had success running Slint on

- Yocto based distributions.
- BuildRoot based distributions.
- Torizon.

**Yocto Linux**

For C++ applications see [meta-slint↗](https://github.com/slint-ui/meta-slint) for recipes.

Rust applications work out of the box with Yocto’s Rust support.

**Torizon**

Toradex provides [Torizon OS↗](https://developer.toradex.com/torizon/), a Linux based platform for its embedded devices that packages applications in docker containers.

We provide our demos compiled for Toradex as docker containers with and without GPU acceleration support.

##### Prerequisites

- A device running Torizon OS 6.0 or later
- SSH access to the Torizon device
- Docker installed on the device

**Note**: The Slint demos run directly on the linux/kms and do not require a Weston container.

##### Running

Our pre-compiled demos are available in multiple variants optimized for different hardware platforms:

1. **Standard ARM64 without GPU build** ( `torizon-demos-arm64` ) - Uses software rendering
2. **i.MX8 GPU build** ( `torizon-demos-arm64-imx8` ) - Optimized for i.MX8 series with GPU acceleration
3. **AM62 GPU build** ( `torizon-demos-arm64-am62` ) - Optimized for AM62 series with GPU acceleration
4. **i.MX95 GPU build** ( `torizon-demos-arm64-imx95` ) - Optimized for i.MX95 series with GPU acceleration

A complete list of all containers can be found at

[https://github.com/orgs/slint-ui/packages?q=torizon&tab=packages&q=torizon↗](https://github.com/orgs/slint-ui/packages?q=torizon&tab=packages&q=torizon)

For example, to run the container on an i.MX8 board with GPU, deploy the following `docker-compose.yaml` with Docker Compose:

Change the `image` to match your hardware platform:

| Platform | Image |
| --- | --- |
| ARM64 no GPU | `ghcr.io/slint-ui/slint/torizon-demos-arm64:latest` |
| i.MX8 w/ GPU | `ghcr.io/slint-ui/slint/torizon-demos-arm64-imx9:latest` |
| AM62 w/ GPU | `ghcr.io/slint-ui/slint/torizon-demos-am62:latest` |
| i.MX95 W/ GPU | `ghcr.io/slint-ui/slint/torizon-demos-imx95:latest` |

Alternative, run the container with the following command line:

```plaintext
sudo docker run --rm --privileged \
  --user=torizon \
  -v /dev:/dev \
  -v /tmp:/tmp \
  -v /run/udev:/run/udev \
  --device-cgroup-rule='c 199:* rmw' \
  --device-cgroup-rule='c 226:* rmw' \
  --device-cgroup-rule='c 13:* rmw' \
  --device-cgroup-rule='c 4:* rmw' \
  ghcr.io/slint-ui/slint/torizon-demos-arm64-imx8
```

##### Selecting Demos

By default, the **home-automation** demo is run. The containers package multiple demo applications:

- **home-automation** (default) - Smart home control panel
- **energy-monitor** - Energy monitoring dashboard
- **printerdemo** - 3D printer control interface
- **gallery** - Image gallery with touch navigation
- **slide_puzzle** - Interactive sliding puzzle game
- **opengl_underlay** - OpenGL rendering demonstration
- **carousel** - 3D carousel interface
- **todo** - Task management application
- **weather-demo** - Weather information display (requires API key)

Run a specific demo by specifying it as a parameter to `docker run`, for example:

```plaintext
sudo docker run --rm --privileged --user=torizon \
  -v /dev:/dev -v /tmp:/tmp -v /run/udev:/run/udev \
  --device-cgroup-rule='c 199:* rmw' --device-cgroup-rule='c 226:* rmw' \
  --device-cgroup-rule='c 13:* rmw' --device-cgroup-rule='c 4:* rmw' \
  ghcr.io/slint-ui/slint/torizon-demos-arm64-imx8 printerdemo
```

### Microcontrollers

Slint’s platform abstraction allows for integration into any Rust or C++ based Microcontroller development environment. Developers need to implement functionality to feed input events such as touch or keyboard, as well as displaying the pixels rendered by Slint into a frame- or linebuffer.

**C++**

We provide templates for a few off-the-shelf development boards from some of the silicon vendors.

**Rust**

You will need to use the `mcu-board-support` crate. This crate re-export a `entry` attribute macro to apply to the `main` function, and a `init()` function that should be called before creating the Slint UI.

In order to use this backend, the final program must depend on both `slint` and `mcu-board-support`. The main.rs will look something like this

```rust
#![no_std]
#![cfg_attr(not(feature = "simulator"), no_main)]
slint::include_modules!();

#[allow(unused_imports)]
use mcu_board_support::prelude::*;

#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();
    MainWindow::new().run();
    panic!("The event loop should not return");
}
```

Since mcu-board-support is at the moment an internal crate not uploaded to crates.io, you must use the git version of slint, slint-build, and mcu-board-support

```toml
[dependencies]
slint = { git = "https://github.com/slint-ui/slint", default-features = false }
mcu-board-support = { git = "https://github.com/slint-ui/slint" }
# ...
[build-dependencies]
slint-build = { git = "https://github.com/slint-ui/slint" }
```

In your build.rs, you must include a call to `slint_build::print_rustc_flags().unwrap()` to set some of the flags.

#### Espressif (ESP32)

**C++**

To use Slint with your C++ application, you can follow the instructions on the [Espressif Documentation site↗](https://components.espressif.com/components/slint/slint)

#### ST (STM32)

**C++**

##### STM32H735G-DK

You can start with the template [slint-cpp-template-stm32h735g-dk.zip](https://github.com/slint-ui/slint/releases/download/v1.16.1/slint-cpp-template-stm32h735g-dk.zip)

##### STM32H747I-DISCO

You can start with the template [slint-cpp-template-stm32h747i-disco.zip](https://github.com/slint-ui/slint/releases/download/v1.16.1/slint-cpp-template-stm32h747i-disco.zip)

**Rust**

Follow the steps below to run the Slint Printer Demo

##### STM32H735G-DK

Using [probe-rs↗](https://probe.rs/).

```sh
CARGO_PROFILE_RELEASE_OPT_LEVEL=s CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER="probe-rs run --chip STM32H735IGKx" cargo run -p printerdemo_mcu --no-default-features  --features=mcu-board-support/stm32h735g --target=thumbv7em-none-eabihf --release
```

#### Raspberry Pi (Pico)

Only Rust programs are currently supported on the Raspberry Pi Pico.

##### On the Raspberry Pi Pico

Ensure the right target is set:

```sh
rustup target add thumbv6m-none-eabi
```

Build the Slint Printer demo with:

```sh
cargo build -p printerdemo_mcu --no-default-features --features=mcu-board-support/pico-st7789 --target=thumbv6m-none-eabi --release
```

The resulting file can be flashed with [elf2uf2-rs↗](https://github.com/jonil/elf2uf2-rs). Install it using:

```sh
cargo install elf2uf2-rs
```

**macOS**

Now power off the Pico and connect it while holding down the “bootsel” button. The device will show up as a storage device with the name `RPI-RP2`.

Then flash the demo to the Pico with:

```sh
elf2uf2-rs -d target/thumbv6m-none-eabi/release/printerdemo_mcu
```

When the flashing completes the Pico will reboot and show the Slint Printer demo. The Mac will warn the drive was unmounted unexpectedly. This is expected and can be ignored.

**Linux**

Now power off the Pico and connect it while holding down the “bootsel” button. The device will show up as a storage device.

Mount the device:

```sh
udisksctl mount -b /dev/sda1
```

Then flash the demo to the Pico with:

```sh
elf2uf2-rs -d target/thumbv6m-none-eabi/release/printerdemo_mcu
```

##### On the Raspberry Pi Pico2

Build the Slint Printer demo with:

```sh
cargo build -p printerdemo_mcu --no-default-features --features=mcu-board-support/pico2-st7789 --target=thumbv8m.main-none-eabihf --release
```

The resulting file can be flashed conveniently with [picotool↗](https://github.com/raspberrypi/picotool). You should build it from source.

Then upload the demo to the Raspberry Pi Pico: push the “bootsel” white button on the device while connecting the micro-usb cable to the device, this connects some USB storage on your workstation where you can store the binary.

Or from the command on linux (connect the device while pressing the “bootsel” button):

```sh
udisksctl mount -b /dev/sda1
picotool load -u -v -x -t elf target/thumbv8m.main-none-eabihf/release/printerdemo_mcu
```

##### Using probe-rs

This requires [probe-rs↗](https://probe.rs/) and to connect the pico via a probe (for example another pico running the probe).

Then you can simply run with `cargo run`

```sh
CARGO_TARGET_THUMBV6M_NONE_EABI_LINKER="flip-link" CARGO_TARGET_THUMBV6M_NONE_EABI_RUNNER="probe-rs run --chip RP2040" cargo run -p printerdemo_mcu --no-default-features --features=mcu-board-support/pico-st7789 --target=thumbv6m-none-eabi --release
```

##### Flashing and Debugging the Pico with `probe-rs`’s VSCode Plugin

Install `probe-rs-debugger` and the VSCode plugin as described [here↗](https://probe.rs/docs/tools/vscode/).

Add this build task to your `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "cargo",
      "command": "build",
      "args": [
        "--package=printerdemo_mcu",
        "--features=mcu-pico-st7789",
        "--target=thumbv6m-none-eabi",
        "--profile=release-with-debug"
      ],
      "problemMatcher": [
        "$rustc"
      ],
      "group": "build",
      "label": "build mcu demo for pico"
    },
  ]
}
```

The `release-with-debug` profile is needed, because the debug build does not fit into flash.

You can define it like this in your top level `Cargo.toml`:

```toml
[profile.release-with-debug]
inherits = "release"
debug = true
```

Now you can add the launch configuration to `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "preLaunchTask": "build mcu demo for pico",
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Flash and Debug MCU Demo",
            "cwd": "${workspaceFolder}",
            "connectUnderReset": false,
            "chip": "RP2040",
            "flashingConfig": {
                "flashingEnabled": true,
                "resetAfterFlashing": true,
                "haltAfterReset": true
            },
            "coreConfigs": [
                {
                    "coreIndex": 0,
                    "rttEnabled": true,
                    "programBinary": "./target/thumbv6m-none-eabi/release-with-debug/printerdemo_mcu"
                }
            ]
        },
    ]
}
```

This was tested using a second Raspberry Pi Pico programmed as a probe with [DapperMime↗](https://github.com/majbthrd/DapperMime).

### Other Platforms

[Contact us↗](https://slint.dev/contact) if you want to use Slint on other platforms/versions.

## Android

Source: `guide/platforms/mobile/android/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/android/

> **Note**
> When developing Slint applications for Android, you can only use Rust as the programming language.

Also see the documentation of the [android module in our Rust API documentation](https://docs.slint.dev/latest/docs/rust/slint/android/).

### Project Setup

Slint uses the [android-activity crate↗](https://github.com/rust-mobile/android-activity) as the interface to the operating system, which is re-exported as `slint::android::android_activity`. To get started, follow these steps:

First, your project needs to be a library crate. Add the following to your `Cargo.toml`:

You also need to select the version of android-activity you want to use:

```toml
[dependencies]
slint = { version = "1.15.0", features = ["backend-android-activity-06"] }
```

This feature compiles with any target_os and can safely be enabled anywhere.

Second, in your `lib.rs`, add this function:

```rs
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let main_window = ...; // window generated by the Slint macros
    main_window.run().unwrap();
}
```

You can also add an Android event ([`android_activity::PollEvent`↗](https://docs.rs/android-activity/latest/android_activity/enum.PollEvent.html)) listener by replacing the call to `slint::android::init` with `slint::android::init_with_event_listener`.

That’s all of the necessary code changes. In the next section, we’re going to set up the environment to build the project.

### Android Setup

The Android development workflow centers around the `adb` command line tool. Use it to connect to Android devices and emulators to upload and run applications (and do other things not relevant here).

The easiest way to install the Android development environment is to download and install [Android Studio↗](https://developer.android.com/studio). In the settings pane, navigate to the Android SDK page and install all SDK versions you need. We recommend to use the latest version available, because it can be configured to be backwards-compatible with older versions of Android. This manager is available in the settings in “Languages & Frameworks” > “Android SDK”.

![Screenshot Android SDK Manager](https://docs.slint.dev/latest/docs/slint/_astro/android_sdk_manager.BcxeiT4p_1LXMF1.webp)

## General Mobile Development

Source: `guide/platforms/mobile/general/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/general/

> **Note**
> When developing Slint applications for Android or iOS, you can only use Rust as the programming language for now.

While Slint is used in the same way for mobile and desktop applications, there are a few things that have to be kept in mind during development. Mobile platforms usually feature a small screen and no hardware keyboard, which have direct consequences that can’t be handled by the user interface framework, but are specific to the application. Touch interfaces also need special handling as opposed to mouse-based interaction.

This page describes considerations that apply to all mobile platforms. The next pages dive into platform-specific issues on top of these.

### Scrolling

By default, Slint [ScrollView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/scrollview/index.html)s only scroll by dragging the scrollbar or the mouse wheel. This is fine for desktop, but touch interfaces are expected to scroll while panning the whole view.

Enable it in Slint like this:

Note that this also enables the behavior when a mouse is attached, which might not be desired. This will be fixed in a future version of Slint.

### Safe Area

Operating system developers for mobile devices try to improve the limitations of the small screen by reducing the amount of system-level overlays to a bare minimum. This means that applications usually have access to the entire screen, but there are a few system items overlaid, for example the cell tower reception and battery status. On Android, this also includes navigation buttons. Also, many cellphone devices embed the front-facing camera into the display, leaving only a small non-rectangular area inaccessible (“the notch”).

Apps can usually draw onto the entire screen and underneath certain overlay areas, but user interaction is not allowed everywhere. For example, buttons drawn underneath the front-facing camera area can’t be tapped. Therefore, Android and iOS describe what’s called a “safe area”: A rectangular region where an application can expect users to be able to interact with interface elements. It is defined by an inset, meaning that it defines the thickness of an invisible border inside the application’s window which should only contain background elements (like background images, patterns, color). This safe area can also change in size at runtime, for example on Android when the navigation buttons at the bottom are shown and hidden.

In Slint, this area is exposed with the [Window.safe-area-insets](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html#safe-area-insets) property on the [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html) element.

If you want to place a rectangle to visualize the safe area, for example for debugging, you can do it like this:

```slint
export component MainWindow inherits Window {
    Rectangle {
        background: yellow;
        x: root.safe-area-insets.left;
        y: root.safe-area-insets.top;
        width: root.width - root.safe-area-insets.right - root.safe-area-insets.left;
        height: root.height - root.safe-area-insets.bottom - root.safe-area-insets.top;
    }
}
```

### Keyboard Handling

Since the introduction of the iPhone, modern smartphones usually don’t feature a hardware keyboard for text input. There are accessories like Bluetooth or USB keyboards and special phone cases that do provide such a feature, but there always has to be a software fallback, the “virtual keyboard”.

Note that the keyboard is not always comprised of rows of buttons, it can also be a handwriting recognition area, voice input, or a camera viewer for capturing text or bar codes that are then inserted. All of these are handled transparently though, so the application doesn’t have to care about that, unless the application provides a virtual keyboard itself.

The virtual keyboard is placed on top of the application that requires text input, reducing the already limited screen space even further. While there are split keyboards, floating keyboards, and more, operating systems always treat the virtual keyboard as a single rectangle overlaying the application window.

The way Slint exposes this is by these two properties defining a rectangle on the [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html) element:

- [Window.virtual-keyboard-position](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html#virtual-keyboard-position)
- [Window.virtual-keyboard-size](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html#virtual-keyboard-size)

Slint calls on the operating system to open and hide the virtual keyboard transparently when focusing a text input element. The operating system decides whether to actually show it, which depends on system settings and whether a hardware keyboard is detected. This might also change at runtime, for example if the user attaches a hardware keyboard while the virtual keyboard is visible (in this case, the operating system might hide the virtual keyboard by itself). In any case, if there is a non-floating virtual keyboard visible, Slint exposes this to the application.

You can detect if the keyboard is shown by checking whether `virtual-keyboard-width * virtual-keyboard-height` is greater than 0.

Some virtual keyboards are translucent, which means that the area below the keyboard can be visible to some extend. So, the application can display basic visual elements like a solid color there to let the keyboard appear to be integrated into the visual style.

Usually, applications don’t have to use these properties, due to the behavior explained in the next section.

> **Note**
> The way the virtual keyboard overlay is defined differs between Android and iOS. Android defines an inset of the window (just like the safe area definition above) while iOS supplies a rectangle. Slint unifies this to behave the same on both platforms, but since these are not mathematically equivalent, there might be certain edge cases with custom virtual keyboards that might not be handled correctly. It is expected that most keyboards just take up a section of the bottom of the screen.

#### Keeping the Editing Area Visible

A common issue is that the text field the user wants to edit ends up below the virtual keyboard, making it invisible. Slint can automatically handle this by scrolling, but the application has to be structured in a certain way for this to work.

When the keyboard is shown, the element currently in focus tries to stay visible. It does that by searching for a scrollable area ([Flickable](https://docs.slint.dev/latest/docs/slint/reference/gestures/flickable/index.html)) in its parent element chain (up to the window). If it finds one, that area is instructed to scroll in a way that it doesn’t overlap the keyboard with the minimal offset possible. The bounds check of the scroll area is reduced by the overlap with the keyboard, which means that the area can be scrolled further than normal. This overscroll is automatically fixed when the keyboard is hidden again.

So to summarize, if you want to keep a text field in view while the virtual keyboard is shown, put it inside a scroll area like this:

```slint
import { LineEdit, ScrollView } from "std-widgets.slint";
export component MyView {
    ScrollView {
        mouse-drag-pan-enabled: true;

        VerticalLayout {
            label := Text {
                text: "Text input";
                horizontal-alignment: left;
                overflow: elide;
            }

            LineEdit {
                placeholder-text: "Write your text";
                accessible-label: label.text;
            }
        }
    }
}
```

As you can see, the [ScrollView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/scrollview/index.html) doesn’t need to be the direct parent of the [LineEdit](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/lineedit/index.html). Note that having multiple nested `ScrollView`s is not supported.

### Application Layout Considerations

While stylus and mouse input can be pixel-accurate, touch input is not, especially with capacitive touch screens. So, touch-ready applications have to take care to make interactive areas as large as possible and there has to be enough spacing between touch areas. Also, certain parts of the screen are more easily accessible with the thumb of the hand that holds the device than others.

Slint is not able to provide any help with this aspect, so you as the application developers have to keep this in mind, especially while developing using a device simulator or in a UI design application like Figma.

We highly recommend reading Apple’s [Human Interface Guidelines↗](https://developer.apple.com/design/human-interface-guidelines/) and Google’s [Design for Android↗](https://developer.android.com/design/ui) page.

## iOS

Source: `guide/platforms/mobile/ios/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/ios/

> **Note**
> When developing Slint applications for iOS, you can only use Rust as the programming language.

A Rust-based Slint application can be cross-compiled to iOS and runs on iPhones, iPads, and their respective simulators. This is implemented through the [Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/index.html) and the [Skia Renderer](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backends_and_renderers/index.html#skia-renderer).

### Prerequisites

- A computer running macOS.
- An up-to-date installation of [Xcode↗](https://developer.apple.com/xcode/) .
- [Xcodegen↗](https://github.com/yonaskolb/XcodeGen?tab=readme-ov-file#installing) .
- [Rust↗](https://rustup.rs/)
- The Rust device and simulator toolchains. Run `rustup target add aarch64-apple-ios` and `rustup target add aarch64-apple-ios-sim` to add them.

### Adding iOS Support to an existing Rust Application

The following steps assume that you have a Rust application with Slint prepared. If you’re just getting started, use our [Slint Rust Template↗](https://github.com/slint-ui/slint-rust-template) to get a minimal application running.

Use XCode to building, deploy, and submit iOS applications to the App Store. Use [Xcodegen↗](https://github.com/yonaskolb/XcodeGen) to create an Xcode project from a minimal description.

1. Verify that your application compiles for iOS, by running:

1. Create a file called `project.yml` with the following contents:

```yml
name: My App
options:
  bundleIdPrefix: com.company
settings:
  ENABLE_USER_SCRIPT_SANDBOXING: NO
targets:
  MyApp:
    type: application
    platform: iOS
    deploymentTarget: "12.0"
    info:
        path: Info.plist
        properties:
            UILaunchScreen:
                - ImageRespectSafeAreaInsets: false
    sources: []
    postCompileScripts:
      - script: |
          ./build_for_ios_with_cargo.bash slint-rust-template
        outputFileLists:
            $TARGET_BUILD_DIR/$EXECUTABLE_PATH
```

Adjust the name, bundle id, and other fields as needed.

This configuration file delegates the build process to cargo through a shell script.

> **Note**
> The shell script is invoked with the name of the binary that cargo produces. Update it to match the name of your project.

1. In a new file called `build_for_ios_with_cargo.bash` , paste the following script code:

1. Make the script executable with `chmod +x build_for_ios_with_cargo.bash`.
2. Run `xcodegen` to create `My App.xcodeproj`, and open it in Xcode. Now you can build, deploy, and debug your iOS application.

![Screenshot Slint Template running in iOS Simulator](https://docs.slint.dev/latest/docs/slint/_astro/ios-simulator.CxMqzxeM_Z1ijnSW.webp)

## Other Platforms

Source: `guide/platforms/other/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/other/

[Contact us↗](https://slint.dev/contact) if you want to use Slint on other platforms/versions.

## Web

Source: `guide/platforms/web/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/platforms/web/

> **Caution**
> Only Rust supports using Slint with WebAssembly.

Slint applications run on desktop, embedded, and mobile platforms. Slint applications written with Rust can also be cross-compiled to WebAssembly (Wasm) and run in the web browser.

> **Note**
> Slint cross-compiled for Wasm uses the [Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/index.html) with the FemtoVG renderer.

Slint renders your UI into a HTML `<canvas>` element using [WebGL↗](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API), without using the DOM or CSS. This results in a consistent look across platforms, but also introduces some limitations:

- Slint renders text directly, instead of benefitting from the browser’s text rendering.
- Accessibility features (such as screen readers) are not available.
- The UI does not behave like a typical web application, since it doesn’t use standard HTML elements.

Because of these trade-offs, running Slint in the browser is currently not recommended for building general-purpose web applications. Instead, it is best suited for:

- Demos and examples that can run directly in the browser without requiring installation.
- Applications where the web is not the primary platform, but a consistent UI is still needed.
- Tools or dashboards where native-style rendering is more important than web integration.

### Building for Wasm

For a step-by-step walkthrough, check out the last chapter of the [quickstart](https://docs.slint.dev/latest/docs/slint/tutorial/quickstart/index.html).

Below is a summary of the main steps:

In your `Cargo.toml`, set the crate type to `"cdylib"` and add [`wasm-bindgen`↗](https://wasm-bindgen.github.io/wasm-bindgen/) as a dependency for the “wasm” target:

Use the `wasm-bindgen(start)` attribute to mark the application’s entry point. The UI is created and run as usual:

```rust
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

slint::include_modules!(); // or slint!(...)

#[cfg_attr(target_family = "wasm", wasm_bindgen(start))]
pub fn main() {
    // Usual application code
    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();
}
```

Build the application using [wasm-pack↗](https://drager.github.io/wasm-pack/).

```bash
wasm-pack build --release --target web
```

This creates a `pkg/` directory with `.wasm` and `.js` files, including a JavaScript file named after your package.

Import the wasm binary in your HTML file. Slint expects a `<canvas>` HTML element with `id = "canvas"`.

```html
<canvas id="canvas"></canvas>
<script type="module">
  import init from "./pkg/YOUR_APPLICATION.js";
  init();
</script>
```

Replace `YOUR_APPLICATION` with the name of your crate.

> **Note**
> Many web browser load `.wasm` files only in trusted contexts. If during development you observe the browser producing permission errors, then you may need to serve files through a web server, instead of the file system directly.

## Figma Variable Export

Source: `guide/tooling/figma-inspector/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/figma-inspector/

### Introduction

The “Figma to Slint” plugin bridges the gap between Figma designs and Slint user interfaces. It provides two primary functionalities:

1. **Object Inspection:** Allows developers to select any object on the Figma canvas and view its properties (like dimensions, colors, fonts, etc.)formatted as a Slint code snippet. This is useful for quickly translating visual design elements into Slint code. When paired with the variable export, this inserts variables into the code, if they are used in the Figma file.
2. **Variable Export:** Enables the reliable export of design tokens (variables for colors, numbers, strings, and booleans) defined in Figma directly into `.slint` files. This feature fully supports Figma’s modes, allowing for themeable designs (e.g., light and dark modes) and generates the necessary Slint structs, enums, and global instances to use these tokens seamlessly in your Slint application.

### Setting up the Figma Inspector

First of all, find and enable the “Figma to Slint” plugin in the Figma Plugin Manager. The inspector currently has 2 functions. One is to inspect individual Figma objects for their properties, and the other is to export the variables defined in Figma so they are useable in Slint directly.

### Key Features of Variable Export

- **Type Conversion:** The plugin maps Figma variable types to their corresponding Slint types as follows: | Figma Variable Type | Slint Type | | --- | --- | | Color | `brush` | | Float | `length` | | String | `string` | | Boolean | `bool` |
- **Mode Support:** For a figma file where Mode `MyCollection` has modes `light` and `dark`, the exporter will create a Slint `enum` for mode selection (e.g., `MyCollectionMode { light, dark }`). The file also contains a `Scheme` struct (`MyCollection-Scheme`) representing the structure of variables. A `Scheme-Mode` struct (`MyCollection-Scheme-Mode`) holds instances of the `Scheme` for each mode. And finally a global instance (`my_collection`) which contains:
  - `mode` : An instance of `Scheme-Mode` holding the resolved values for each mode.
  - `current-mode` : An `in-out` property using the mode `enum` to control the active mode.
  - `current` : An `out` property dynamically selecting the correct scheme instance based on `current-mode` .
- **Reliable Variable Resolution:** Automatically resolves all variable aliases (references) to their concrete values, ensuring consistent and predictable output regardless of how variables reference each other.
- **Hierarchy Generation:** Interprets `/` in Figma like `colors/background/primary` to generate nested Slint structs so the corresponding variables are nested for dot notation in Slint, thus the above example turns into `colors.background.primary`
- **Robust Mode Handling:** Intelligently matches Figma modes to ensure all variables export with their correct values, even when mode IDs don’t match perfectly between collections and variables.
- **Name Sanitization:** Cleans up collection, variable, and mode names to be valid Slint identifiers (e.g., converting spaces and special characters to underscores, handling leading digits).
- **Export Options:**
  - **Separate Files:** Exports each Figma collection into its own `.slint` file. Creates `<collection_name>.slint` files with automatic cross-collection imports when needed.
  - **Single File:** Combines all collections into a single `design-tokens.slint` file for simpler project integration.
- **README Generation:** Creates a `README.md` file alongside the export, summarizing exported collections, renamed variables, and any warnings.

### Usage

1. **Inspector:**
  - Bring up the plugin UI and select any object you’d like to inspect. ![alt text](https://docs.slint.dev/latest/docs/slint/_astro/inspect.B2uspMyd_ZLP0QU.webp)
2. **Variables:**
  - To see and export variables, check the “Use Figma Variables” checkbox in the plugin UI. ![alt text](https://docs.slint.dev/latest/docs/slint/_astro/use-variables.Dh-vJAkC_MBMhp.webp)
  - Now when selecting objects to inspect you should see variable names instead of resolved values (if they are assigned) ![alt text](https://docs.slint.dev/latest/docs/slint/_astro/inspect-variables.Dl9R47i0_Z1pCke5.webp)
3. **Export:** Click the “Export” button and choose: ![alt text](https://docs.slint.dev/latest/docs/slint/_astro/export-variables.B6y5Rfj2_ZFdXLM.webp)
  - `Separate Files Per Collection…` : Recommended for organization. Creates `<collection_name>.slint` files.
  - `Single Design-Tokens File…` : Creates `design-tokens.slint` for simpler project integration.
4. **Integrate:**
  - Place the generated `.slint` file(s) in your Slint project.

### Design-Token File Structure

1. **Import** ```slint // If single file: import { Colors, Spacing } from "design-tokens.slint"; ```
2. **Use variables** ```slint // Use the tokens: MyComponent := Rectangle { background: Colors.current.background.primary; // Access via .current for mode switching height: Spacing.medium; // Access directly if single-mode or not mode-dependent } ```
3. **Mode Switching:** (For multi-mode collections) Modify the `current-mode` property of the imported global: ```slint // Example: In your main component's logic init => { // Set initial mode Colors.current-mode = ColorsMode.dark; } // Or in response to a user action: clicked => { Colors.current-mode = (Colors.current-mode == ColorsMode.light) ? ColorsMode.dark : ColorsMode.light; } ```
  - All properties accessed via `<collection_name>.current.*` will automatically update.

### Naming Conventions & Sanitization

- **Hierarchy:** Figma uses `/` in variable names (e.g., `radius/small`, `font/body/weight`). The export will use these to create nested Slint structs for code completion clarity.
- **Sanitization:** Collection names, variable names (path segments), and mode names are automatically sanitized: ##### Example Variable `Color & Shade (only #hex values)` in a collection named `Color Primitives` would get turned into `color-primitives.color_and_shade_only_hex_values`
  - Spaces and invalid characters ( `&` , `+` , `:` , `-` , etc.) are typically converted to `-` for collection/struct names and `_` for property/enum names.
  - Leading/trailing invalid characters are removed.
  - Names starting with a digit are prefixed with `_` or `m_` .
  - Any variable named exactly `mode` at the root of a collection is renamed to `mode-var` in the Slint output to avoid conflicts with the generated scheme `mode` property.

## Helix

Source: `guide/tooling/helix/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/helix/

To install the Slint Language server, check the [LSP Documentation](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp).

[Helix↗](https://helix-editor.com/) works out of the box without further configuration. To check if Helix detects Slint Language server successfully, run this command:

The output should be like:

```plaintext
Configured language servers:
  ✓ slint-lsp: /home/user/.local/bin/slint-lsp
Configured debug adapter: None
Configured formatter: None
Highlight queries: ✓
Textobject queries: ✓
Indent queries: ✓
```

#### Live Preview

To open the live preview, place the caret over a component name and trigger the code actions (bound to `<space>a` by default). Depending on your configuration, this action might be bound to something else, so please check your configuration for the appropriate key binding (`code_action`).

![Opening a live Preview from Helix](https://docs.slint.dev/latest/docs/slint/_astro/helix-show-preview.TdChPpU4_Z2dWUPP.webp)

## JetBrains IDE

Source: `guide/tooling/jetbrains-ide/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/jetbrains-ide/

[kizeevov/slint-idea-plugin↗](https://github.com/kizeevov/slint-idea-plugin) has a plugin for the Intellij platform.

*Note: This plugin is developed by [@kizeevov↗](https://github.com/kizeevov).*

## Kate

Source: `guide/tooling/kate/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/kate/

Before we start, it’s important to note that Kate relies on the presence of syntax highlighting file for the usage of the LSP. Therefore, we’ll set up the syntax highlighting first.

#### Syntax Highlighting

The file [slint.ksyntaxhighlighter.xml↗](https://github.com/slint-ui/slint/blob/master/editors/kate/slint.ksyntaxhighlighter.xml) needs to be copied into a location where Kate can find it. See the [kate documentation↗](https://docs.kde.org/stable5/en/kate/katepart/highlight.html#katehighlight-xml-format)

On Linux, this can be done by running this command

On Windows, download [slint.ksyntaxhighlighter.xml↗](https://github.com/slint-ui/slint/blob/master/editors/kate/slint.ksyntaxhighlighter.xml) into `%USERPROFILE%\AppData\Local\org.kde.syntax-highlighting\syntax`

#### LSP

After setting up the syntax highlighting, you can now install the Slint Language server. Check the [LSP Documentation](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp) for instructions.

Once it is installed, go to *Settings > Configure Kate*. In the *Plugins* section, enable the *LSP-Client* plugin. This will add a *LSP Client* section in the settings dialog. In that *LSP Client* section, go to the *User Server Settings*, and enter the following in the text area:

```json
{
  "servers": {
    "Slint": {
      "path": ["%{ENV:HOME}/.cargo/bin", "%{ENV:USERPROFILE}/.cargo/bin"],
      "command": ["slint-lsp"],
      "highlightingModeRegex": "Slint"
    }
  }
}
```

![Kate LSP Setup](https://docs.slint.dev/latest/docs/slint/_astro/kate-lsp-setup.ChKVN83j_DJcUa.webp)

#### Live Preview

Once the LSP is correctly set up, to preview a component, first, position your cursor on the name definition of the component you want to preview (for instance, `MainWindow` in `component MainWindow inherits Window {`). Then, activate the *Show Preview* code action. You can do this by using the Alt+Enter shortcut to bring up the code action menu, or find it in the menu bar at *LSP Client > Code Action > Show Preview*

## Manual Setup

Source: `guide/tooling/manual-setup/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/

We provide extensions or configuration files for different editors to better support .slint files.

#### Editors

- [Visual Studio Code](https://docs.slint.dev/latest/docs/slint/guide/tooling/vscode/index.html)
- [Kate](https://docs.slint.dev/latest/docs/slint/guide/tooling/kate/index.html)
- [Qt Creator](https://docs.slint.dev/latest/docs/slint/guide/tooling/qt-creator/index.html)
- [Helix](https://docs.slint.dev/latest/docs/slint/guide/tooling/helix/index.html)
- [(Neo-)Vim](https://docs.slint.dev/latest/docs/slint/guide/tooling/neo-vim/index.html)
- [Sublime Text](https://docs.slint.dev/latest/docs/slint/guide/tooling/sublime-text/index.html)
- [JetBrains IDE](https://docs.slint.dev/latest/docs/slint/guide/tooling/jetbrains-ide/index.html)
- [Zed](https://docs.slint.dev/latest/docs/slint/guide/tooling/zed/index.html)

If your favorite editor is not in the list of supported editors, it just means we did not test it, not that it doesn’t work. We do provide a language server for Slint that should work with most editor that supports the Language Server Protocol (LSP). If you do test your editor with it, we would be happy to accept a pull request that adds instructions here.

### Slint Language Server (`slint-lsp`)

Most modern editors use the [Language Server Protocol (LSP)↗](https://microsoft.github.io/language-server-protocol/) to add support for different programming languages. Slint provides an LSP implementation with the `slint-lsp` binary.

#### Installation

If you have Rust installed, you can install the binary by running the following command:

This makes the latest released version available in `$HOME/.cargo/bin`. If you would like to try a development version, you can also point `cargo install` to the git repository: for the released version. Or, to install the development version:

```sh
cargo install slint-lsp --git https://github.com/slint-ui/slint --force
```

Alternatively, you can download one of our pre-built binaries for Linux or Windows:

1. Go to [the latest Slint release↗](https://github.com/slint-ui/slint/releases/latest)
2. From “Assets” download either `slint-lsp-linux.tar.gz` for a Linux x86-64 binary or `slint-lsp-windows-x86_64.zip` for a Windows x86-64 binary.
3. Uncompress the downloaded archive into a location of your choice.

Make sure the required dependencies are found. On Debian-like systems install them with the following command:

```shell
sudo apt install -y build-essential libx11-xcb1 libx11-dev libxcb1-dev libxkbcommon0 libinput10 libinput-dev libgbm1 libgbm-dev
```

#### Editor configuration

Once you have `slint-lsp` installed, configure your editor to use the binary, no arguments are required. See our [list of editor configurations](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/#editors) for details. If your editor is not on this list, please refer to your editors documentation for details on how to set up language servers.

#### Formatting Slint files

Slint files can be auto-formatted with `slint-lsp` in the following ways:

- `slint-lsp format <path>` - reads the file and outputs the formatted version to stdout
- `slint-lsp format -i <path>` - reads the file and saves the output to the same file
- `slint-lsp format /dev/stdin` - using /dev/stdin you can achieve the special behavior of reading from stdin and writing to stdout

Note that `.slint` files are formatted, while `.md` and `.rs` files are searched for `.slint` blocks. All other files are left untouched.

If you have `slint-lsp` configured in your editor, you should be able to format .slint files via your editor as well.

### Slint Live Preview (`slint-viewer`)

Slint’s live preview feature lets you see your code changes in real-time. This is a really powerful way to iterate quickly on your UI without having to recompile anything.

If you have slint-lsp configured in your editor, you can launch the live preview directly from your editor.

**Example in Neovim:**

![Opening Live Preview from a Neovim Popup](https://docs.slint.dev/latest/docs/slint/_astro/nvim-show-preview.SDu5V0A7_dCVRL.webp)

#### Running `slint-viewer` from the terminal

To open the live preview from a terminal, you can use the `slint-viewer` binary.

To install it either:

- [Download the binary↗](https://github.com/slint-ui/slint/releases/latest)
- Run `cargo install slint-viewer` if you have a Rust installation

Then run `slint-viewer --auto-reload <path/to/file.slint>`.

**Example on the [Printer Demo↗](https://github.com/slint-ui/slint/blob/master/demos/printerdemo/ui/printerdemo.slint)**

![Slint Printer Demo](https://docs.slint.dev/latest/docs/slint/_astro/slint-viewer.ClU3OYkB_1LIhGk.webp)

### Additional resources

- [Slint Plugin for (Neo)vim↗](https://github.com/slint-ui/vim-slint)
- [Slint TreeSitter Grammar↗](https://github.com/slint-ui/tree-sitter-slint)
- [Editor Configuration Source↗](https://github.com/slint-ui/slint/tree/master/editors)

## (Neo-)Vim

Source: `guide/tooling/neo-vim/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/neo-vim/

### Vim

To install the Slint Language server, check the [LSP Documentation](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp).

Vim support the Language Server Protocol via its [Conquer of Completion↗](https://github.com/neoclide/coc.nvim) plugin. Together with the Slint LSP server, this enables inline diagnostics and code completion when editing `.slint` files.

After installing the extension, for example via [vim-plug↗](https://github.com/junegunn/vim-plug), two additional configuration changes are needed to integrate the LSP server with vim:

1. Make vim recognize the `.slint` files with the correct file type

Install the [`slint-ui/vim-slint`↗](https://github.com/slint-ui/vim-slint) plugin.

Alternatively you can add the following to your vim configuration file (e.g. `vimrc`) to enable automatic recognition of `.slint` files:

1. Make sure the slint language server is installed and can be found in PATH.
2. Configure Conquer of Completion to use the Slint LSP server

Start `vim` and run the `:CocConfig` command to bring up the buffer that allows editing the JSON configuration file (`coc-settings.json`), and make sure the following mapping exists under the `language` server section:

```json
{
  "languageserver": {
    "slint": {
      "command": "slint-lsp",
      "filetypes": ["slint"]
    }
  }
}
```

### Neovim

Follow step 1. of the Vim section to get support for `.slint` files.

The easiest way to use the language server in Neovim is via the [`neovim/nvim-lspconfig`↗](https://github.com/neovim/nvim-lspconfig) plugin.

To install the language server you can:

- [install and configure manually](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp)
- install [Mason↗](https://github.com/mason-org/mason.nvim) and run `:MasonInstall slint-lsp` (on Windows, Linux and macOS)

#### Live Preview

Once the `slint-lsp` language server is installed and running, you can open the live preview by placing the caret over a component name and triggering the code actions (bound to `gra` by default). Depending on your configuration, this action might be bound to something else, so please check your configuration for the appropriate key binding (`vim.lsp.buf.code_action()`).

You may also want to install a plugin that indicates when code actions are available like [kosayoda/nvim-lightbulb↗](https://github.com/kosayoda/nvim-lightbulb).

**Example with nvim-lightbulb**:

![Opening Live Preview from Neovim](https://docs.slint.dev/latest/docs/slint/_astro/nvim-show-preview.SDu5V0A7_dCVRL.webp)

#### Tree-sitter

If you use `nvim-treesitter` you can install the Tree Sitter parser for Slint using `TSInstall slint` for syntax highlighting and indentation support.

## Qt Creator

Source: `guide/tooling/qt-creator/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/qt-creator/

#### Syntax Highlighting

For the **syntax highlighting**, QtCreator supports the same format as Kate, with the [xml file↗](https://github.com/slint-ui/slint/blob/master/editors/kate/slint.ksyntaxhighlighter.xml) at the same location. Refer to the instruction from the [Kate page](https://docs.slint.dev/latest/docs/slint/guide/tooling/kate/index.html#syntax-highlighting) to enable syntax highlighting.

#### LSP

To install the Slint Language server, check the [LSP Documentation](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp).

To setup the lsp:

1. Install the `slint-lsp` binary
2. Then in Qt creator, go to *Tools > Option* and select the *Language Client* section.
3. Click *Add*
4. As a name, use “Slint”
5. use `*.slint` as a file pattern. (don’t use MIME types)
6. As executable, select the `slint-lsp` binary (no arguments required)
7. Click *Apply* or *Ok*

![Qt Creator LSP Configuration](https://docs.slint.dev/latest/docs/slint/_astro/qt-creator-lsp-setup.CKpLTNKg_2lkOP.webp)

#### Live Preview

Once you have set up the LSP, in order to **preview a component**, when you have a .slint file open, place your cursor to the name of the component you would like to preview and press *Alt + Enter* to open the code action menu. Select *Show Preview* from that menu.

## Sublime Text

Source: `guide/tooling/sublime-text/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/sublime-text/

To install the Slint Language server, check the [LSP Documentation](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp).

To setup the LSP:

1. Make sure the slint language server is installed
2. Using Package Control in Sublime Text, install the LSP package ( [sublimelsp/LSP↗](https://github.com/sublimelsp/LSP) )
3. Download the Slint syntax highlighting files into your User Package folder, e.g. on macOS `~/Library/Application Support/Sublime Text/Packages/User/` : [https://raw.githubusercontent.com/slint-ui/slint/master/editors/sublime/Slint.sublime-syntax↗](https://raw.githubusercontent.com/slint-ui/slint/master/editors/sublime/Slint.sublime-syntax) [https://raw.githubusercontent.com/slint-ui/slint/master/editors/sublime/Slint.tmPreferences↗](https://raw.githubusercontent.com/slint-ui/slint/master/editors/sublime/Slint.tmPreferences)
4. Download the LSP package settings file into your User Package folder: [https://raw.githubusercontent.com/slint-ui/slint/master/editors/sublime/LSP.sublime-settings↗](https://raw.githubusercontent.com/slint-ui/slint/master/editors/sublime/LSP.sublime-settings)
5. Modify the slint-lsp command path in `LSP.sublime-settings` to point to the cargo installation path in your home folder ( **Replace YOUR_USER by your username** ): `"command": ["/home/YOUR_USER/.cargo/bin/slint-lsp"]`
6. Run “LSP: Enable Language Server Globally” or “LSP: Enable Language Server in Project” from Sublime’s Command Palette to allow the server to start.
7. Open a .slint file - if the server starts its name will be in the left side of the status bar.

#### Live Preview

In order to **preview a component**, when you have a .slint file open, place your cursor to the name of the component you would like to preview and select the “Show preview” button that will appear on the right of the editor pane.

## Visual Studio Code

Source: `guide/tooling/vscode/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/vscode/

[YouTube video](https://www.youtube.com/watch?v=fWwctiowLnY)

If you are new to Slint and want to quickly get started and learn the basics, we recommend using Visual Studio Code (VS Code). VS Code is popular, free and thanks to the Slint extension it’s also the easiest to get started with.

> **Note**
> We support many other tools and editors, see [here](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html).

### Setting Up VS Code

1. **Install VS Code.** Download it [here↗](https://code.visualstudio.com/).
2. **Install the Slint extension.** Find it [here↗](https://marketplace.visualstudio.com/items?itemName=Slint.slint).
3. **Create a new project based on a Slint template.** This is done via the command palette (CTRL+Shift+P) or on MacOS (CMD+Shift+P). ![Command palette](https://docs.slint.dev/latest/docs/slint/_astro/macos-vscode-template-1.C0N31v8__ZyyUCP.webp)
4. **Choose your language.** ![Command palette](https://docs.slint.dev/latest/docs/slint/_astro/macos-vscode-template-3.vKZBp4ej_Z21HCvA.webp)
5. **Choose a folder to save the project in.** ![Project folder](https://docs.slint.dev/latest/docs/slint/_astro/macos-vscode-template-2.5omuz7Qq_1KVu5A.webp)
6. **Name the project.** Give the project a name and now a new project will be created in the selected folder based on a simple template to get you started.

## Zed

Source: `guide/tooling/zed/`
Official URL: https://docs.slint.dev/latest/docs/slint/guide/tooling/zed/

[Zed↗](https://zed.dev/) is a high-performance, multiplayer code editor. The [zed-slint extension↗](https://github.com/slint-ui/slint/tree/master/editors/zed), originally developed and donated by Luke Jones, now lives under the slint organization. It integrates the latest release of the [slint language server](https://docs.slint.dev/latest/docs/slint/guide/tooling/manual-setup/index.html#slint-lsp) into Zed, offering code completion and syntax highlighting. Install the extension via the following steps:

1. Open the extensions tab via the Zed -> Extensions menu.
2. In the search field, enter “slint”.
3. Click on “Install” for the “Slint” extension.

# Reference

## Colors & Brushes

Source: `reference/colors-and-brushes/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/

Color literals follow the syntax of CSS:

In addition to plain colors, many elements have properties that are of type `brush` instead of `color`. A brush is a type that can be either a color or gradient. The brush is then used to fill an element or draw the outline.

CSS Color names are only in scope in expressions of type `color` or `brush`. Otherwise, access colors from the `Colors` namespace.

### Color Properties

The following properties are exposed:

#### red

#### green

#### blue

#### alpha

These properties are in the range 0-255.

Use the colors namespace to select colors by their name. For example you can use `Colors.aquamarine` or `Colors.bisque`. The entire list of names is very long. You can find a complete list in the [CSS Specification↗](https://www.w3.org/TR/css-color-3/#svg-color).

These color names are available in scope of `color` and `brush` expressions, as well as in the `Colors` namespace.

```slint
// Using the Colors namespace
background: Colors.aquamarine;

// Using the functions via global scope.
background: aquamarine;
```

### Global Color Functions

#### rgb(int, int, int) -> color

#### rgba(int, int, int, float) -> color

Return the color as in CSS. Like in CSS, these two functions are actually aliases that can take three or four parameters.

The first 3 parameters can be either number between 0 and 255, or a percentage with a `%` unit. The fourth value, if present, is an alpha value between 0 and 1.

Unlike in CSS, the commas are mandatory.

#### hsv(h: float, s: float, v: float) -> color

#### hsv(h: float, s: float, v: float, a: float) -> color

Returns a color using HSV (Hue, Saturation, Value) coordinates. The hue parameter is a float representing degrees (0-360) and wraps around (e.g., 480 becomes 120). The saturation, value, and optional alpha parameter are expected to be within the range of 0 and 1.

#### oklch(l: float, c: float, h: float) -> color

#### oklch(l: float, c: float, h: float, a: float) -> color

Returns a color using the [Oklch color space↗](https://en.wikipedia.org/wiki/Oklab_color_space) (a perceptually uniform color space).

- `l` (lightness): 0 (black) to 1 (white), or 0% to 100%
- `c` (chroma): 0 (grayscale) to ~0.4 (vivid), or 0% to 100% (where 100% = 0.4)
- `h` (hue): 0-360 degrees, or as an angle (e.g., `180deg` , `0.5turn` )
- `a` (alpha): 0-1, defaults to 1

### Color Methods

All colors and brushes define the following methods:

#### brighter(factor: float) -> brush

Returns a new color derived from this color but has its brightness increased by the specified factor. This is done by converting the color to the HSV color space and multiplying the brightness (value) with (1 + factor). For example if the factor is 0.5 (or for example 50%) the returned color is 50% brighter. Negative factors decrease the brightness.

#### darker(factor: float) -> brush

Returns a new color derived from this color but has its brightness decreased by the specified factor. This is done by converting the color to the HSV color space and dividing the brightness (value) by (1 + factor). For example if the factor is .5 (or for example 50%) the returned color is 50% darker. Negative factors increase the brightness.

#### mix(other: brush, factor: float) -> brush

Returns a new color that is a mix of this color and `other`. The specified factor is clamped to be between `0.0` and `1.0` and then applied to this color, while `1.0 - factor` is applied to `other`. For example `red.mix(green, 70%)` will have a stronger tone of red, while `red.mix(green, 30%)` will have a stronger tone of green.

#### transparentize(factor: float) -> brush

Returns a new color with the opacity decreased by `factor`. The transparency is obtained by multiplying the alpha channel by `(1 - factor)`.

#### with-alpha(alpha: float) -> brush

Returns a new color with the alpha value set to `alpha` (between 0 and 1)

#### to-hsv() -> { hue: float, saturation: float, value: float, alpha: float }

Converts this color to the HSV color space and returns a struct with the `hue`, `saturation`, `value`, and `alpha` fields. `hue` is between 0 and 360 while `saturation`, `value`, and `alpha` are between 0 and 1.

#### to-oklch() -> { lightness: float, chroma: float, hue: float, alpha: float }

Converts this color to the [Oklch color space↗](https://en.wikipedia.org/wiki/Oklab_color_space) and returns a struct with the `lightness`, `chroma`, `hue`, and `alpha` fields. `lightness` is between 0 and 1, `chroma` is typically between 0 and ~0.4, `hue` is between 0 and 360, and `alpha` is between 0 and 1.

### Linear Gradients

Linear gradients describe smooth, colorful surfaces. They’re specified using an angle and a series of color stops. The colors will be linearly interpolated between the stops, aligned to an imaginary line that is rotated by the specified angle. This is called a linear gradient and is specified using the `@linear-gradient` macro with the following signature:

#### @linear-gradient(angle, color percentage, color percentage, …)

The first parameter to the macro is an angle (see [Types](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html)). The gradient line’s starting point will be rotated by the specified value.

Following the initial angle is one or multiple color stops, describe as a space separated pair of a `color` value and a `percentage`. The color specifies which value the linear color interpolation should reach at the specified percentage along the axis of the gradient.

The following example shows a rectangle that’s filled with a linear gradient that starts with a light blue color, interpolates to a very light shade in the center and finishes with an orange tone:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;

    Rectangle {
        background: @linear-gradient(90deg, #3f87a6 0%, #ebf8e1 50%, #f69d3c 100%);
    }
}
```

![Conic Gradient Example](https://docs.slint.dev/latest/docs/slint/_astro/gradients-linear.BI1UFnhL_ZiO7qj.webp)

### Radial Gradients

Radial gradients are like linear gradients but the colors are interpolated circularly instead of along a line. To describe a radial gradient, use the `@radial-gradient` macro with the following signature:

#### @radial-gradient(circle, color percentage, color percentage, …)

The first parameter to the macro is always `circle` because only circular gradients are supported. The syntax is otherwise based on the CSS `radial-gradient` function.

Example:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;
    Rectangle {
        background: @radial-gradient(circle, #f00 0%, #0f0 50%, #00f 100%);
    }
}
```

![Conic Gradient Example](https://docs.slint.dev/latest/docs/slint/_astro/gradients-radial.BglY0Fdu_ZAtVH3.webp)

### Conic Gradients

Conic gradients are gradients where the color transitions rotate around a center point (like the angle on a color wheel). To describe a conic gradient, use the `@conic-gradient` macro with the following signature:

#### @conic-gradient([from angle,] color angle, color angle, …)

The conic gradient is described by a series of color stops, each consisting of a color and an angle. The angle specifies where the color is placed along the circular sweep (0deg to 360deg). Colors are interpolated between the stops along the circular path.

The optional `from` parameter specifies the starting angle of the gradient rotation. If omitted, the gradient starts at 0deg (pointing upward). For example, `from 90deg` rotates the entire gradient 90 degrees clockwise.

Example:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;
    Rectangle {
        background: @conic-gradient(#f00 0deg, #0f0 120deg, #00f 240deg, #f00 360deg);
    }
}
```

![Conic Gradient Example](https://docs.slint.dev/latest/docs/slint/_astro/gradients-conic.CH_U8oen_Z4lmD1.webp)

This creates a color wheel effect with red at the top (0deg/360deg), green at 120 degrees, and blue at 240 degrees.

You can also rotate the gradient using the `from` parameter:

```slint
export component Example inherits Window {
    preferred-width: 100px;
    preferred-height: 100px;
    Rectangle {
        background: @conic-gradient(from 90deg, #f00 0deg, #0f0 120deg, #00f 240deg, #f00 360deg);
    }
}
```

![Rotated Conic Gradient Example](https://docs.slint.dev/latest/docs/slint/_astro/gradients-conic-rotated.P5EHMAMq_1MrMJb.webp)

This rotates the same color wheel 90 degrees clockwise, so red starts at the right (90deg) instead of the top.

> **Known Limitation**
> Negative angles cannot be used directly in conic gradients (e.g., `#ff0000 -90deg`). Instead, use one of these workarounds:
>
> - Convert to positive angles: `-90deg` → `270deg`
> - Use variables: `property <angle> start: -90deg;` then use `start` in the gradient
> - Use explicit subtraction: `#ff0000 0deg - 90deg`

## Common Properties & Callbacks

Source: `reference/common/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/common/

The Slint elements have many common properties, callbacks and behavior. This page describes these properties and their usage.

### Common Visual Properties

These properties are valid on all visual items. For example `Rectangle`, `Text`, and `layouts`. Non visual items such as `Timer` don’t have these properties.

#### x, y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The position of the element relative to its parent.

#### z

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

Allows to specify a different order to stack the items with its siblings. The value must be a compile time constant.

> **Note**
> Currently the `z` value is a compile time constant and cannot be changed at runtime.

#### width, height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The width and height of the element. When set, this overrides the default size.

#### Transforms

Transforms allow for rotating and scaling items around a specified origin point. The default origin point is the center of the element.

Transforms are not available for the software renderer.

##### transform-rotation

[angle](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#angle) default: `0deg`

##### transform-origin

[struct Point](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#point) default: `{x: self.width / 2, y: self.height / 2}`

The origin to rotate and scale around.

Default to the center of the element.

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

##### transform-scale

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `100%`

The scale factor to apply to the element and all its children.

This doesn’t affect the geometry (width, height) of the element, but affects the rendering. The scale is done around the `transform-origin` point.

It is also possible to use the `transform-scale-x` and `transform-scale-y` properties to specify the scale factors for the x and y axis.

##### transform-scale-x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `self.scale`

##### transform-scale-y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `self.scale`

#### opacity

```slint
component ImageInfo inherits Rectangle {
    in property <float> img-opacity: 1.0;
    background: transparent;
    VerticalLayout {
        spacing: 5px;
        Image {
            source: @image-url("elements/slint-logo.png");
            opacity: img-opacity;
        }
        Text {
            text: "opacity: " + img-opacity;
            color: white;
            horizontal-alignment: center;
        }
    }
}
export component Example inherits Window {
    width: 100px;
    height: 310px;
    background: transparent;
    Rectangle {
        background: #141414df;
        border-radius: 10px;
    }
    VerticalLayout {
        spacing: 15px;
        padding-top: 10px;
        padding-bottom: 10px;
        ImageInfo {
            img-opacity: 1.0;
        }
        ImageInfo {
            img-opacity: 0.6;
        }
        ImageInfo {
            img-opacity: 0.3;
        }
    }
}
```

![rectangle opacity](https://docs.slint.dev/latest/docs/slint/_astro/rectangle-opacity.BndiRjFt_Z1UIegT.webp)

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `1`

A value between 0 and 1 (or a percentage) that is used to draw the element and its children with transparency. 0 is fully transparent (invisible), and 1 is fully opaque. The opacity is applied to the tree of child elements as if they were first drawn into an intermediate layer, and then the whole layer is rendered with this opacity.

The following example demonstrates the opacity property with children. Note the software renderer does not support layer opacity and this will result in a different end result as shown.

```slint
 Rectangle {
        opacity: 50%;
        Rectangle {
            x: 0;
            y: 0;
            width: 100px;
            height: 100px;
            background: blue;
        }

        Rectangle {
            x: 50px;
            y: 50px;
            width: 100px;
            height: 100px;
            background: green;
        }
    }
```

![layer opacity](https://docs.slint.dev/latest/docs/slint/_astro/layer-opacity.l61Bt-JO_1ya74M.webp)

#### visible

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When set to `false`, the element and all his children won’t be drawn and not react to mouse input. The element will still take up layout space within any layout container.

#### absolute-position

[struct Point](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#point) `(out)` default: `a struct with all default values`

A common issue is that in a UI with many nested components it’s useful to know their (x,y)position relative to the main window or screen. This convenience property gives easy read only access to that value.

It represents a point specifying the absolute position within the enclosing [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html) or [PopupWindow](https://docs.slint.dev/latest/docs/slint/reference/window/popupwindow/index.html). It defines coordinates (x,y) relative to the enclosing Window or PopupWindow, but the reference frame is unspecified (could be screen, window, or popup coordinates).

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

### Miscellaneous

#### cache-rendering-hint

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

When set to `true`, this provides a hint to the renderer to cache the contents of the element and all the children into an intermediate cached layer. For complex sub-trees that rarely change this may speed up the rendering, at the expense of increased memory consumption. Not all rendering backends support this, so this is merely a hint.

#### dialog-button-role

[enum DialogButtonRole](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#dialogbuttonrole) default: `none`

Specify that this is a button in a `Dialog`.

`DialogButtonRole`

This enum represents the value of the `dialog-button-role` property which can be added to any element within a `Dialog` to put that item in the button row, and its exact position depends on the role and the platform.

- **`none`** : This isn’t a button meant to go into the bottom row
- **`accept`** : This is the role of the main button to click to accept the dialog. e.g. “Ok” or “Yes”
- **`reject`** : This is the role of the main button to click to reject the dialog. e.g. “Cancel” or “No”
- **`apply`** : This is the role of the “Apply” button
- **`reset`** : This is the role of the “Reset” button
- **`help`** : This is the role of the “Help” button
- **`action`** : This is the role of any other button that performs another action.

### Common Callbacks

#### init()

Every element implicitly declares an `init` callback. You can assign a code block to it that will be invoked when the element is instantiated and after all properties are initialized with the value of their final binding. The order of invocation is from inside to outside. The following example will print “first”, then “second”, and then “third”:

```slint
component MyButton inherits Rectangle {
    in-out property <string> text: "Initial";
    init => {
        // If `text` is queried here, it will have the value "Hello".
        debug("first");
    }
}

component MyCheckBox inherits Rectangle {
    init => { debug("second"); }
}

export component MyWindow inherits Window {
    MyButton {
        text: "Hello";
        init => { debug("third"); }
    }
    MyCheckBox {
    }
}
```

Don’t use this callback to initialize properties, because this violates the declarative principle.

Even though the `init` callback exists on all components, it cannot be set from application code, i.e. an `on_init` function does not exist in the generated code. This is because the callback is invoked during the creation of the component, before you could call `on_init` to actually set it.

While the `init` callback can invoke other callbacks, e.g. one defined in a `global` section, and you *can* bind these in the backend, this doesn’t work for statically-created components, including the window itself, because you need an instance to set the globals binding. But it is possible to use this for dynamically created components (for example ones behind an `if`):

```slint
export global SystemService  {
    // This callback can be implemented in native code using the Slint API
    callback ensure_service_running();
}

component MySystemButton inherits Rectangle {
    init => {
        SystemService.ensure_service_running();
    }
    // ...
}

export component AppWindow inherits Window {
    in property <bool> show-button: false;

    // MySystemButton isn't initialized at first, only when show-button is set to true.
    // At that point, its init callback will call ensure_service_running()
    if show-button : MySystemButton {}
}
```

### Accessibility Properties

Use the following `accessible-` properties to make your items interact well with software like screen readers, braille terminals and other software to make your application accessible. `accessible-role` must be set in order to be able to set any other accessible property or callback.

#### accessible-role

[enum AccessibleRole](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#accessiblerole) default: `the first enum value`

The role of the element. This property is mandatory to be able to use any other accessible properties. It should be set to a constant value. (default value: `none` for most elements, but `text` for the Text element)

`AccessibleRole`

This enum represents the different values for the `accessible-role` property, used to describe the role of an element in the context of assistive technology such as screen readers.

- **`none`** : The element isn’t accessible.
- **`button`** : The element is a `Button` or behaves like one.
- **`checkbox`** : The element is a `CheckBox` or behaves like one.
- **`combobox`** : The element is a `ComboBox` or behaves like one.
- **`groupbox`** : The element is a `GroupBox` or behaves like one.
- **`image`** : The element is an `Image` or behaves like one. This is automatically applied to `Image` elements.
- **`list`** : The element is a `ListView` or behaves like one.
- **`slider`** : The element is a `Slider` or behaves like one.
- **`spinbox`** : The element is a `SpinBox` or behaves like one.
- **`tab`** : The element is a `Tab` or behaves like one.
- **`tab-list`** : The element is similar to the tab bar in a `TabWidget` .
- **`tab-panel`** : The element is a container for tab content.
- **`text`** : The role for a `Text` element. This is automatically applied to `Text` elements.
- **`table`** : The role for a `TableView` or behaves like one.
- **`tree`** : The role for a TreeView or behaves like one. (Not provided yet)
- **`progress-indicator`** : The element is a `ProgressIndicator` or behaves like one.
- **`text-input`** : The role for widget with editable text such as a `LineEdit` or a `TextEdit` . This is automatically applied to `TextInput` elements.
- **`switch`** : The element is a `Switch` or behaves like one.
- **`list-item`** : The element is an item in a `ListView` .
- **`radio-button`** : The element is a `RadioButton` or behaves like one.

#### accessible-checkable

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element is can be checked or not.

#### accessible-checked

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element is checked or not. This maps to the “checked” state of checkboxes, radio buttons, and other widgets.

#### accessible-description

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The description for the current element.

#### accessible-enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element is enabled or not. This maps to the “enabled” state of most widgets. (default value: `true`)

#### accessible-expandable

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element can be expanded or not. For example, a `ComboBox` widget should set this to true, as its selection can be changed via an expandable popup.

#### accessible-expanded

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element is expanded or not. Applies to combo boxes, menu items, tree view items and other widgets.

#### accessible-id

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

A unique identifier for the element, used to identify widgets for automation and testing purposes. This property can be set to any string value and is particularly useful when writing automated tests or when using accessibility tools to uniquely identify specific widgets in your application.

If you need to identify repeated elements, make sure to assign different values to each instance as illustrated below.

```slint
for i in 5: Rectangle {
    accessible-role: button;
    accessible-id: "btn-" + i;
    accessible-label: "Button " + i;
}
```

#### accessible-label

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The label for an interactive element. (default value: empty for most elements, or the value of the `text` property for Text elements)

#### accessible-value-maximum

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The maximum value of the item. This is used for example by spin boxes.

#### accessible-value-minimum

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The minimum value of the item.

#### accessible-value-step

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The smallest increment or decrement by which the current value can change. This corresponds to the step by which a handle on a slider can be dragged.

#### accessible-value

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The current value of the item.

#### accessible-placeholder-text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

A placeholder text to use when the item’s value is empty. Applies to text elements.

#### accessible-read-only

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element’s content can be edited. This maps to the “read-only” state of line edit and text edit widgets.

#### accessible-item-selectable

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element can be selected or not.

#### accessible-item-selected

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the element is selected or not. This maps to the “is-selected” state of listview items.

#### accessible-item-index

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The index (starting from 0) of this element in a group of similar elements. Applies to list items, radio buttons and other elements.

#### accessible-item-count

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The total number of elements in a group. Applies to the parent container of a group of element such as list views, radio button groups or other grouping elements.

### Accessibility Callbacks

You can also use the following callbacks that are going to be called by the accessibility framework:

#### accessible-action-default()

Invoked when the default action for this widget is requested (eg: pressed for a button).

#### accessible-action-set-value(string)

Invoked when the user wants to change the accessible value.

#### accessible-action-increment()

Invoked when the user requests to increment the value.

#### accessible-action-decrement()

Invoked when the user requests to decrement the value.

#### accessible-action-expand()

Invoked when the user requests to expand the widget (eg: disclose the list of available choices for a combo box).

## Image

Source: `reference/elements/image/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/elements/image/

![image example](https://docs.slint.dev/latest/docs/slint/_astro/image-example.CkIssxCb_ZujAr2.webp)

Use the `Image` element to display an [image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image).

### Properties

#### colorize

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `a transparent brush`

When set, the image is used as an alpha mask and is drawn in the given color (or with the gradient).

```slint
Image {
    source: @image-url("slint-logo-simple-dark.png");
    colorize: darkorange;
}
```

![image example](https://docs.slint.dev/latest/docs/slint/_astro/image-colorize.DRuvxZTB_1RdRaY.webp)

#### horizontal-alignment

[enum ImageHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imagehorizontalalignment) default: `center`

The horizontal alignment of the image within the element.

`ImageHorizontalAlignment`

This enum specifies the horizontal alignment of the source image.

- **`center`** : Aligns the source image at the center of the `Image` element.
- **`left`** : Aligns the source image at the left of the `Image` element.
- **`right`** : Aligns the source image at the right of the `Image` element.

#### vertical-alignment

[enum ImageVerticalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imageverticalalignment) default: `center`

The vertical alignment of the image within the element.

`ImageVerticalAlignment`

This enum specifies the vertical alignment of the source image.

- **`center`** : Aligns the source image at the center of the `Image` element.
- **`top`** : Aligns the source image at the top of the `Image` element.
- **`bottom`** : Aligns the source image at the bottom of the `Image` element.

### Image Tiling

#### horizontal-tiling

[enum ImageTiling](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imagetiling) default: `none`

How the image is tiled horizontally.

`ImageTiling`

This enum specifies how the source image will be tiled.

- **`none`** : The source image will not be tiled.
- **`repeat`** : The source image will be repeated to fill the `Image` element.
- **`round`** : The source image will be repeated and scaled to fill the `Image` element, ensuring an integer number of repetitions.

#### vertical-tiling

[enum ImageTiling](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imagetiling) default: `none`

```slint
Image {
    width: 400px;
    height: 400px;
    source: @image-url("slint-logo.png");
    horizontal-tiling: repeat;
}
```

![image horizontal tiling repeat example](https://docs.slint.dev/latest/docs/slint/_astro/image-horizontal-repeat.2TgM0yIg_q2wgV.webp)

```slint
Image {
    width: 400px;
    height: 400px;
    source: @image-url("slint-logo.png");
    horizontal-tiling: round;
}
```

![image horizontal tiling round example](https://docs.slint.dev/latest/docs/slint/_astro/image-horizontal-round.DR2oGVDV_vOOmi.webp)

```slint
Image {
    width: 400px;
    height: 400px;
    source: @image-url("slint-logo.png");
    vertical-tiling: repeat;
}
```

![image vertical tiling repeat example](https://docs.slint.dev/latest/docs/slint/_astro/image-vertical-repeat.DDIkUi76_2jT89U.webp)

```slint
Image {
    width: 400px;
    height: 400px;
    source: @image-url("slint-logo.png");
    vertical-tiling: round;
}
```

![image vertical tiling round example](https://docs.slint.dev/latest/docs/slint/_astro/image-vertical-round.BnvzSQAg_Z3fG3c.webp)

```slint
Image {
    width: 400px;
    height: 400px;
    source: @image-url("slint-logo.png");
    vertical-tiling: round;
    horizontal-tiling: round;
}
```

![image vertical and horizontal tiling round example](https://docs.slint.dev/latest/docs/slint/_astro/image-vertical-horizontal-round.DIlH9q45_28rdlX.webp)

`ImageTiling`

This enum specifies how the source image will be tiled.

- **`none`** : The source image will not be tiled.
- **`repeat`** : The source image will be repeated to fill the `Image` element.
- **`round`** : The source image will be repeated and scaled to fill the `Image` element, ensuring an integer number of repetitions.

#### image-fit

[enum ImageFit](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imagefit) default: ```contain` when the `Image` element is part of a layout, `fill` otherwise``

```slint
Image {
    width: 200px; height: 50px;
    source: @image-url("mini-banner.png");
    image-fit: fill;
}
```

![image fill example](https://docs.slint.dev/latest/docs/slint/_astro/image-fill.CRdeEROy_khoSV.webp)

```slint
Image {
    width: 250px; height: 40px;
    source: @image-url("mini-banner.png");
    image-fit: contain;
}
```

![image contain example](https://docs.slint.dev/latest/docs/slint/_astro/image-contain.t4ji_Vq1_J237H.webp)

```slint
Image {
    width: 250px; height: 250px;
    source: @image-url("mini-banner.png");
    image-fit: cover;
}
```

![image cover example](https://docs.slint.dev/latest/docs/slint/_astro/image-cover.Cg7tK-FN_5TnLY.webp)

```slint
Image {
    width: 400px; height: 400px;
    source: @image-url("mini-banner.png");
    image-fit: preserve;
}
```

![image preserve example](https://docs.slint.dev/latest/docs/slint/_astro/image-preserve.C6DZGvPt_ZIRax2.webp)

`ImageFit`

This enum defines how the source image or path shall fit into an `Image` or `Path` element.

- **`fill`** : Scales and stretches the source to fit the width and height of the element.
- **`contain`** : The source is scaled to fit into the element’s dimensions while preserving the aspect ratio.
- **`cover`** : The source is scaled to cover the element’s dimensions while preserving the aspect ratio. If the aspect ratios don’t match, the source will be clipped to fit.
- **`preserve`** : Preserves the size of the source in logical pixels. The source will still be scaled by the scale factor that applies to all elements in the window. Any extra space will be left blank.

#### image-rendering

[enum ImageRendering](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imagerendering) default: `smooth`

```slint
Image {
    width: 800px;
    source: @image-url("mini-banner.png");
    image-rendering: smooth;
}
```

![image smooth example](https://docs.slint.dev/latest/docs/slint/_astro/image-smooth.f7_-wIWq_t6PWV.webp)

```slint
Image {
    width: 800px;
    source: @image-url("mini-banner.png");
    image-rendering: pixelated;
}
```

![image pixelated example](https://docs.slint.dev/latest/docs/slint/_astro/image-pixelated.B0PIBm3I_Z1lx6Dw.webp)

`ImageRendering`

This enum specifies how the source image will be scaled.

- **`smooth`** : The image is scaled with a linear interpolation algorithm.
- **`pixelated`** : The image is scaled with the nearest neighbor algorithm.

### Source Properties

#### source

[image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) default: `the empty image`

The `image` type is a reference to an image. It’s defined using the `@image-url("...")` construct. The address within the `@image-url` function must be known at compile time.

Slint looks for images in the following places:

1. The absolute path or the path relative to the current `.slint` file.
2. The include path used by the compiler to look up `.slint` files.

Images can also be loaded from [`data:` URIs↗](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data), with either base64 or URL-encoded content. For example: `@image-url("data:image/png;base64,iVBORw0KGgo...")`.

Access an `image`’s source dimension using its `source.width` and `source.height` properties.

```slint
export component Example inherits Window {
    preferred-width: 150px;
    preferred-height: 50px;

    in property <image> some_image: @image-url("https://slint.dev/logo/slint-logo-full-light.svg");

    Text {
        text: "The image is " + some_image.width + "x" + some_image.height;
    }
}
```

```slint
// nine-slice scaling
export component Example inherits Window {
    width: 100px;
    height: 150px;
    VerticalLayout {
        Image {
            source: @image-url("https://interactive-examples.mdn.mozilla.net/media/examples/border-diamonds.png", nine-slice(30 30 30 30));
        }
    }
}
```

Use the [`@image-url` macro](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) to specify the image’s path.

#### source-clip-x

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

#### source-clip-y

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

#### source-clip-width

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `source.width - source.clip-x`

#### source-clip-height

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `source.height - source.clip-y`

Properties in source image coordinates that define the region of the source image that is rendered. By default the entire source image is visible:

### Accessibility

#### Alternative text

Consider giving an alternative text description of your image by setting the `accessible-label` property:

```slint
Image {
    width: 100px;
    height: 100px;
    source: @image-url("slint-logo.png");
    accessible-label: "Slint logo";
}
```

#### Filtering out images for users of assistive technologies

By default, images have the `accessible-role` property set to `image`. If your image is purely decorative and doesn’t convey any information, consider removing it from the accessibility tree:

```slint
Image {
    source: @image-url("mini-banner.png");
    accessible-role: none;
}
```

## Path

Source: `reference/elements/path/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/elements/path/

The `Path` element allows rendering a generic shape, composed of different geometric commands. A path shape can be filled and outlined.

When not part of a layout, its width or height defaults to 100% of the parent element when not specified.

A path can be defined in two different ways:

- Using SVG path commands as a string
- Using path command elements in `.slint` markup.

The coordinates used in the geometric commands are within the imaginary coordinate system of the path. When rendering on the screen, the shape is drawn relative to the `x` and `y` properties. If the `width` and `height` properties are non-zero, then the entire shape is fit into these bounds - by scaling accordingly.

### Properties

#### fill

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `a transparent brush`

The color for filling the shape of the path.

#### fill-rule

[enum FillRule](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#fillrule) default: `nonzero`

The fill rule to use for the path.

`FillRule`

This enum describes the different ways of deciding what the inside of a shape described by a path shall be.

- **`nonzero`** : The [“nonzero” fill rule as defined in SVG↗](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule#nonzero) .
- **`evenodd`** : The [“evenodd” fill rule as defined in SVG↗](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule#evenodd)

#### stroke

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `a transparent brush`

The color for drawing the outline of the path.

#### stroke-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The width of the outline.

#### stroke-line-cap

[enum LineCap](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#linecap) default: `butt`

The appearance of the ends of the path’s outline.

`LineCap`

This enum describes the appearance of the ends of stroked paths.

- **`butt`** : The stroke ends with a flat edge that is perpendicular to the path.
- **`round`** : The stroke ends with a rounded edge.
- **`square`** : The stroke ends with a square projection beyond the path.

#### stroke-line-join

[enum LineJoin](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#linejoin) default: `miter`

The appearance of the joins between segments of stroked paths.

`LineJoin`

This enum describes the appearance of the joins between segments of stroked paths.

- **`miter`** : The stroke joins with a sharp corner or a clipped corner, depending on the miter limit.
- **`round`** : The stroke joins with a smooth, rounded corner.
- **`bevel`** : The stroke joins with a beveled (flattened) corner.

#### stroke-miter-limit

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `4`

The limit on the ratio of the miter length to the stroke width when `stroke-line-join` is set to `miter`. When the limit is exceeded, the join is rendered as a bevel instead.

#### width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

If non-zero, the path will be scaled to fit into the specified width.

#### height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

If non-zero, the path will be scaled to fit into the specified height.

### Viewbox Properties

#### viewbox-x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

#### viewbox-y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

#### viewbox-width

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

#### viewbox-height

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

These four properties allow defining the position and size of the viewport of the path in path coordinates.

If the `viewbox-width` or `viewbox-height` is less or equal than zero, the viewbox properties are ignored and instead the bounding rectangle of all path elements is used to define the view port.

#### fit

[enum ImageFit](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#imagefit) default: `contain`

Defines how the path’s view box is scaled to fit the element’s width and height. If no view box is defined, the implicit bounding rectangle is used.

`ImageFit`

This enum defines how the source image or path shall fit into an `Image` or `Path` element.

- **`fill`** : Scales and stretches the source to fit the width and height of the element.
- **`contain`** : The source is scaled to fit into the element’s dimensions while preserving the aspect ratio.
- **`cover`** : The source is scaled to cover the element’s dimensions while preserving the aspect ratio. If the aspect ratios don’t match, the source will be clipped to fit.
- **`preserve`** : Preserves the size of the source in logical pixels. The source will still be scaled by the scale factor that applies to all elements in the window. Any extra space will be left blank.

#### clip

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

By default, when a path has a view box defined and the elements render outside of it, they are still rendered. When this property is set to `true`, then rendering will be clipped at the boundaries of the view box.

#### anti-alias

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

By default, the fill and stroke of a path is rendered with anti-aliasing, for best quality. Some GPUs have performance issues when rendering with anti-aliasing and animation. Setting the value to `false` might improve the frame-rate at the expense of a smoother looking path.

### Path Using SVG Commands

SVG is a popular file format for defining scalable graphics, which are often composed of paths. In SVG paths are composed using [commands↗](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#path_commands), which in turn are written in a string. In `.slint` the path commands are provided to the `commands` property. The following example renders a shape consists of an arc and a rectangle, composed of `line-to`, `move-to` and `arc` commands:

The commands are provided in a property:

#### Commands

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

A string providing the commands according to the SVG path specification. This property can only be set in a binding and cannot be accessed in an expression.

### Path Using SVG Path Elements

The shape of the path can also be described using elements that resemble the SVG path commands but use the `.slint` markup syntax. The earlier example using SVG commands can also be written like that:

```slint
export component Example inherits Path {
    width: 100px;
    height: 100px;
    stroke: blue;
    stroke-width: 1px;

    MoveTo {
        x: 0;
        y: 0;
    }
    LineTo {
        x: 0;
        y: 100;
    }
    ArcTo {
        radius-x: 1;
        radius-y: 1;
        x: 100;
        y: 100;
    }
    LineTo {
        x: 100;
        y: 0;
    }
    Close {
    }
}
```

Note how the coordinates of the path elements don’t use units - they operate within the imaginary coordinate system of the scalable path.

### MoveTo Sub-Element for `Path`

The `MoveTo` sub-element closes the current sub-path, if present, and moves the current point to the location specified by the `x` and `y` properties. Subsequent elements such as `LineTo` will use this new position as their starting point, therefore this starts a new sub-path.

#### x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The x position of the new current point.

#### y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The y position of the new current point.

### LineTo Sub-Element for `Path`

The `LineTo` sub-element describes a line from the path’s current position to the location specified by the `x` and `y` properties.

#### x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target x position of the line.

#### y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target y position of the line.

### ArcTo Sub-Element for `Path`

The `ArcTo` sub-element describes the portion of an ellipse. The arc is drawn from the path’s current position to the location specified by the `x` and `y` properties. The remaining properties are modelled after the SVG specification and allow tuning visual features such as the direction or angle.

#### large-arc

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Out of the two arcs of a closed ellipse, this flag selects that the larger arc is to be rendered. If the property is `false`, the shorter arc is rendered instead.

#### radius-x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The x-radius of the ellipse.

#### radius-y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The y-radius of the ellipse.

#### sweep

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

If the property is `true`, the arc will be drawn as a clockwise turning arc; anti-clockwise otherwise.

#### x-rotation

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The x-axis of the ellipse will be rotated by the value of this properties, specified in as angle in degrees from 0 to 360.

#### x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target x position of the line.

#### y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target y position of the line.

### CubicTo Sub-Element for `Path`

The `CubicTo` sub-element describes a smooth Bézier from the path’s current position to the location specified by the `x` and `y` properties, using two control points specified by their respective properties.

#### control-1-x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The x coordinate of the curve’s first control point.

#### control-1-y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The y coordinate of the curve’s first control point.

#### control-2-x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The x coordinate of the curve’s second control point.

#### control-2-y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The y coordinate of the curve’s second control point.

#### x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target x position of the curve.

#### y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target y position of the curve.

### QuadraticTo Sub-Element for `Path`

The QuadraticTo sub-element describes a smooth Bézier from the path’s current position to the location specified by the `x` and `y` properties, using the control points specified by the `control-x` and `control-y` properties.

#### control-x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The x coordinate of the curve’s control point.

#### control-y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The y coordinate of the curve’s control point.

#### x

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target x position of the curve.

#### y

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

The target y position of the curve.

### Close Sub-Element for `Path`

The `Close` element closes the current sub-path and draws a straight line from the current position to the beginning of the path.

## Rectangle

Source: `reference/elements/rectangle/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/elements/rectangle/

By default, a `Rectangle` is just an empty item that shows nothing. By setting a color or configuring a border, it’s then possible to draw a rectangle on the screen.

When not part of a layout, its width and height default to 100% of the parent element.

![rectangle example](https://docs.slint.dev/latest/docs/slint/_astro/rectangle-example.CraeGWfp_ZiGOHO.webp)

### Properties

#### background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `transparent`

The background brush of this `Rectangle`.

```slint
property <brush> rainbow-gradient: @linear-gradient(40deg, rgba(255, 0, 0, 1) 0%, rgba(255, 154, 0, 1) 10%, rgba(208, 222, 33, 1) 20%,rgba(79, 220, 74, 1) 30%, rgba(63, 218, 216, 1) 40%, rgba(47, 201, 226, 1) 50%, rgba(28, 127, 238, 1) 60%, rgba(95, 21, 242, 1) 70%, rgba(186, 12, 248, 1) 80%, rgba(251, 7, 217, 1) 90%, rgba(255, 0, 0, 1) 100%);

Rectangle {
    x: 10px;
    y: 10px;
    width: 180px;
    height: 180px;
    background: #315afd;
}

Rectangle {
    x: 10px;
    y: 210px;
    width: 180px;
    height: 180px;
    background: rainbow-gradient;
}
```

![rectangle background](https://docs.slint.dev/latest/docs/slint/_astro/rectangle-background.BwpXiHkZ_Z1mzJlb.webp)

#### border-color

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `transparent`

```slint
Rectangle {
    width: 200px;
    height: 200px;
    border-width: 10px;
    border-color: lightslategray;
}
```

![rectangle border-color](https://docs.slint.dev/latest/docs/slint/_astro/rectangle-border-color.COqndOoO_1yXybd.webp)

The color of the border.

> **Caution**
> The default `border-width` is `0px`, so the border is invisible. After setting a color also ensure that the `border-width` is set to a non-zero value.

#### border-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0`

```slint
Rectangle {
    width: 200px;
    height: 200px;
    border-width: 30px;
    border-color: lightslategray;
}
```

![rectangle border-width](https://docs.slint.dev/latest/docs/slint/_astro/rectangle-border-width.yX7fq6UX_qsmw9.webp)

The width of the border.

#### clip

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

```slint
// clip: false; the default
Rectangle {
    x: 50px; y: 50px;
    width: 150px;
    height: 150px;
    background: darkslategray;
    Rectangle {
        x: -40px; y: -40px;
        width: 100px;
        height: 100px;
        background: lightslategray;
    }
}

// clip: true; Clips the children of this Rectangle
Rectangle {
    x: 50px; y: 250px;
    width: 150px;
    height: 150px;
    background: darkslategray;
    clip: true;
    Rectangle {
        x: -40px; y: -40px;
        width: 100px;
        height: 100px;
        background: lightslategray;
    }
}
```

![rectangle clip](https://docs.slint.dev/latest/docs/slint/_astro/rectangle-clip.fN6Z1ogM_Z1oSxA6.webp)

By default, when child elements are outside the bounds of a parent, they are still shown. When this property is set to `true`, the children of this `Rectangle` are clipped and only the contents inside the elements bounds are shown.

### Border Radius Properties

#### border-radius

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0`

The size of the radius. This single value is applied to all four corners.

To target specific corners with different values use the following properties:

#### border-top-left-radius

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### border-top-right-radius

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### border-bottom-left-radius

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### border-bottom-right-radius

### Drop Shadows

To achieve the graphical effect of a visually elevated shape that shows a shadow effect underneath the frame of an element, it’s possible to set the following `drop-shadow` properties:

#### drop-shadow-blur

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The radius of the shadow that also describes the level of blur applied to the shadow. Negative values are ignored and zero means no blur.

#### drop-shadow-color

[color](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#color) default: `a transparent color`

The base color of the shadow to use. Typically that color is the starting color of a gradient that fades into transparency.

#### drop-shadow-offset-x

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The horizontal distance of the shadow from the element’s frame.

#### drop-shadow-offset-y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The vertical distance of the shadow from the element’s frame.

## StyledText

Source: `reference/elements/styled-text/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/elements/styled-text/

The `StyledText` element renders text with various styling and interactive properties, such as bolded, underlined and colored sections as well as HTTP links. It is based on a subset of the [commonmark↗](https://commonmark.org/) spec.

![Styled Text Example](https://docs.slint.dev/latest/docs/slint/_astro/styled-text-example.CUxjLXJF_ZGrdAA.webp)

### Features

Styled Text supports the following features:

| Feature | Method |
| --- | --- |
| Italics | Builtin |
| Strikethroughs | Builtin |
| Inline code | Builtin |
| Links | Builtin |
| Ordered and unordered lists | Builtin |
| Underlines | `<u>` HTML tag |
| Text Colors | `<font color="...">` HTML tags |

#### Currently Unsupported

| Feature |
| --- |
| Headings |
| Images |
| Tables |
| Block Quotes |
| Subscripts |
| Superscripts |
| Horizontal Rules |
| Footnotes |
| Math expressions |
| Other HTML tags |

### Properties

#### default-color

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `<depends on theme>`

The default color of the text, used when no color is specified via markup.

#### default-font-family

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The default font family used to render the text, when no font is specified via markup. If left empty, the value falls back to the enclosing `Window`’s `default-font-family`.

#### default-font-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The default font size used to render the text, when no size is specified via markup. If unset (or zero), the value falls back to the enclosing `Window`’s `default-font-size`.

#### horizontal-alignment

[enum TextHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#texthorizontalalignment) default: `the first enum value`

The horizontal alignment of the text.

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

#### link-color

[color](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#color) default: `#00f`

The color used for rendering links in the text.

#### text

[styled-text](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#styled-text) default: `""`

The styled text rendered, using CommonMark markup with additional HTML tags for styling.

#### vertical-alignment

[enum TextVerticalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textverticalalignment) default: `the first enum value`

The vertical alignment of the text.

`TextVerticalAlignment`

This enum describes the different types of alignment of text along the vertical axis of a `Text` or `StyledText` element.

- **`top`** : The text will be aligned to the top of the containing box.
- **`center`** : The text will be vertically centered within the containing box.
- **`bottom`** : The text will be aligned to the bottom of the containing box.

### Callbacks

#### link-clicked(link: string)

A callback that’s invoked when a link in the text is clicked. The parameter contains the clicked link as a string.

## Text

Source: `reference/elements/text/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/elements/text/

A `Text` element for displaying text.

By default, the `min-width`, `min-height`, `preferred-width`, and `preferred-height` of a `Text` element are set to fit the full text as if it were displayed on a single line (unless the text contains explicit line breaks). However, if the `wrap` property is set to `word-wrap`, and/or if the `overflow` property is set to `elide`, the `min-width` is reduced to zero, allowing the text to wrap or be elided, while the `preferred-width` and `preferred-height` remain unchanged.

### Properties

#### color

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `<depends on theme>`

The color of the text.

```slint
Text {
    text: "Hello";
    color: #3586f4;
    font-size: 40pt;
}
```

![text color](https://docs.slint.dev/latest/docs/slint/_astro/text-color.BTiwrd_9_ZfXilU.webp)

#### font-family

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The name of the font family selected for rendering the text.

```slint
Text {
    text: "CoMiC!";
    color: black;
    font-size: 40pt;
    font-family: "Comic Sans MS";
}
```

![text font-family](https://docs.slint.dev/latest/docs/slint/_astro/text-font-family.D-oLSyQj_ZUKqjI.webp)

> **Note**
> Make sure the font is loaded before using it in a `Text` element. See [FontHandling](https://docs.slint.dev/latest/docs/slint/guide/development/fonts/index.html) for more.

#### font-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The font size of the text.

```slint
Text {
    text: "Big";
    color: black;
    font-size: 70pt;
}
```

![text font-size](https://docs.slint.dev/latest/docs/slint/_astro/text-font-size.Cz22HPSB_ATt6L.webp)

#### font-weight

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The weight of the font. The values range from 100 (lightest) to 900 (thickest). 400 is the normal weight. Use the [FontWeight](https://docs.slint.dev/latest/docs/slint/reference/global-namespaces/font-weight/index.html) namespace for predefined constants.

```slint
Text {
    text: "BOLD";
    color: black;
    font-size: 30pt;
    font-weight: FontWeight.extra-bold;
}
```

![text font-weight](https://docs.slint.dev/latest/docs/slint/_astro/text-font-weight.BKAutUs0_Z1IaPCp.webp)

#### font-italic

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether or not the font face should be drawn italicized or not.

```slint
Text {
    text: "Italic";
    color: black;
    font-italic: true;
    font-size: 40pt;
}
```

![text font-family](https://docs.slint.dev/latest/docs/slint/_astro/text-font-italic.DI1qPAxy_Z2b92EB.webp)

#### font-metrics

[struct FontMetrics](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#fontmetrics) `(out)` default: `a struct with all default values`

The design metrics of the font scaled to the font pixel size used by the element.

`FontMetrics`

A structure to hold metrics of a font for a specified pixel size.

- **`ascent`** ( *length* ): The distance between the baseline and the top of the tallest glyph in the font.
- **`descent`** ( *length* ): The distance between the baseline and the bottom of the tallest glyph in the font. This is usually negative.
- **`x_height`** ( *length* ): The distance between the baseline and the horizontal midpoint of the tallest glyph in the font, or zero if not specified by the font.
- **`cap_height`** ( *length* ): The distance between the baseline and the top of a regular upper-case glyph in the font, or zero if not specified by the font.

#### horizontal-alignment

[enum TextHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#texthorizontalalignment) default: `the first enum value`

```slint
Text {
    x: 0;
    text: "Hello";
    color: black;
    font-size: 40pt;
    horizontal-alignment: left;
}
```

![text-horizontal-alignment](https://docs.slint.dev/latest/docs/slint/_astro/text-horizontal-alignment.BsxAbKQo_Z1Mzmpm.webp)

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

#### letter-spacing

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The letter spacing allows changing the spacing between the glyphs. A positive value increases the spacing and a negative value decreases the distance.

```slint
Text {
    text: "Spaced!";
    color: black;
    font-size: 30pt;
    letter-spacing: 4px;
}
```

![text-horizontal-alignment](https://docs.slint.dev/latest/docs/slint/_astro/text-letter-spacing.BUmcYS1C_2vVdQO.webp)

#### overflow

[enum TextOverflow](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textoverflow) default: `the first enum value`

How the text should behave when it exceeds the available space.

`TextOverflow`

This enum describes the how the text appears if it is too wide to fit in the width of a `Text` or `StyledText` element.

- **`clip`** : The text will simply be clipped.
- **`elide`** : The text will be elided with `…` .

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text rendered.

#### vertical-alignment

[enum TextVerticalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textverticalalignment) default: `the first enum value`

The vertical alignment of the text.

`TextVerticalAlignment`

This enum describes the different types of alignment of text along the vertical axis of a `Text` or `StyledText` element.

- **`top`** : The text will be aligned to the top of the containing box.
- **`center`** : The text will be vertically centered within the containing box.
- **`bottom`** : The text will be aligned to the bottom of the containing box.

#### wrap

[enum TextWrap](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textwrap) default: `the first enum value`

```slint
Text {
    text: "This paragraph breaks into multiple lines of text";
    font-size: 20pt;
    wrap: word-wrap;
    width: 180px;
}
```

![wrap](https://docs.slint.dev/latest/docs/slint/_astro/text_wrap.fK2D00hx_z3s9s.webp)

`TextWrap`

This enum describes the how the text wraps if it is too wide to fit in the width of a `Text` or `StyledText` element.

- **`no-wrap`** : The text won’t wrap, but instead will overflow.
- **`word-wrap`** : The text will be wrapped at word boundaries if possible, or at any location for very long words.
- **`char-wrap`** : The text will be wrapped at any character. Currently only supported by the Qt and Software renderers.

#### stroke

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `a transparent brush`

The brush used for the text outline.

```slint
Text {
    text: "Stroke";
    stroke-width: 2px;
    stroke: darkblue;
    stroke-style: center;
    font-size: 80px;
    color: lightblue;
}
```

![text stroke](https://docs.slint.dev/latest/docs/slint/_astro/text-stroke.Dj6HzCFb_Z2wlVg5.webp)

#### stroke-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The width of the text outline. If the width is zero, then a hairline stroke (1 physical pixel) will be rendered.

#### stroke-style

[enum TextStrokeStyle](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textstrokestyle) default: `the first enum value`

```slint
Text {
    text: "Style";
    stroke-width: 2px;
    stroke: #3586f4;
    stroke-style: center;
    font-size: 60px;
    color: white;
}
```

![stroke-style](https://docs.slint.dev/latest/docs/slint/_astro/text-stroke-style.DuBYCNM6_Z2wjRd7.webp)

`TextStrokeStyle`

This enum describes the positioning of a text stroke relative to the border of the glyphs in a `Text` or `StyledText` element.

- **`outside`** : The inside edge of the stroke is at the outer edge of the text.
- **`center`** : The center line of the stroke is at the outer edge of the text, like in Adobe Illustrator.

### Accessibility

By default, `Text` elements have the following accessibility properties set:

- `accessible-role: text;`
- `accessible-label: text;`

## Flickable

Source: `reference/gestures/flickable/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/gestures/flickable/

The `Flickable` is a low-level element that is the base for scrollable widgets, such as the [ScrollView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/scrollview/index.html). When the `viewport-width` or the `viewport-height` is greater than the parent’s `width` or `height` respectively, the element becomes scrollable. Note that the `Flickable` doesn’t create a scrollbar. When unset, the `viewport-width` and `viewport-height` are calculated automatically based on the `Flickable`’s children. This isn’t the case when using a `for` loop to populate the elements. This is a bug tracked in issue [#407↗](https://github.com/slint-ui/slint/issues/407). The maximum and preferred size of the `Flickable` are based on the viewport.

When not part of a layout, its width or height defaults to 100% of the parent element when not specified.

### Pointer Event Interaction

If the `Flickable`’s area contains elements that use `TouchArea` to act on clicking, such as `Button` widgets, then the following algorithm is used to distinguish between the user’s intent of scrolling or interacting with `TouchArea` elements:

1. If the `Flickable` ’s `interactive` property is `false` , all events are forwarded to elements underneath.
2. If a press event is received where the event’s coordinates interact with a `TouchArea` , the event is stored and any subsequent move and release events are handled as follows:
  1. If 100ms elapse without any events, the stored press event is delivered to the `TouchArea` .
  2. If a release event is received before 100ms have elapsed, the stored press event as well as the release event are immediately delivered to the `TouchArea` and the algorithm resets.
  3. Any move events received will start a flicking operation on the `Flickable` if all of the following conditions are met:
    1. The event is received before 500ms have elapsed since receiving the press event.
    2. The distance to the press event exceeds 8 logical pixels in an orientation in which we are allowed to move. If `Flickable` decides to flick, any press event sent previously to a `TouchArea` , is followed up by an exit event. During the phase of receiving move events, the flickable follows the coordinates.
3. If the interaction of press, move, and release events begins at coordinates that do not intersect with a `TouchArea` , then `Flickable` will flick immediately on pointer move events when the euclidean distance to the coordinates of the press event exceeds 8 logical pixels.

All pointer and mouse events that occur within the bounds of a `Flickable` are intercepted by the `Flickable` itself and are not propagated to any elements underneath it.

### Wheel/Scroll Event Interaction

The `Flickable` also supports scrolling with the mouse wheel and touchpad scroll gestures. It will scroll regardless of the `interactive` property. If the `Flickable` can scroll in the event’s direction, the event will be intercepted. If the Flickable can’t scroll in the direction of the event, the event will be forwarded to the parent.

### Properties

#### interactive

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

```slint
Flickable {
    interactive: false;
}
```

![flickable interactive](https://docs.slint.dev/latest/docs/slint/_astro/flickable-interactive.CLcwxJ07_2vXt7K.webp)

When true, the viewport can be scrolled by clicking on it and dragging it with the cursor.

#### viewport-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The total width of the scrollable element.

#### viewport-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The total height of the scrollable element.

#### viewport-x

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The position of the scrollable element relative to the `Flickable`. This is usually a negative value.

#### viewport-y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The position of the scrollable element relative to the `Flickable`. This is usually a negative value.

### Callbacks

#### flicked()

Invoked when `viewport-x` or `viewport-y` is changed by a user action (dragging, scrolling).

## ScaleRotateGestureHandler

Source: `reference/gestures/scalerotategesturehandler/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/gestures/scalerotategesturehandler/

Use the `ScaleRotateGestureHandler` to handle pinch and rotation gestures. Recognition is limited to the element’s geometry.

The `ScaleRoteGestureHandler` supports touchscreens on all platforms, and additionally supports trackpad gestures on macOS and iOS.

The `scale` property provides a cumulative scale factor relative to the start of the gesture (starting at `1.0`). The `rotation` property provides a cumulative rotation angle (starting at `0deg`). Use the `started` callback to capture your initial state, then multiply by `scale` and add `rotation` in the `updated` callback to apply the gesture.

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When disabled, the `ScaleRotateGestureHandler` doesn’t recognize any gestures and any on-going gesture is cancelled.

#### active

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

`true` while a gesture is being recognized, `false` otherwise.

#### scale

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) `(out)` default: `1.0`

The cumulative scale factor of the gesture. Always starts at `1.0` when the gesture begins. A value greater than `1.0` means zooming in, less than `1.0` means zooming out. When the gesture is not active, the value is `1.0`.

#### rotation

[angle](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#angle) `(out)` default: `0deg`

The cumulative rotation angle of the gesture. Always starts at `0deg` when the gesture begins. Positive values indicate clockwise rotation, negative values indicate counter-clockwise rotation. When the gesture is not active, the value is `0deg`.

#### center

[struct Point](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#point) `(out)` default: `a struct with all default values`

The center point of the gesture, in the coordinate system of the `ScaleRotateGestureHandler`. For two-finger touch input, this is the midpoint between the two fingers. For trackpad gestures, this is the mouse cursor position.

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

### Callbacks

#### started()

Invoked when a gesture begins. Use this to capture the initial state you want to transform.

#### updated()

Invoked whenever the `scale`, `rotation`, or `center` changes during the gesture.

#### ended()

Invoked when the gesture completes normally (fingers lifted).

#### cancelled()

Invoked when the gesture is cancelled, for example when the handler is disabled during an active gesture or the window loses focus.

## SwipeGestureHandler

Source: `reference/gestures/swipegesturehandler/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/gestures/swipegesturehandler/

Use the `SwipeGestureHandler` to handle swipe gesture in some particular direction. Recognition is limited to the element’s geometry.

The `SwipeGestureHandler` recognizes touchscreen swipes and mouse drags.

Specify the different swipe directions you’d like to handle by setting the `handle-swipe-left/right/up/down` properties and react to the gesture in the `swiped` callback.

Pointer press events on the recognizer’s area are forwarded to the children with a small delay. If the pointer moves by more than 8 logical pixels in one of the enabled swipe directions, the gesture is recognized, and events are no longer forwarded to the children.

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When disabled, the `SwipeGestureHandler` doesn’t recognize any gestures.

#### pressed-position

[struct Point](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#point) `(out)` default: `a struct with all default values`

The position of the pointer when the swipe started.

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

#### current-position

[struct Point](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#point) `(out)` default: `a struct with all default values`

The current pointer position.

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

#### swiping

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

`true` while the gesture is recognized, false otherwise.

#### Handle swipe directions properties

#### handle-swipe-left

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

#### handle-swipe-right

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

#### handle-swipe-up

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

#### handle-swipe-down

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

### Callbacks

#### moved()

Invoked when the pointer is moved.

#### swiped()

Invoked after the swipe gesture was recognized and the pointer was released.

#### cancelled()

Invoked when the swipe is cancelled programmatically or if the window loses focus.

### Functions

#### cancel()

Cancel any on-going swipe gesture recognition.

## TouchArea

Source: `reference/gestures/toucharea/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/gestures/toucharea/

Use `TouchArea` to control what happens when the region it covers is touched or interacted with using the mouse.

When not part of a layout, its width or height default to 100% of the parent element.

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When disabled, the `TouchArea` doesn’t recognize any touch or mouse events and they are passed through to elements underneath.

```slint
import { Button, CheckBox } from "std-widgets.slint";

export component Example inherits Window {
    width: 200px; height: 100px;

    VerticalLayout {
        Rectangle {
            Button {
                text: "Try to press me";
            }
            TouchArea {
                enabled: event-blocker.checked;
            }
        }
        event-blocker := CheckBox {
            text: "Block Access";
        }
    }
}
```

![Basic syntax](https://docs.slint.dev/latest/docs/slint/_astro/basic-syntax.BAuYrWhi_4zyMM.webp)

> **Note**
> When `enabled` is set to false while the `TouchArea` is pressed, `pointer-event` will be invoked with `PointerEventKind.Cancel`, and the `pressed` and `has-hover` properties will be reset to `false`.

#### has-hover

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the mouse is over the `TouchArea` area.

#### mouse-cursor

[enum MouseCursor](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#mousecursor) default: `the first enum value`

The mouse cursor type when the mouse is hovering the `TouchArea`.

`MouseCursor`

This enum represents different types of mouse cursors. It’s a subset of the mouse cursors available in CSS. For details and pictograms see the [MDN Documentation for cursor↗](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#values). Depending on the backend and used OS unidirectional resize cursors may be replaced with bidirectional ones.

- **`default`** : The systems default cursor.
- **`none`** : No cursor is displayed.
- **`help`** : A cursor indicating help information.
- **`pointer`** : A pointing hand indicating a link.
- **`progress`** : The program is busy but can still be interacted with.
- **`wait`** : The program is busy.
- **`crosshair`** : A crosshair.
- **`text`** : A cursor indicating selectable text.
- **`alias`** : An alias or shortcut is being created.
- **`copy`** : A copy is being created.
- **`move`** : Something is to be moved.
- **`no-drop`** : Something can’t be dropped here.
- **`not-allowed`** : An action isn’t allowed
- **`grab`** : Something is grabbable.
- **`grabbing`** : Something is being grabbed.
- **`col-resize`** : Indicating that a column is resizable horizontally.
- **`row-resize`** : Indicating that a row is resizable vertically.
- **`n-resize`** : Unidirectional resize north.
- **`e-resize`** : Unidirectional resize east.
- **`s-resize`** : Unidirectional resize south.
- **`w-resize`** : Unidirectional resize west.
- **`ne-resize`** : Unidirectional resize north-east.
- **`nw-resize`** : Unidirectional resize north-west.
- **`se-resize`** : Unidirectional resize south-east.
- **`sw-resize`** : Unidirectional resize south-west.
- **`ew-resize`** : Bidirectional resize east-west.
- **`ns-resize`** : Bidirectional resize north-south.
- **`nesw-resize`** : Bidirectional resize north-east-south-west.
- **`nwse-resize`** : Bidirectional resize north-west-south-east.

#### mouse-x

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

Set by the `TouchArea` to the position of the mouse within it.

#### mouse-y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

Set by the `TouchArea` to the position of the mouse within it.

#### pressed-x

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

Set by the `TouchArea` to the position of the mouse at the moment it was last pressed.

#### pressed-y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

Set by the `TouchArea` to the position of the mouse at the moment it was last pressed.

#### pressed

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to `true` by the `TouchArea` when the mouse is pressed over it.

### Callbacks

#### clicked()

Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.

#### double-clicked()

Invoked when double-clicked. The left mouse button is pressed and released twice on this element in a short period of time, or the same is done with a finger. The `clicked()` callbacks will be triggered before the `double-clicked()` callback is triggered.

#### moved()

The mouse or finger has been moved. This will only be called if the mouse is also pressed or the finger continues to touch the display. See also **pointer-event(PointerEvent)**.

#### pointer-event(event: PointerEvent)

`PointerEvent`

Represents a Pointer event sent by the windowing system. This structure is passed to the `pointer-event` callback of the `TouchArea` element.

- **`button`** ( *PointerEventButton* ): The button that was pressed or released
- **`kind`** ( *PointerEventKind* ): The kind of the event
- **`modifiers`** ( *KeyboardModifiers* ): The keyboard modifiers pressed during the event

#### scroll-event(event: PointerScrollEvent) -> EventResult

Invoked when the mouse wheel was rotated or another scroll gesture was made. The `PointerScrollEvent` argument contains information about how much to scroll in what direction.

`PointerScrollEvent`

Represents a Pointer scroll (or wheel) event sent by the windowing system. This structure is passed to the `scroll-event` callback of the `TouchArea` element.

- **`delta_x`** ( *length* ): The amount of pixel in the horizontal direction
- **`delta_y`** ( *length* ): The amount of pixel in the vertical direction
- **`modifiers`** ( *KeyboardModifiers* ): The keyboard modifiers pressed during the event

The returned `EventResult`indicates whether to accept or ignore the event. Ignored events are forwarded to the parent element.

`EventResult`

This enum describes whether an event was rejected or accepted by an event handler.

- **`reject`** : The event is rejected by this event handler and may then be handled by the parent item
- **`accept`** : The event is accepted and won’t be processed further

## Builtin Functions

Source: `reference/global-functions/builtinfunctions/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/global-functions/builtinfunctions/

### animation-tick() -> duration

This function returns a monotonically increasing time, which can be used for animations. Calling this function from a binding will constantly re-evaluate the binding. It can be used like so: `x: 1000px + sin(animation-tick() / 1s * 360deg) * 100px;` or `y: 20px * mod(animation-tick(), 2s) / 2s`

### debug(…)

The debug function can take one or multiple values as arguments, prints them, and returns nothing.

## Math

Source: `reference/global-functions/math/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/global-functions/math/

The **Math** namespace contains functions that are available both in the global scope and in the `Math` namespace.

```slint
// Using the functions via global scope. No need for 'Math' prefix.
x: abs(-10); // sets x to 10
```

Some of the math functions can be used in a postfix style which can make the code more readable.

```slint
// Using the postfix style.
x: (-10).abs(); // sets x to 10
```

**T type** Many of the math functions can be used with any [numeric type](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#numeric-types) such as `angle`, `duration`, `float`, `int`, `length`, and `percent`. These are represented on this page as `T`.

### General Math Functions

#### abs(T) -> T

Return the absolute value, where T is a numeric type.

```slint
Math.abs(-10); // returns 10
abs(-10px); // returns 10px
(-10).abs(); // returns 10
```

#### ceil(float) -> int

Returns the value rounded up to the nearest integer.

```slint
Math.ceil(4); // returns 4
Math.ceil(2.3); // returns 3
Math.ceil(-1.5); // returns -1
```

#### floor(float) -> int

Returns the value rounded down to the nearest integer.

```slint
Math.floor(4); // returns 4
Math.floor(2.3); // returns 2
Math.floor(-1.5); // returns -2
```

#### round(float) -> int

Return the value rounded to the nearest integer

```slint
Math.round(4.5); // returns 5
Math.round(4.4); // returns 4
Math.round(-1.2); // returns -1
```

#### sign(float) -> float

Returns 1 or -1, indicating the sign of the number passed as argument. Returns 1 if the input is 0 or -0.

```slint
Math.sign(10); // returns 1
Math.sign(-30); // returns -1
Math.sign(0); // returns 1
```

#### clamp(T, T, T) -> T

Takes a `value`, `minimum` and `maximum` and returns `maximum` if `value > maximum`, `minimum` if `value < minimum`, or `value` in all other cases.

#### log(float, float) -> float

Return the log of the first value with a base of the second value

#### ln(float) -> float

Return the natural log of the value. Same as log(e, x)

#### min(T, T) -> T

#### max(T, T) -> T

Return the arguments with the minimum (or maximum) value. All arguments must be of the same numeric type.

```slint
Math.min(1, 2); // returns 1
Math.min(2, 1); // returns 1
Math.max(1, 2); // returns 2
Math.max(2, 1); // returns 2
```

#### mod(T, T) -> T

Perform a modulo operation, where T is some numeric type. Returns the remainder of the euclidean division of the arguments. This always returns a positive number between 0 and the absolute value of the second value.

#### sqrt(float) -> float

Square root

#### pow(float, float) -> float

Return the value of the first value raised to the second

#### exp(float, float) -> float

Return the value of the e raised to the x

### Trigonometric Functions

#### acos(float) -> angle

Returns the arccosine, or inverse cosine, of a number. The arccosine is the angle whose cosine is number.

#### asin(float) -> angle

Returns the arcsine, or inverse sine, of a number. The arcsine is the angle whose sine is number.

#### atan(float) -> angle

Returns the arctangent, or inverse tangent, of a number.

#### atan2(float, float) -> angle

#### cos(angle) -> float

#### sin(angle) -> float

#### tan(angle) -> float

The trigonometry function. Note that the should be typed with `deg` or `rad` unit (for example `cos(90deg)` or `sin(slider.value * 1deg)`).

## FontWeight

Source: `reference/global-namespaces/font-weight/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/global-namespaces/font-weight/

The **FontWeight** namespace contains constants for various font weights, following the CSS specification.

### Constants

The following constants are available in the **FontWeight** namespace. They are all unitless `float` values.

| Name | Value | Description |
| --- | --- | --- |
| `thin` | `100` | Thin |
| `extra-light` | `200` | Extra Light |
| `light` | `300` | Light |
| `normal` | `400` | Normal |
| `medium` | `500` | Medium |
| `semi-bold` | `600` | Semi Bold |
| `bold` | `700` | Bold |
| `extra-bold` | `800` | Extra Bold |
| `black` | `900` | Black |

### Example

## Platform

Source: `reference/global-namespaces/platform/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/global-namespaces/platform/

The **Platform** namespace contains properties that help deal with platform specific differences.

### Properties

#### os

[enum OperatingSystemType](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#operatingsystemtype) default: `known at runtime`

This property holds the type of the operating system detected at run-time.

> **Note**
> When running in a web browser, the value of this property is computed at run-time by querying the web browser’s navigator properties.

> **Note**
> When Slint is ported to new operating systems in the future, new enum values will be added.

`OperatingSystemType`

This enum describes the detected operating system types.

- **`android`** : This variant includes any version of Android running mobile phones, tablets, as well as embedded Android devices.
- **`ios`** : This variant covers iOS running on iPhones and iPads.
- **`macos`** : This variant covers macOS running on Apple’s Mac computers.
- **`linux`** : This variant covers any version of Linux, except Android.
- **`windows`** : This variant covers Microsoft Windows.
- **`other`** : This variant is reported when the operating system is none of the above.

#### style-name

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `known at runtime`

The name of the currently selected [widget style](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/index.html). Some widget styles have dark and light variant suffixes, such as `fluent-light`. This property contains the style name without the suffix. Use [Palette](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/palette/index.html)‘s `color-scheme` to determine the currently used scheme.

### Functions

#### open-url(url: string) -> bool

Opens the specified URL in an external browser. This function invokes the platform’s URL opening mechanism. Returns `true` on success, or `false` if the platform doesn’t support opening URLs or the operation failed.

## Global Structs and Enums

Source: `reference/global-structs-enums/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/

### Structs

#### Edges

`Edges`

A structure representing the four edges of an axis-aligned rectangle

- **`left`** ( *length* ): The left edge value
- **`top`** ( *length* ): The top edge value
- **`right`** ( *length* ): The right edge value
- **`bottom`** ( *length* ): The bottom edge value

#### FontMetrics

`FontMetrics`

A structure to hold metrics of a font for a specified pixel size.

- **`ascent`** ( *length* ): The distance between the baseline and the top of the tallest glyph in the font.
- **`descent`** ( *length* ): The distance between the baseline and the bottom of the tallest glyph in the font. This is usually negative.
- **`x_height`** ( *length* ): The distance between the baseline and the horizontal midpoint of the tallest glyph in the font, or zero if not specified by the font.
- **`cap_height`** ( *length* ): The distance between the baseline and the top of a regular upper-case glyph in the font, or zero if not specified by the font.

#### KeyEvent

`KeyEvent`

This structure is generated and passed to the key press and release callbacks of the `FocusScope` element.

- **`text`** ( *string* ): The unicode representation of the key pressed.
- **`modifiers`** ( *KeyboardModifiers* ): The keyboard modifiers active at the time of the key press event.
- **`repeat`** ( *bool* ): This field is set to true for key press events that are repeated, i.e. the key is held down. It’s always false for key release events.

#### KeyboardModifiers

`KeyboardModifiers`

The `KeyboardModifiers` struct provides booleans to indicate possible modifier keys on a keyboard, such as Shift, Control, etc. It is provided as part of `KeyEvent`’s `modifiers` field.

Keyboard shortcuts on Apple platforms typically use the Command key (⌘), such as Command+C for “Copy”. On other platforms the same shortcut is typically represented using Control+C. To make it easier to develop cross-platform applications, on macOS, Slint maps the Command key to the control modifier, and the Control key to the meta modifier.

On Windows, the Windows key is mapped to the meta modifier.

- **`alt`** ( *bool* ): Indicates the Alt key on a keyboard.
- **`control`** ( *bool* ): Indicates the Control key on a keyboard, except on macOS, where it is the Command key (⌘).
- **`shift`** ( *bool* ): Indicates the Shift key on a keyboard.
- **`meta`** ( *bool* ): Indicates the Control key on macos, and the Windows key on Windows.

#### Point

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

#### PointerEvent

`PointerEvent`

Represents a Pointer event sent by the windowing system. This structure is passed to the `pointer-event` callback of the `TouchArea` element.

- **`button`** ( *PointerEventButton* ): The button that was pressed or released
- **`kind`** ( *PointerEventKind* ): The kind of the event
- **`modifiers`** ( *KeyboardModifiers* ): The keyboard modifiers pressed during the event

#### PointerScrollEvent

`PointerScrollEvent`

Represents a Pointer scroll (or wheel) event sent by the windowing system. This structure is passed to the `scroll-event` callback of the `TouchArea` element.

- **`delta_x`** ( *length* ): The amount of pixel in the horizontal direction
- **`delta_y`** ( *length* ): The amount of pixel in the vertical direction
- **`modifiers`** ( *KeyboardModifiers* ): The keyboard modifiers pressed during the event

#### Size

`Size`

This structure represents a size with width and height

- **`width`** ( *length* ):
- **`height`** ( *length* ):

#### StandardListViewItem

`StandardListViewItem`

Represents an item in a StandardListView and a StandardTableView.

- **`text`** ( *string* ): The text content of the item

#### TableColumn

`TableColumn`

This is used to define the column and the column header of a TableView

- **`title`** ( *string* ): The title of the column header
- **`min_width`** ( *length* ): The minimum column width (logical length)
- **`horizontal_stretch`** ( *float* ): The horizontal column stretch
- **`sort_order`** ( *SortOrder* ): Sorts the column
- **`width`** ( *length* ): the actual width of the column (logical length)

### Enums

#### AccessibleRole

`AccessibleRole`

This enum represents the different values for the `accessible-role` property, used to describe the role of an element in the context of assistive technology such as screen readers.

- **`none`** : The element isn’t accessible.
- **`button`** : The element is a `Button` or behaves like one.
- **`checkbox`** : The element is a `CheckBox` or behaves like one.
- **`combobox`** : The element is a `ComboBox` or behaves like one.
- **`groupbox`** : The element is a `GroupBox` or behaves like one.
- **`image`** : The element is an `Image` or behaves like one. This is automatically applied to `Image` elements.
- **`list`** : The element is a `ListView` or behaves like one.
- **`slider`** : The element is a `Slider` or behaves like one.
- **`spinbox`** : The element is a `SpinBox` or behaves like one.
- **`tab`** : The element is a `Tab` or behaves like one.
- **`tab-list`** : The element is similar to the tab bar in a `TabWidget` .
- **`tab-panel`** : The element is a container for tab content.
- **`text`** : The role for a `Text` element. This is automatically applied to `Text` elements.
- **`table`** : The role for a `TableView` or behaves like one.
- **`tree`** : The role for a TreeView or behaves like one. (Not provided yet)
- **`progress-indicator`** : The element is a `ProgressIndicator` or behaves like one.
- **`text-input`** : The role for widget with editable text such as a `LineEdit` or a `TextEdit` . This is automatically applied to `TextInput` elements.
- **`switch`** : The element is a `Switch` or behaves like one.
- **`list-item`** : The element is an item in a `ListView` .
- **`radio-button`** : The element is a `RadioButton` or behaves like one.

#### AnimationDirection

`AnimationDirection`

This enum describes the direction of an animation.

- **`normal`** : The [“normal” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#normal) .
- **`reverse`** : The [“reverse” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#reverse) .
- **`alternate`** : The [“alternate” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#alternate) .
- **`alternate-reverse`** : The [“alternate reverse” direction as defined in CSS↗](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction#alternate-reverse) .

#### ColorScheme

`ColorScheme`

This enum indicates the color scheme used by the widget style. Use this to explicitly switch between dark and light schemes, or choose Unknown to fall back to the system default.

- **`unknown`** : The scheme is not known and a system wide setting configures this. This could mean that the widgets are shown in a dark or light scheme, but it could also be a custom color scheme.
- **`dark`** : The style chooses light colors for the background and dark for the foreground.
- **`light`** : The style chooses dark colors for the background and light for the foreground.

#### DialogButtonRole

`DialogButtonRole`

This enum represents the value of the `dialog-button-role` property which can be added to any element within a `Dialog` to put that item in the button row, and its exact position depends on the role and the platform.

- **`none`** : This isn’t a button meant to go into the bottom row
- **`accept`** : This is the role of the main button to click to accept the dialog. e.g. “Ok” or “Yes”
- **`reject`** : This is the role of the main button to click to reject the dialog. e.g. “Cancel” or “No”
- **`apply`** : This is the role of the “Apply” button
- **`reset`** : This is the role of the “Reset” button
- **`help`** : This is the role of the “Help” button
- **`action`** : This is the role of any other button that performs another action.

#### EventResult

`EventResult`

This enum describes whether an event was rejected or accepted by an event handler.

- **`reject`** : The event is rejected by this event handler and may then be handled by the parent item
- **`accept`** : The event is accepted and won’t be processed further

#### FillRule

`FillRule`

This enum describes the different ways of deciding what the inside of a shape described by a path shall be.

- **`nonzero`** : The [“nonzero” fill rule as defined in SVG↗](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule#nonzero) .
- **`evenodd`** : The [“evenodd” fill rule as defined in SVG↗](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule#evenodd)

#### FocusReason

`FocusReason`

This enum describes the different reasons for a FocusEvent

- **`programmatic`** : A built-in function invocation caused the event ( `.focus()` , `.clear-focus()` )
- **`tab-navigation`** : Keyboard navigation caused the event (tabbing)
- **`pointer-click`** : A mouse click caused the event
- **`popup-activation`** : A popup caused the event
- **`window-activation`** : The window manager changed the active window and caused the event

#### ImageFit

`ImageFit`

This enum defines how the source image or path shall fit into an `Image` or `Path` element.

- **`fill`** : Scales and stretches the source to fit the width and height of the element.
- **`contain`** : The source is scaled to fit into the element’s dimensions while preserving the aspect ratio.
- **`cover`** : The source is scaled to cover the element’s dimensions while preserving the aspect ratio. If the aspect ratios don’t match, the source will be clipped to fit.
- **`preserve`** : Preserves the size of the source in logical pixels. The source will still be scaled by the scale factor that applies to all elements in the window. Any extra space will be left blank.

#### ImageHorizontalAlignment

`ImageHorizontalAlignment`

This enum specifies the horizontal alignment of the source image.

- **`center`** : Aligns the source image at the center of the `Image` element.
- **`left`** : Aligns the source image at the left of the `Image` element.
- **`right`** : Aligns the source image at the right of the `Image` element.

#### ImageRendering

`ImageRendering`

This enum specifies how the source image will be scaled.

- **`smooth`** : The image is scaled with a linear interpolation algorithm.
- **`pixelated`** : The image is scaled with the nearest neighbor algorithm.

#### ImageTiling

`ImageTiling`

This enum specifies how the source image will be tiled.

- **`none`** : The source image will not be tiled.
- **`repeat`** : The source image will be repeated to fill the `Image` element.
- **`round`** : The source image will be repeated and scaled to fill the `Image` element, ensuring an integer number of repetitions.

#### ImageVerticalAlignment

`ImageVerticalAlignment`

This enum specifies the vertical alignment of the source image.

- **`center`** : Aligns the source image at the center of the `Image` element.
- **`top`** : Aligns the source image at the top of the `Image` element.
- **`bottom`** : Aligns the source image at the bottom of the `Image` element.

#### InputType

`InputType`

This enum is used to define the type of the input field.

- **`text`** : The default value. This will render all characters normally
- **`password`** : This will render all characters with a character that defaults to ”*”
- **`number`** : This will only accept and render number characters (0-9)
- **`decimal`** : This will accept and render characters if it’s valid part of a decimal

#### LayoutAlignment

`LayoutAlignment`

Enum representing the `alignment` property of a `HorizontalBox`, a `VerticalBox`, a `HorizontalLayout`, or `VerticalLayout`.

- **`stretch`** : Use the minimum size of all elements in a layout, distribute remaining space based on `*-stretch` among all elements.
- **`center`** : Use the preferred size for all elements, distribute remaining space evenly before the first and after the last element.
- **`start`** : Use the preferred size for all elements, put remaining space after the last element.
- **`end`** : Use the preferred size for all elements, put remaining space before the first element.
- **`space-between`** : Use the preferred size for all elements, distribute remaining space evenly between elements.
- **`space-around`** : Use the preferred size for all elements, distribute remaining space evenly between the elements, and use half spaces at the start and end.
- **`space-evenly`** : Use the preferred size for all elements, distribute remaining space evenly before the first element, after the last element and between elements.

#### LineCap

`LineCap`

This enum describes the appearance of the ends of stroked paths.

- **`butt`** : The stroke ends with a flat edge that is perpendicular to the path.
- **`round`** : The stroke ends with a rounded edge.
- **`square`** : The stroke ends with a square projection beyond the path.

#### LineJoin

`LineJoin`

This enum describes the appearance of the joins between segments of stroked paths.

- **`miter`** : The stroke joins with a sharp corner or a clipped corner, depending on the miter limit.
- **`round`** : The stroke joins with a smooth, rounded corner.
- **`bevel`** : The stroke joins with a beveled (flattened) corner.

#### MouseCursor

`MouseCursor`

This enum represents different types of mouse cursors. It’s a subset of the mouse cursors available in CSS. For details and pictograms see the [MDN Documentation for cursor↗](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#values). Depending on the backend and used OS unidirectional resize cursors may be replaced with bidirectional ones.

- **`default`** : The systems default cursor.
- **`none`** : No cursor is displayed.
- **`help`** : A cursor indicating help information.
- **`pointer`** : A pointing hand indicating a link.
- **`progress`** : The program is busy but can still be interacted with.
- **`wait`** : The program is busy.
- **`crosshair`** : A crosshair.
- **`text`** : A cursor indicating selectable text.
- **`alias`** : An alias or shortcut is being created.
- **`copy`** : A copy is being created.
- **`move`** : Something is to be moved.
- **`no-drop`** : Something can’t be dropped here.
- **`not-allowed`** : An action isn’t allowed
- **`grab`** : Something is grabbable.
- **`grabbing`** : Something is being grabbed.
- **`col-resize`** : Indicating that a column is resizable horizontally.
- **`row-resize`** : Indicating that a row is resizable vertically.
- **`n-resize`** : Unidirectional resize north.
- **`e-resize`** : Unidirectional resize east.
- **`s-resize`** : Unidirectional resize south.
- **`w-resize`** : Unidirectional resize west.
- **`ne-resize`** : Unidirectional resize north-east.
- **`nw-resize`** : Unidirectional resize north-west.
- **`se-resize`** : Unidirectional resize south-east.
- **`sw-resize`** : Unidirectional resize south-west.
- **`ew-resize`** : Bidirectional resize east-west.
- **`ns-resize`** : Bidirectional resize north-south.
- **`nesw-resize`** : Bidirectional resize north-east-south-west.
- **`nwse-resize`** : Bidirectional resize north-west-south-east.

#### OperatingSystemType

`OperatingSystemType`

This enum describes the detected operating system types.

- **`android`** : This variant includes any version of Android running mobile phones, tablets, as well as embedded Android devices.
- **`ios`** : This variant covers iOS running on iPhones and iPads.
- **`macos`** : This variant covers macOS running on Apple’s Mac computers.
- **`linux`** : This variant covers any version of Linux, except Android.
- **`windows`** : This variant covers Microsoft Windows.
- **`other`** : This variant is reported when the operating system is none of the above.

#### Orientation

`Orientation`

Represents the orientation of an element or widget such as the `Slider`.

- **`horizontal`** : Element is oriented horizontally.
- **`vertical`** : Element is oriented vertically.

#### PathEvent

`PathEvent`

PathEvent is a low-level data structure describing the composition of a path. Typically it is generated at compile time from a higher-level description, such as SVG commands.

- **`begin`** : The beginning of the path.
- **`line`** : A straight line on the path.
- **`quadratic`** : A quadratic bezier curve on the path.
- **`cubic`** : A cubic bezier curve on the path.
- **`end-open`** : The end of the path that remains open.
- **`end-closed`** : The end of a path that is closed.

#### PointerEventButton

`PointerEventButton`

This enum describes the different types of buttons for a pointer event, typically on a mouse or a pencil.

- **`other`** : A button that is none of left, right, middle, back or forward. For example, this is used for the task button on a mouse with many buttons.
- **`left`** : The left button.
- **`right`** : The right button.
- **`middle`** : The center button.
- **`back`** : The back button.
- **`forward`** : The forward button.

#### PointerEventKind

`PointerEventKind`

The enum reports what happened to the `PointerEventButton` in the event

- **`cancel`** : The action was cancelled.
- **`down`** : The button was pressed.
- **`up`** : The button was released.
- **`move`** : The pointer has moved,

#### PopupClosePolicy

`PopupClosePolicy`

- **`close-on-click`** : Closes the `PopupWindow` when user clicks or presses the escape key.
- **`close-on-click-outside`** : Closes the `PopupWindow` when user clicks outside of the popup or presses the escape key.
- **`no-auto-close`** : Does not close the `PopupWindow` automatically when user clicks.

#### ScrollBarPolicy

`ScrollBarPolicy`

This enum describes the scrollbar visibility

- **`as-needed`** : Scrollbar will be visible only when needed
- **`always-off`** : Scrollbar never shown
- **`always-on`** : Scrollbar always visible

#### SortOrder

`SortOrder`

This enum represents the different values of the `sort-order` property. It’s used to sort a `StandardTableView` by a column.

- **`unsorted`** : The column is unsorted.
- **`ascending`** : The column is sorted in ascending order.
- **`descending`** : The column is sorted in descending order.

#### StandardButtonKind

`StandardButtonKind`

Use this enum to add standard buttons to a `Dialog`. The look and positioning of these `StandardButton`s depends on the environment (OS, UI environment, etc.) the application runs in.

- **`ok`** : A “OK” button that accepts a `Dialog` , closing it when clicked.
- **`cancel`** : A “Cancel” button that rejects a `Dialog` , closing it when clicked.
- **`apply`** : A “Apply” button that should accept values from a `Dialog` without closing it.
- **`close`** : A “Close” button, which should close a `Dialog` without looking at values.
- **`reset`** : A “Reset” button, which should reset the `Dialog` to its initial state.
- **`help`** : A “Help” button, which should bring up context related documentation when clicked.
- **`yes`** : A “Yes” button, used to confirm an action.
- **`no`** : A “No” button, used to deny an action.
- **`abort`** : A “Abort” button, used to abort an action.
- **`retry`** : A “Retry” button, used to retry a failed action.
- **`ignore`** : A “Ignore” button, used to ignore a failed action.

#### TextHorizontalAlignment

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

#### TextOverflow

`TextOverflow`

This enum describes the how the text appears if it is too wide to fit in the width of a `Text` or `StyledText` element.

- **`clip`** : The text will simply be clipped.
- **`elide`** : The text will be elided with `…` .

#### TextStrokeStyle

`TextStrokeStyle`

This enum describes the positioning of a text stroke relative to the border of the glyphs in a `Text` or `StyledText` element.

- **`outside`** : The inside edge of the stroke is at the outer edge of the text.
- **`center`** : The center line of the stroke is at the outer edge of the text, like in Adobe Illustrator.

#### TextVerticalAlignment

`TextVerticalAlignment`

This enum describes the different types of alignment of text along the vertical axis of a `Text` or `StyledText` element.

- **`top`** : The text will be aligned to the top of the containing box.
- **`center`** : The text will be vertically centered within the containing box.
- **`bottom`** : The text will be aligned to the bottom of the containing box.

#### TextWrap

`TextWrap`

This enum describes the how the text wraps if it is too wide to fit in the width of a `Text` or `StyledText` element.

- **`no-wrap`** : The text won’t wrap, but instead will overflow.
- **`word-wrap`** : The text will be wrapped at word boundaries if possible, or at any location for very long words.
- **`char-wrap`** : The text will be wrapped at any character. Currently only supported by the Qt and Software renderers.

## FocusScope

Source: `reference/keyboard-input/focusscope/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/focusscope/

The `FocusScope` can react to [keyboard shortcuts](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html#key-bindings) using the [KeyBinding element](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/focusscope/index.html#keybinding), and exposes callbacks to handle key events manually. Note that `FocusScope` will only handle key events when it either `has-focus`, or when it surrounds another FocusScope that `has-focus` (see [Key Event Delivery](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/focusscope/#key-event-delivery))

The [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) has a text property, which is a character of the key entered. When a non-printable key is pressed, the character will be either a control character, or it will be mapped to a private unicode character. The mapping of these non-printable, special characters is available in the [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) namespace

### Key Event Delivery

Key events are delivered to the element that `has-focus`.

Before attempting to deliver the `KeyEvent`, it is checked whether some other element wants to intercept the `KeyEvent`. Visiting all the elements starting at the Window, going down toward the focused element, `capture_key_pressed` or `capture_key_released` is called. If any of these returns `EventResult::accept`, then key event processing stops at this point. If `EventResult::reject` is returned, then event delivery continues.

If no element captures the `KeyEvent`, then the `KeyEvent` is delivered to the focused element by calling `key-pressed` or `key-released`. If these callbacks return `EventResult::accept`, then event delivery is finished and the event has been handled. Otherwise, (recursively) try to deliver the key event to the parent element.

### Properties

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Is `true` when the element has keyboard focus.

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When false, the FocusScope will not accept focus, neither via click nor via tab focus traversal, not even programmatically.

A parent `FocusScope` will still receive key events from child `FocusScope`s that were rejected, even if `enabled` is set to false.

#### focus-on-click

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When true, the `FocusScope` will make itself the focused element when clicked.

This property has no effect if the `enabled` property is set to false.

#### focus-on-tab-navigation

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When true, the `FocusScope` will accept focus as part of the tab focus traversal.

This property has no effect if the `enabled` property is set to false.

### Functions

#### focus()

Call this function to transfer keyboard focus to this `FocusScope`, to receive future [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html)s.

#### clear-focus()

Call this function to remove keyboard focus from this `FocusScope` if it currently has the focus. See also [FocusHandling](https://docs.slint.dev/latest/docs/slint/guide/development/focus/index.html).

### Callbacks

#### capture-key-pressed(event: KeyEvent) -> EventResult

This function is called during key event handling, *before* `key-pressed` is called. Use this to intercept key press events. The returned [EventResult](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#eventresult) indicates whether to accept or reject the event. Rejected events are forwarded to the parent element.

#### capture-key-released(event: KeyEvent) -> EventResult

This function is called during key event handling, *before* `key-released` is called. Use this to intercept key release events. The returned [EventResult](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#eventresult) indicates whether to accept or reject the event. Rejected events are forwarded to the parent element.

#### key-pressed(event: KeyEvent) -> EventResult

Invoked when a key is pressed, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. The returned [EventResult](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#eventresult) indicates whether to accept or reject the event. Rejected events are forwarded to the parent element.

#### key-released(event: KeyEvent) -> EventResult

Invoked when a key is released, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. The returned [EventResult](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#eventresult) indicates whether to accept or reject the event. Rejected events are forwarded to the parent element.

#### focus-changed-event(reason: FocusReason)

Invoked when the focus on the `FocusScope` has changed. The argument is a a [FocusReason](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#focusreason) enum containing the reason for focus change.

#### focus-gained(reason: FocusReason)

Invoked when the `FocusScope` gains focus. The argument is a a [FocusReason](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#focusreason) enum containing the reason for focus gain.

#### focus-lost(reason: FocusReason)

Invoked when the `FocusScope` loses focus. The argument is a a [FocusReason](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#focusreason) enum containing the reason for focus loss.

### `KeyBinding`

Place `KeyBinding` elements inside a `FocusScope` to declare keyboard shortcuts. KeyBindings use **logical keys**, based on the character a key produces, not physical key positions.

See [Key Bindings](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html#key-bindings) for details.

#### Properties of `KeyBinding`

##### keys

[keys](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#keys) default: `@keys()`

The [keys](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#keys) to match against incoming key events.

##### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

Whether this KeyBinding is currently enabled. Disabled KeyBinding elements don’t consume key events and never invoke their `activated()` callback.

#### Callbacks of `KeyBinding`

##### activated()

Invoked when the parent `FocusScope` receives a key event that matches the `keys` of this `KeyBinding`.

## Key Handling Overview

Source: `reference/keyboard-input/overview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/

To handle keyboard input in Slint, use the `FocusScope` with `KeyBinding` elements or individual `key-pressed` and `key-released` callbacks in various elements. Keyboard input is delivered via [`KeyEvent`](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#keyevent) data structures. The primary field of this data structure is the `text` property, which holds all affected keys encoded in a string. Use the [`Key` namespace](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#key-namespace) to identify known named keys.

See the [Key Bindings](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#key-bindings) section for how to handle key bindings declaratively with Slint’s [KeyBinding](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/focusscope/index.html#keybinding) elements and the built-in [keys](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#keys) type, which is constructed from the `@keys` macro.

### KeyEvent

This structure is generated and passed to the key press and release callbacks of the `FocusScope` element.

- **`text`** ( *string* ): The unicode representation of the key pressed.
- **`modifiers`** ( *KeyboardModifiers* ): The keyboard modifiers active at the time of the key press event.
- **`repeat`** ( *bool* ): This field is set to true for key press events that are repeated, i.e. the key is held down. It’s always false for key release events.

### Key Bindings

Use the [KeyBinding element](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/focusscope/index.html#keybinding) inside a `FocusScope` to declare key bindings (or keyboard shortcuts). The `KeyBinding`’s `keys` property takes an instance of the [keys type](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#keys) and invokes the `activated` callback when the key combination is detected.

The `keys` type represents a key, combined with modifiers and is constructed with the `@keys` macro.

Key bindings in Slint are based on **logical keys** — the character a keypress produces on the current keyboard layout — not the physical position of a key. This means, for example, that `@keys(Control + Z)` activates when the user presses the key that produces `z` on their layout, regardless of where that key sits on the keyboard. As a consequence, numpad digit keys are not distinguished from the main row digit keys, and `AltGr` isn’t available as a modifier (it is consumed by the OS to produce characters).

The `@keys(..)` macro accepts a key from the [`Key` namespace](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#key-namespace), combined with any of these modifiers:

- `Meta` (⌃ control on macOS and Windows key on PC)
- `Control` (⌘ command on macOS)
- `Shift`
- `Alt` (⌥ option on macOS)

> **Caution**
> The behavior of ambiguous KeyBinding elements is currently undefined, for example when two or more KeyBindings declare the same `keys`.
>
> If possible, conflicts are detected at compile-time. Ambiguous key bindings that are detected at runtime will log a warning.

#### Different Keyboard Layouts

Different keyboard layouts are handled automatically by the `@keys(..)` macro for keys from the [`Key` namespace](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#key-namespace). For example, `@keys(Control + Plus)` fires whenever the user presses the key that produces `+` on their layout — on a US layout that requires `Shift` + `=`, while on a German layout `+` has its own key and no `Shift` is needed.

Named keys inside `@keys(..)` are logical: `A` represents the character `a`, not a specific physical key location. This means `@keys(Control + A)` will activate on whichever key produces `a` on the user’s layout, regardless of where that key sits on the keyboard.

If you need to define a key binding that involves a key outside of the [`Key` namespace](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#key-namespace), continue reading the next section which covers advanced key definitions.

#### Advanced Key Bindings

If possible, prefer to use a key from the [`Key` namespace](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#key-namespace) inside `@keys(..)`. If you absolutely need to define a binding to a different key, you can use a string literal inside the `@keys(..)` macro. For example: `@keys(Control + "ä")`.

Note though that the `@keys(..)` macro no longer automatically handles different keyboard layouts and Shift behavior in this case. The provided string is simply matched against the **lowercased** `text`, plus the modifiers of the given `KeyEvent`

To prevent issues with characters that entirely change their keycode when `Shift` (or sometimes `Alt` on macOS) is applied, you can mark these modifiers as optional with a trailing `?`, e.g.:

- `Shift?`
- `Alt?`

When a modifier is marked as optional, the `keys` will ignore the state of the corresponding modifier when matching.

For example, to support key bindings that produce a Euro-Sign (€), add both `Shift?` and `Alt?`: `@keys(Control + Shift? + Alt? + "€")`. This will allow your users to reach the binding on almost all keyboard layouts that have a key with this label.

As another example, `@keys(Control + Plus)` is equivalent to `@keys(Control + Shift? + "+")`.

### Key Namespace

The `Key` namespace contains a list of well-known logical key codes (including any non-printable characters). Key names in this namespace identify keys by the **character they produce**, not by their physical position on the keyboard. For example, `Key.A` represents the character `a`, not the top-left letter key of a QWERTY keyboard.

Note that alphabetic keys (e.g. Key.A-Key.Z) are represented as lowercase (e.g. “a”-“z”). Make sure to case-convert the [`KeyEvent`](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/#keyevent) `text` or the Key as necessary.

- **`Backspace`**
- **`Tab`**
- **`Return`**
- **`Escape`**
- **`Backtab`**
- **`Delete`**
- **`Shift`**
- **`Control`**
- **`Alt`**
- **`AltGr`**
- **`CapsLock`**
- **`ShiftR`**
- **`ControlR`**
- **`Meta`**
- **`MetaR`**
- **`Space`**
- **`UpArrow`**
- **`DownArrow`**
- **`LeftArrow`**
- **`RightArrow`**
- **`F1`**
- **`F2`**
- **`F3`**
- **`F4`**
- **`F5`**
- **`F6`**
- **`F7`**
- **`F8`**
- **`F9`**
- **`F10`**
- **`F11`**
- **`F12`**
- **`F13`**
- **`F14`**
- **`F15`**
- **`F16`**
- **`F17`**
- **`F18`**
- **`F19`**
- **`F20`**
- **`F21`**
- **`F22`**
- **`F23`**
- **`F24`**
- **`Insert`**
- **`Home`**
- **`End`**
- **`PageUp`**
- **`PageDown`**
- **`ScrollLock`**
- **`Pause`**
- **`SysReq`**
- **`Stop`**
- **`Menu`**
- **`Back`**
- **`A`**
- **`B`**
- **`C`**
- **`D`**
- **`E`**
- **`F`**
- **`G`**
- **`H`**
- **`I`**
- **`J`**
- **`K`**
- **`L`**
- **`M`**
- **`N`**
- **`O`**
- **`P`**
- **`Q`**
- **`R`**
- **`S`**
- **`T`**
- **`U`**
- **`V`**
- **`W`**
- **`X`**
- **`Y`**
- **`Z`**
- **`Digit0`**
- **`Digit1`**
- **`Digit2`**
- **`Digit3`**
- **`Digit4`**
- **`Digit5`**
- **`Digit6`**
- **`Digit7`**
- **`Digit8`**
- **`Digit9`**
- **`Circumflex`**
- **`Exclamation`**
- **`DoubleQuote`**
- **`Hash`**
- **`Dollar`**
- **`Percent`**
- **`Ampersand`**
- **`Underscore`**
- **`OpenParen`**
- **`CloseParen`**
- **`Asterisk`**
- **`Plus`**
- **`Pipe`**
- **`HyphenMinus`**
- **`OpenCurlyBracket`**
- **`CloseCurlyBracket`**
- **`Tilde`**
- **`Colon`**
- **`Semicolon`**
- **`LessThan`**
- **`Equals`**
- **`GreaterThan`**
- **`QuestionMark`**
- **`At`**
- **`Comma`**
- **`Period`**
- **`Slash`**
- **`BackQuote`**
- **`OpenBracket`**
- **`BackSlash`**
- **`CloseBracket`**
- **`Quote`**

## TextInput

Source: `reference/keyboard-input/textinput/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/textinput/

The `TextInput` is a lower-level item that shows text and allows entering text. You should probably not use this directly, but instead use the [LineEdit](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/lineedit/index.html) or [TextEdit](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/textedit/index.html) component.

When not part of a layout, its width and height defaults to 100% of the parent element.

The `TextInput` does not scroll automatically when the cursor is outside of the visible area. This is the responsibility of the enclosing widget to ensure using the `cursor-position-changed` callback.

### Example

### Properties

#### color

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `depends on the style`

The color of the text.

#### font-family

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The name of the font family selected for rendering the text.

#### font-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The font size of the text.

#### font-weight

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The weight of the font. The values range from 100 (lightest) to 900 (thickest). 400 is the normal weight.

#### font-italic

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether or not the font face should be drawn italicized or not.

#### font-metrics

[struct FontMetrics](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#fontmetrics) `(out)` default: `a struct with all default values`

The design metrics of the font scaled to the font pixel size used by the element.

`FontMetrics`

A structure to hold metrics of a font for a specified pixel size.

- **`ascent`** ( *length* ): The distance between the baseline and the top of the tallest glyph in the font.
- **`descent`** ( *length* ): The distance between the baseline and the bottom of the tallest glyph in the font. This is usually negative.
- **`x_height`** ( *length* ): The distance between the baseline and the horizontal midpoint of the tallest glyph in the font, or zero if not specified by the font.
- **`cap_height`** ( *length* ): The distance between the baseline and the top of a regular upper-case glyph in the font, or zero if not specified by the font.

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

`TextInput` sets this to `true` when it’s focused. Only then it receives [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html)s.

#### horizontal-alignment

[enum TextHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#texthorizontalalignment) default: `the first enum value`

The horizontal alignment of the text.

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

#### input-type

[enum InputType](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#inputtype) default: `text`

Use this to configure `TextInput` for editing special input, such as password fields.

`InputType`

This enum is used to define the type of the input field.

- **`text`** : The default value. This will render all characters normally
- **`password`** : This will render all characters with a character that defaults to ”*”
- **`number`** : This will only accept and render number characters (0-9)
- **`decimal`** : This will accept and render characters if it’s valid part of a decimal

#### letter-spacing

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0`

The letter spacing allows changing the spacing between the glyphs. A positive value increases the spacing and a negative value decreases the distance.

#### page-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The height of the page used to compute how much to scroll when the user presses page up or page down.

#### read-only

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

When set to `true`, text editing via keyboard and mouse is disabled but selecting text is still enabled as well as editing text programmatically.

#### selection-background-color

[color](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#color) default: `a transparent color`

The background color of the selection.

#### selection-foreground-color

[color](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#color) default: `a transparent color`

The foreground color of the selection.

#### single-line

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When set to `true`, the text is always rendered as a single line, regardless of new line separators in the text.

#### text-cursor-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `provided at run-time by the selected widget style`

The width of the text cursor.

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) `(in-out)` default: `""`

The text rendered and editable by the user.

#### vertical-alignment

[enum TextVerticalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textverticalalignment) default: `the first enum value`

The vertical alignment of the text.

`TextVerticalAlignment`

This enum describes the different types of alignment of text along the vertical axis of a `Text` or `StyledText` element.

- **`top`** : The text will be aligned to the top of the containing box.
- **`center`** : The text will be vertically centered within the containing box.
- **`bottom`** : The text will be aligned to the bottom of the containing box.

#### wrap

[enum TextWrap](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textwrap) default: `no-wrap`

The way the text input wraps. Only makes sense when `single-line` is false.

`TextWrap`

This enum describes the how the text wraps if it is too wide to fit in the width of a `Text` or `StyledText` element.

- **`no-wrap`** : The text won’t wrap, but instead will overflow.
- **`word-wrap`** : The text will be wrapped at word boundaries if possible, or at any location for very long words.
- **`char-wrap`** : The text will be wrapped at any character. Currently only supported by the Qt and Software renderers.

### Functions

#### focus()

Call this function to focus the text input and make it receive future keyboard events.

#### clear-focus()

Call this function to remove keyboard focus from this `TextInput` if it currently has the focus. See also [FocusHandling](https://docs.slint.dev/latest/docs/slint/guide/development/focus/index.html).

#### set-selection-offsets(start: int, end: int)

Selects the text between two UTF-8 offsets.

#### select-all()

Selects all text.

#### clear-selection()

Clears the selection.

#### copy()

Copies the selected text to the clipboard.

#### cut()

Copies the selected text to the clipboard and removes it from the editable area.

#### paste()

Pastes the text content of the clipboard at the cursor position.

### Callbacks

#### accepted()

Invoked when the enter key is pressed.

#### cursor-position-changed(position: Point)

The cursor was moved to the new (x, y) position described by the `Point` argument.

#### edited()

Invoked when the text has changed because the user modified it.

#### key-pressed(event: KeyEvent) -> EventResult

Invoked when a key is pressed, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. Use this callback to handle keys before `TextInput` does. Return `accept` to indicate that you’ve handled the event, or return `reject` to let `TextInput` handle it.

#### key-released(event: KeyEvent) -> EventResult

Invoked when a key is released, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. Use this callback to handle keys before `TextInput` does. Return `accept` to indicate that you’ve handled the event, or return `reject` to let `TextInput` handle it.

### Accessibility

By default, `TextInput` elements have the following accessibility properties set:

- `accessible-role: text-input;`
- `accessible-value: text;`
- `accessible-enabled: enabled;`
- `accessible-read-only: read-only;`

## TextInputInterface

Source: `reference/keyboard-input/textinputinterface/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/textinputinterface/

### Properties

The `TextInputInterface.text-input-focused` property can be used to find out if a `TextInput` element has the focus. If you’re implementing your own virtual keyboard, this property is an indicator whether the virtual keyboard should be shown or hidden.

#### text-input-focused

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

True if an `TextInput` element has the focus; false otherwise.

## GridLayout

Source: `reference/layouts/gridlayout/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/layouts/gridlayout/

`GridLayout` places elements on a grid.

`GridLayout` covers its entire surface with cells. Cells are not aligned. The elements constituting the cells will be stretched inside their allocated space, unless their size constraints—like, e.g., `min-height` or `max-width`—work against this.

![gridlayout example](https://docs.slint.dev/latest/docs/slint/_astro/gridlayout-example1.D5Mfrnd-_22KTzp.webp)

```slint
// This example uses the `col` and `row` properties
export component Foo inherits Window {
    width: 200px;
    height: 150px;
    GridLayout {
        Rectangle { background: red; }
        Rectangle { background: blue; }
        Rectangle { background: yellow; row: 1; }
        Rectangle { background: green; }
        Rectangle { background: black; col: 2; row: 0; }
    }
}
```

![gridlayout example2](https://docs.slint.dev/latest/docs/slint/_astro/gridlayout-example2.DFg5vRwz_1n8Q0O.webp)

### Spacing Properties

#### spacing

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The distance between the elements in the layout. This single value is applied to both horizontal and vertical spacing.

To target specific axis with different values use the following properties:

#### spacing-horizontal

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### spacing-vertical

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

### Padding Properties

#### padding

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The padding around the grid structure as a whole. This single value is applied to all sides.

To target specific sides with different values use the following properties:

#### padding-left

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-right

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-top

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-bottom

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

### Cell elements

Cell elements inside a `GridLayout` obtain the following new properties. Any bindings to these properties must be compile-time constants:

#### row

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `auto`

The index of the element’s row within the grid. Setting this property resets the element’s column to zero, unless explicitly set.

#### col

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `auto`

The index of the element’s column within the grid. Set this property to override the sequential column assignment (e.g., to skip a column).

#### rowspan

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `1`

The number of rows this element should span.

#### colspan

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `1`

The number of columns this element should span.

To implicitly sequentially assign row indices—just like with `col`—wrap cell elements in `Row` elements.

The following example creates a 2-by-2 grid with `Row` elements, omitting one cell:

```slint
import { Button } from "std-widgets.slint";
export component Foo inherits Window {
    width: 200px;
    height: 100px;
    GridLayout {
        Row { // children implicitly on row 0
            Button { col: 1; text: "Top Right"; } // implicit column after this would be 2
        }
        Row { // children implicitly on row 1
            Button { text: "Bottom Left"; }  // implicitly in column 0...
            Button { text: "Bottom Right"; } // ...and 1
        }
    }
}
```

The following example creates the same grid using the `row` property. Row indices must be taken care of manually:

```slint
import { Button } from "std-widgets.slint";
export component Foo inherits Window {
    width: 200px;
    height: 100px;
    GridLayout {
        Button { row: 0; col: 1; text: "Top Right"; } // `row: 0;` could even be left out at the start
        Button { row: 1; text: "Bottom Left"; } // new row, implicitly resets column to 0
        Button { text: "Bottom Right"; } // same row, sequentially assigned column 1
    }
}
```

## HorizontalLayout

Source: `reference/layouts/horizontallayout/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/layouts/horizontallayout/

Places its children next to each other horizontally. The size of elements can either be fixed with the `width` or `height` property, or if they aren’t set they will be computed by the layout respecting the minimum and maximum sizes and the stretch factor.

### Spacing Properties

#### spacing

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The distance between the elements in the layout.

### Padding Properties

#### padding

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The padding within the layout as a whole. This single value is applied to all sides.

To target specific sides with different values use the following properties:

#### padding-left

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-right

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-top

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-bottom

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

### Alignment Properties

#### alignment

[enum LayoutAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#layoutalignment) default: `the first enum value`

Set the alignment. Matches the CSS flex box.

`LayoutAlignment`

Enum representing the `alignment` property of a `HorizontalBox`, a `VerticalBox`, a `HorizontalLayout`, or `VerticalLayout`.

- **`stretch`** : Use the minimum size of all elements in a layout, distribute remaining space based on `*-stretch` among all elements.
- **`center`** : Use the preferred size for all elements, distribute remaining space evenly before the first and after the last element.
- **`start`** : Use the preferred size for all elements, put remaining space after the last element.
- **`end`** : Use the preferred size for all elements, put remaining space before the first element.
- **`space-between`** : Use the preferred size for all elements, distribute remaining space evenly between elements.
- **`space-around`** : Use the preferred size for all elements, distribute remaining space evenly between the elements, and use half spaces at the start and end.
- **`space-evenly`** : Use the preferred size for all elements, distribute remaining space evenly before the first element, after the last element and between elements.

## Common Properties

Source: `reference/layouts/overview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/layouts/overview/

### Properties

These properties are valid on all visible items and can be used to specify constraints when used in layouts:

#### col, row

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

See [GridLayout](https://docs.slint.dev/latest/docs/slint/reference/layouts/gridlayout/index.html).

#### colspan, rowspan

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

See [GridLayout](https://docs.slint.dev/latest/docs/slint/reference/layouts/gridlayout/index.html).

#### horizontal-stretch, vertical-stretch

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) `(in-out)` default: `0.0`

Specify how much relative space these elements are stretching in a layout. When 0, this means that the elements won’t be stretched unless all elements are 0. Builtin widgets have a value of either 0 or 1.

#### max-width, max-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The maximum size of an element.

#### min-width, min-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The minimum size of an element.

#### preferred-width, preferred-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The preferred size of an element.

## VerticalLayout

Source: `reference/layouts/verticallayout/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/layouts/verticallayout/

Places its children next to each other vertically. The size of elements can either be fixed with the `width` or `height` property, or if they aren’t set they will be computed by the layout respecting the minimum and maximum sizes and the stretch factor.

### Spacing Properties

#### spacing

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The distance between the elements in the layout.

### Padding Properties

#### padding

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The padding within the layout as a whole. This single value is applied to all sides.

To target specific sides with different values use the following properties:

#### padding-left

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-right

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-top

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

#### padding-bottom

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

### Alignment Properties

#### alignment

[enum LayoutAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#layoutalignment) default: `the first enum value`

Set the alignment. Matches the CSS flex box.

`LayoutAlignment`

Enum representing the `alignment` property of a `HorizontalBox`, a `VerticalBox`, a `HorizontalLayout`, or `VerticalLayout`.

- **`stretch`** : Use the minimum size of all elements in a layout, distribute remaining space based on `*-stretch` among all elements.
- **`center`** : Use the preferred size for all elements, distribute remaining space evenly before the first and after the last element.
- **`start`** : Use the preferred size for all elements, put remaining space after the last element.
- **`end`** : Use the preferred size for all elements, put remaining space before the first element.
- **`space-between`** : Use the preferred size for all elements, distribute remaining space evenly between elements.
- **`space-around`** : Use the preferred size for all elements, distribute remaining space evenly between the elements, and use half spaces at the start and end.
- **`space-evenly`** : Use the preferred size for all elements, distribute remaining space evenly before the first element, after the last element and between elements.

## Reference Overview

Source: `reference/overview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/overview/

This section contains the API reference for all aspects of the Slint language. All the elements, properties, functions, callbacks and namespaces. It also contains the API reference for the `std-widgets` library. A set of cross platform components that can be used to build desktop applications.

## Types

Source: `reference/primitive-types/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/primitive-types/

Slint is a statically typed language and offers a rich range of primitive types.

### Primitive Types

#### bool

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

boolean whose value can be either `true` or `false`.

#### string

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

Any sequence of utf-8 encoded characters surrounded by quotes is a `string`: `"foo"`.

Escape sequences may be embedded into strings to insert characters that would be hard to insert otherwise:

| Escape | Result |
| --- | --- |
| `\"` | `"` |
| `\\` | `\` |
| `\n` | new line |
| `\u{x}` | where `x` is a hexadecimal number, expands to the unicode code point represented by this number |
| `\{expression}` | the result of evaluating the expression |

Anything else following an unescaped `\` is an error.

> **Note**
> The `\{...}` syntax is not valid within the `slint!` macro in Rust.

`is-empty` property is true when `string` doesn’t contain anything.

```slint
export component LengthOfString {
    property<bool> empty: "".is-empty; // true
    property<bool> not-empty: "hello".is-empty; // false
}
```

`character-count` property returns the number of [grapheme clusters↗](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries).

```slint
export component CharacterCountOfString {
    property<int> empty: "".character-count; // 0
    property<int> hello: "hello".character-count; // 5
    property<int> hiragana: "あいうえお".character-count; // 5
    property<int> surrogate-pair: "😊𩸽".character-count; // 2
    property<int> variation-selectors: "👍🏿".character-count; // 1
    property<int> combining-character: "パ".character-count; // 1
    property<int> zero-width-joiner: "👨‍👩‍👧‍👦".character-count; // 1
    property<int> region-indicator-character: "🇦🇿🇿🇦".character-count; // 2
    property<int> emoji-tag-sequences: "🏴󠁧󠁢󠁥󠁮󠁧󠁿".character-count; // 1
}
```

The `to-lowercase` and `to-uppercase` methods convert `string` to lowercase or uppercase according to the [Unicode Character Property↗](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-4/#G124722).

```slint
export component ChangeCaseOfString {
    property<string> hello: "HELLO".to-lowercase(); // "hello"
    property<string> bye: "tschüß".to-uppercase(); // "TSCHÜSS"
    property<string> odysseus: "ὈΔΥΣΣΕΎΣ".to-lowercase(); // "ὀδυσσεύς"
    property<string> new_year: "农历新年".to-uppercase(); // "农历新年"
}
```

The `to-float` method can be used to convert `string` to a `float`. It returns 0 if the string isn’t a valid number. You can check with `is-float` if the string contains a valid number.

```slint
export component StringToFloat {
    property<float> hello: "hello".to-float(); // 0
    property<bool> goodbye: "goodbye".is-float(); // false
    property<float> value: "1.5".to-float(); // 1.5
}
```

#### styled-text

[styled-text](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#styled-text) default: `""`

A property that is used with the [StyledText element](https://docs.slint.dev/latest/docs/slint/reference/elements/styled-text/index.html).

Create styled text with the `@markdown()` macro. For example: `@markdown("Hello **Bold**")`.

`@markdown()` supports interpolation: For example, `@markdown("5 + 5 = *\{ 5 + 5 }*")` becomes `"5 + 5 = *10*"`.

Any text passed as an argument to the macro will be escaped, for example `@markdown("Hello \{"*World*"}")` will become `"Hello \*World\*"`.

If multiple string literal are used, they are concatenated.

### Numeric Types

#### angle

[angle](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#angle) default: `0deg`

Angle measurement, corresponds to a literal like `90deg`, `1.2rad`, `0 25turn`

#### duration

[duration](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#duration) default: `0ms`

Type for the duration of animations. A suffix like `ms` (millisecond) or `s` (second) is used to indicate the precision.

#### float

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0`

Signed, 32-bit floating point number. Numbers with a `%` suffix are automatically divided by 100, so for example `30%` is the same as `0.30`.

#### int

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

Signed integral number.

#### length

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The type used for `x`, `y`, `width` and `height` coordinates. Corresponds to a literal like `1px`, `1pt`, `1in`, `1mm`, or `1cm`. It can be converted to and from length provided the binding is run in a context where there is an access to the device pixel ratio.

#### percent

[percent](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#percent) default: `0%`

Signed, 32-bit floating point number that is interpreted as percentage. Literal number assigned to properties of this type must have a `%` suffix.

#### physical-length

[physical-length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#physical-length) default: `0phx`

This is an amount of physical pixels. To convert from an integer to a length unit, one can simply multiply by `1px`. Or to convert from a length to a float, one can divide by `1phx`.

#### relative-font-size

[relative-font-size](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#relative-font-size) default: `0rem`

Relative font size factor that is multiplied with the `Window.default-font-size` and can be converted to a `length`.

Please see the language specific API references how these types are mapped to the APIs of the different programming languages.

### Color and Brush Types

#### brush

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `transparent`

A brush is a special type that can be either initialized from a `color` or a `gradient`. See [Colors & Brushes](https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/index.html).

#### color

[color](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#color) default: `transparent`

RGB color with an alpha channel, with 8 bit precision for each channel. CSS color names as well as the hexadecimal color encodings are supported, such as #RRGGBBAA or #RGB. See [Colors & Brushes](https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/index.html).

### Keyboard input

#### keys

[keys](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#keys) default: `empty keys`

A keys represents a key combined with a list of modifiers. This is the primitive type used to detect if a KeyEvent should trigger a given key binding.

Key bindings in Slint are based on **logical keys** — the character a key produces on the current keyboard layout — not the physical position of a key on the keyboard. See [Key Bindings](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html#key-bindings) for details.

Use the `to-string` function to convert a `keys` instance to `string`. It returns a platform-native description of the key combination.

```slint
export component KeysToString inherits FocusScope {
    undo := KeyBinding {
        keys: @keys(Control + Z);
        activated => { debug("UNDO") }
    }

    Text {
        text: "Press \{undo.keys.to-string()} to undo!";
        // Results in "Press ⌘Z to undo!" on macOS
        // Results in "Press Ctrl+Z to undo!" on other platforms
    }
}
```

### Images

#### image

[image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) default: `empty image`

The `image` type is a reference to an image.

In Slint, an image can be loaded from a file with the `@image-url("...")` construct. The address within the `@image-url` function must be a string literal and the image is resolved at compile time.

Slint looks for images in the following places:

1. The absolute path or the path relative to the current `.slint` file.
2. The include path used by the compiler to look up `.slint` files.

Loading image from `http` is only supported in [SlintPad↗](https://slintpad.com/).

Images can also be loaded from [`data:` URIs↗](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data), with either base64 or URL-encoded content. For example: `@image-url("data:image/png;base64,iVBORw0KGgo...")`.

Supported format are SVG, and formats supported by the [`image` crate↗](https://crates.io/crates/image): AVIF, BMP, DDS, Farbfeld, GIF, HDR, ICO, JPEG, EXR, PNG, PNM, QOI, TGA, TIFF, WebP.

For Rust applications, not all formats are enabled by default. Enable them with the `image-default-formats` Cargo feature.

**C++**

In C++, properties or struct fields of the image type are mapped to [`slint::Image`](https://docs.slint.dev/latest/docs/cpp/api/structslint_1_1Image).

**Rust**

In Rust, properties or struct fields of the image type are mapped to [`slint::Image`](https://docs.slint.dev/latest/docs/rust/slint/struct.Image).

> **Note**
> Some image formats can be disabled using cargo features to reduce binary size and speed up compilation.

**NodeJS**

In JavaScript properties or struct fields of the image type are mapped an object that implement the [ImageData interface](https://docs.slint.dev/latest/docs/node/interfaces/ImageData.html).

**Python**

In Python, properties or struct fields of the image type are mapped to [`Image`](https://docs.slint.dev/latest/docs/python/slint.html#Image).

Access an `image`’s dimension using its `width` and `height` properties.

```slint
export component Example inherits Window {
    preferred-width: 150px;
    preferred-height: 50px;

    // Note: http URL only work on the web version.
    in property <image> some_image: @image-url("https://slint.dev/logo/slint-logo-full-light.svg");

    HorizontalLayout {
        Text {
            text: "The image is " + some_image.width + "x" + some_image.height;
        }

        // Check the size to find out if the image is empty.
        if some_image.width > 0 : Image {
            source: some_image;
        }
    }
}
```

It is also possible to load images supporting [9 slice scaling↗](https://en.wikipedia.org/wiki/9-slice_scaling) (also called nine patch or border images) by adding a `nine-slice(...)` argument. The argument can have either one, two, or four numbers that specifies the size of the edges. The numbers are either `top right bottom left` or `vertical horizontal`, or one number for everything

```slint
// nine-slice scaling
export component Example inherits Window {
    width: 100px;
    height: 150px;
    VerticalLayout {
        Image {
            source: @image-url("https://interactive-examples.mdn.mozilla.net/media/examples/border-diamonds.png", nine-slice(30 30 30 30));
        }
    }
}
```

See also the [Image element](https://docs.slint.dev/latest/docs/slint/reference/elements/image/index.html).

### Animation

#### easing

[easing](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#easing) default: `linear`

The `easing` type allows defining an easing curve for animations.

To specify an easing curve, use the values from the `Easing` namespace. For example you can use `Easing.ease-out` or `Easing.ease-in-quad`. The namespace consists of the following names (see [`easings.net`↗](https://easings.net/) for a visual reference):

- `linear`
- `ease-in-quad`
- `ease-out-quad`
- `ease-in-out-quad`
- `ease`
- `ease-in`
- `ease-out`
- `ease-in-out`
- `ease-in-quart`
- `ease-out-quart`
- `ease-in-out-quart`
- `ease-in-quint`
- `ease-out-quint`
- `ease-in-out-quint`
- `ease-in-expo`
- `ease-out-expo`
- `ease-in-out-expo`
- `ease-in-sine`
- `ease-out-sine`
- `ease-in-out-sine`
- `ease-in-back`
- `ease-out-back`
- `ease-in-out-back`
- `ease-in-circ`
- `ease-out-circ`
- `ease-in-out-circ`
- `ease-in-elastic`
- `ease-out-elastic`
- `ease-in-out-elastic`
- `ease-in-bounce`
- `ease-out-bounce`
- `ease-in-out-bounce`
- `cubic-bezier(a, b, c, d)` as in CSS

Additionally, in expressions of type `easing`, those names are available directly.

```slint
struct AnimationData {
    curve: easing,
}

component Custom inherits Rectangle {
    property<AnimationData> animation: {
        // Using the Easing namespace.
        curve: Easing.ease-in-circ,
    };

  animate x {
      // In easing expressions the names are available via global scope.
      easing: ease-out-bounce;
  }
}
```

### Type Conversions

Slint supports conversions between different types. Explicit conversions are required to make the UI description more robust, but implicit conversions are allowed between some types for convenience.

The following conversions are possible:

- `int` can be converted implicitly to `float` and vice-versa. When converting from `float` to `int` , the value is truncated.
- `int` and `float` can be converted implicitly to `string`
- `physical-length` , `relative-font-size` , and `length` can be converted implicitly to each other only in context where the pixel ratio is known.
- the units type ( `length` , `physical-length` , `duration` , …) can’t be converted to numbers ( `float` or `int` ) but they can be divided by themselves to result in a number. Similarly, a number can be multiplied by one of these unit. The idea is that one would multiply by `1px` or divide by `1px` to do such conversions
- The literal `0` can be converted to any of these types that have associated unit.
- Struct types convert with another struct type if they have the same property names and their types can be converted. The source struct can have either missing properties, or extra properties. But not both.
- Arrays generally don’t convert between each other. Array literals can be converted if the element types are convertible.
- String can be converted to float by using the `to-float` function. That function returns 0 if the string isn’t a valid number. You can check with `is-float()` if the string contains a valid number
- `float` can be converted to a formatted `string` using `to-fixed` and `to-precision` which can be passed the number of digits after the decimal point and and the number of significant digits respectively. They behave like their JavaScript counterparts [`toFixed()`↗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toFixed) and [`toPrecision()`↗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision) .

```slint
export component Example {
    // OK: int converts to string
    property<{a: string, b: int}> prop1: {a: 12, b: 12 };
    // OK: even if a is missing, it will just have the default value ("")
    property<{a: string, b: int}> prop2: { b: 12 };
    // OK: even if c is too many, it will be discarded
    property<{a: string, b: int}> prop3: { a: "x", b: 12, c: 42 };
    // ERROR: b is missing and c is extra, this doesn't compile, because it could be a typo.
    // property<{a: string, b: int}> prop4: { a: "x", c: 42 };

    property<string> xxx: "42.1";
    property<float> xxx1: xxx.to-float(); // 42.1
    property<bool> xxx2: xxx.is-float(); // true
    property<int> xxx3: 45.8; // 45
}
```

## Button

Source: `reference/std-widgets/basic-widgets/button/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/button/

A simple button. Common types of buttons can also be created with [StandardButton](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/standardbutton/index.html).

### Properties

#### checkable

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Shows whether the button can be checked or not. This enables the `checked` property to possibly become true.

```slint
Button {
    text: "Checkable Button";
    checkable: true;
}
```

#### checked

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in-out)` default: `false`

Shows whether the button is checked or not. Needs `checkable` to be true to work.

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

Defaults to true. When false, the button cannot be pressed.

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the button has keyboard focus

#### icon

[image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) default: `the empty image`

The image to show in the button. Note that not all styles support drawing icons.

#### icon-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The size of the icon shown in the button. The default value depends on the style. The button will grow if needed to accommodate for large icon sizes.

#### pressed

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the button is pressed.

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text written in the button.

```slint
Button {
    text: "Button with text";
}
```

#### primary

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

If set to true the button is displayed with the primary accent color.

#### colorize-icon

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

If set to true, the icon will be colorized to the same color as the Button’s text color.

### Callbacks

#### clicked()

Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.

```slint
Button {
    text: "Click me";
    clicked() => {
        debug("Button clicked");
    }
}
```

## CheckBox

Source: `reference/std-widgets/basic-widgets/checkbox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/checkbox/

![checkbox example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-checkbox-example.DJKED1JN_ZXzSJO.webp)

Use a `CheckBox` to let the user select or deselect values, for example in a list with multiple options. Consider using a `Switch` element instead if the action resembles more something that’s turned on or off.

### Properties

#### checked

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in-out)` default: `false`

Whether the checkbox is checked or not.

```slint
CheckBox {
    text: self.checked ? "Checked" : "Not checked";
    checked: true;
}
```

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

Defaults to true. When false, the checkbox can’t be pressed.

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the checkbox has keyboard focus.

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text written next to the checkbox.

```slint
CheckBox {
    text: "CheckBox with text";
}
```

### Callbacks

#### toggled()

The checkbox value changed

```slint
CheckBox {
    text: "CheckBox";
    toggled() => {
        debug("CheckBox checked: ", self.checked);
    }
}
```

## ComboBox

Source: `reference/std-widgets/basic-widgets/combobox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/combobox/

![combobox example](https://docs.slint.dev/latest/docs/slint/_astro/std-widget-combobox-example.Bw2NqGak_1pjrMK.webp)

A button that, when clicked, opens a popup to select a value.

### Properties

#### current-index

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) `(in-out)` default: `0`

The index of the selected value.

```slint
ComboBox {
    model: ["first", "second", "third"];
    current-index: 1;
}
```

#### current-value

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) `(in-out)` default: `""`

The currently selected text

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

Defaults to true. When false, the combobox can’t be interacted with

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the combobox has keyboard focus.

#### model

[[string]](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `[]`

The list of possible values

```slint
ComboBox {
    model: ["first", "second", "third"];
}
```

> **Note**
> When setting this property, the `ComboBox` will try to preserve the `current-index`. If the `current-index` is larger than the new model length, it is set to the last item in the model.

### Callbacks

#### selected(string)

A value was selected from the combo box by the user. The argument is the currently selected value.

```slint
ComboBox {
    model: ["first", "second", "third"];
    selected(value) => {
        debug("Selected value: ", value);
    }
}
```

## ProgressIndicator

Source: `reference/std-widgets/basic-widgets/progressindicator/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/progressindicator/

![progressindicator example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-progressindicator.DSJBxvj8_Z2fBpFn.webp)

The `ProgressIndicator` informs the user about the status of an on-going operation, such as loading data from the network.

### Properties

#### indeterminate

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Set to true if the progress of the operation cannot be determined by value.

#### progress

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0`

Percentage of completion, as value between 0 and 1. Values less than 0 or greater than 1 are capped.

```slint
ProgressIndicator {
    progress: 0.5;
}
```

## Slider

Source: `reference/std-widgets/basic-widgets/slider/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/slider/

![slider example](https://docs.slint.dev/latest/docs/slint/_astro/slider-example.Dwxrs7tX_2rLH8m.webp)

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

You can’t interact with the slider if enabled is false.

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the slider currently has the focus

#### value

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) `(in-out)` default: `0`

The value. Defaults to the minimum.

```slint
Slider {
    value: 50;
}
```

#### step

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `1`

The change step when pressing arrow key.

```slint
Slider {
    step: 1;
}
```

#### minimum

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0`

The minimum value.

```slint
Slider {
    minimum: 10;
    value: 11;
}
```

#### maximum

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `100`

The maximum value.

```slint
Slider {
    maximum: 10;
    value: 9;
}
```

#### orientation

[enum Orientation](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#orientation) default: `horizontal`

Describes the orientation of the Slider, vertical or horizontal.

`Orientation`

Represents the orientation of an element or widget such as the `Slider`.

- **`horizontal`** : Element is oriented horizontally.
- **`vertical`** : Element is oriented vertically.

### Callbacks

#### changed(float)

The value was changed

```slint
Slider {
    changed(value) => {
        debug("New value: ", value);
    }
}
```

#### released(float)

Invoked when the user completed changing the slider’s value, i.e. when the press on the knob was released or the arrow keys lifted.

```slint
Slider {
    released(position) => {
        debug("Released at position: ", position);
    }
}
```

## SpinBox

Source: `reference/std-widgets/basic-widgets/spinbox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/spinbox/

![spinbox example](https://docs.slint.dev/latest/docs/slint/_astro/spinbox-example.Bl_Uigby_Z29pyOV.webp)

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

You can’t interact with the spinbox if enabled is false.

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the spinbox currently has the focus.

#### value

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) `(in-out)` default: `0`

The value. Defaults to the minimum.

```slint
SpinBox {
    value: 50;
}
```

#### minimum

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The minimum value.

```slint
SpinBox {
    minimum: 10;
    value: 11;
}
```

#### maximum

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `100`

The maximum value.

```slint
SpinBox {
    maximum: 10;
    value: 9;
}
```

#### read-only

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

If true, the user can’t modify the value.

#### step-size

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `1`

The size that is used on increment or decrement of `value`.

#### horizontal-alignment

[enum TextHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#texthorizontalalignment) default: `left`

The horizontal alignment of the text.

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

### Callbacks

#### edited(int)

Emitted when the value has changed because the user modified it

```slint
SpinBox {
    edited(value) => {
        debug("New value: ", value);
    }
}
```

## Spinner

Source: `reference/std-widgets/basic-widgets/spinner/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/spinner/

![std-widgets spinner example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-spinner.Dt114VA8_Z1fMsf3.webp)

The `Spinner` informs the user about the status of an on-going operation, such as loading data from the network. It provides the same properties as [ProgressIndicator](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/progressindicator/index.html) but differs in shape.

### Properties

### indeterminate

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Set to true if the progress of the operation cannot be determined by value.

### progress

[float](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#float) default: `0.0`

Percentage of completion, as value between 0 and 1. Values less than 0 or greater than 1 are capped.

```slint
Spinner {
    progress: 0.5;
}
```

## StandardButton

Source: `reference/std-widgets/basic-widgets/standardbutton/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/standardbutton/

![std-widgets standardbutton example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-standardbutton.3G3PLzIU_Ajjqk.webp)

The StandardButton looks like a button, but instead of customizing with `text` and `icon`, it can used one of the pre-defined `kind` and the text and icon will depend on the style.

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Defaults to true. When false, the button can’t be pressed

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the button currently has the focus

#### kind

[enum StandardButtonKind](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#standardbuttonkind) default: `the first enum value`

The kind of button, one of `ok` `cancel`, `apply`, `close`, `reset`, `help`, `yes`, `no,` `abort`, `retry` or `ignore`

```slint
StandardButton {
    kind: ok;
}
```

`StandardButtonKind`

Use this enum to add standard buttons to a `Dialog`. The look and positioning of these `StandardButton`s depends on the environment (OS, UI environment, etc.) the application runs in.

- **`ok`** : A “OK” button that accepts a `Dialog` , closing it when clicked.
- **`cancel`** : A “Cancel” button that rejects a `Dialog` , closing it when clicked.
- **`apply`** : A “Apply” button that should accept values from a `Dialog` without closing it.
- **`close`** : A “Close” button, which should close a `Dialog` without looking at values.
- **`reset`** : A “Reset” button, which should reset the `Dialog` to its initial state.
- **`help`** : A “Help” button, which should bring up context related documentation when clicked.
- **`yes`** : A “Yes” button, used to confirm an action.
- **`no`** : A “No” button, used to deny an action.
- **`abort`** : A “Abort” button, used to abort an action.
- **`retry`** : A “Retry” button, used to retry a failed action.
- **`ignore`** : A “Ignore” button, used to ignore a failed action.

#### primary

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

If set to true the button is displayed with the primary accent color.

#### pressed

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the button is pressed.

### Callbacks

#### clicked()

Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.

```slint
StandardButton {
    clicked() => {
        debug("Button clicked");
    }
}
```

## Switch

Source: `reference/std-widgets/basic-widgets/switch/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/basic-widgets/switch/

![std-widgets switch example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-switch.B0gsRI8B_Z26uOWd.webp)

A `Switch` is a representation of a physical switch that allows users to turn things on or off. Consider using a `CheckBox` instead if you want the user to select or deselect values, for example in a list with multiple options.

### Properties

#### checked

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in-out)` default: `false`

Whether the switch is checked or not.

```slint
Switch {
    text: self.checked ? "Checked" : "Not checked";
    checked: true;
}
```

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

When false, the switch can’t be pressed

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the switch has keyboard focus

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text written next to the switch.

```slint
Switch {
    text: "Switch with text";
}
```

### Callbacks

#### toggled()

The switch value changed

```slint
Switch {
    text: "Switch";
    toggled() => {
        debug("CheckBox checked: ", self.checked);
    }
}
```

## Palette

Source: `reference/std-widgets/globals/palette/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/palette/

Use `Palette` to create custom widgets that match the colors of the selected style e.g. fluent, cupertino, material, or qt.

See [Widget Styles](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/index.html) for details on the available styles.

### Properties

#### background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the default background brush. Use this if none of the more specialized background brushes apply.

#### foreground

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the foreground brush that is used for content that is displayed on `background` brush.

#### alternate-background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines an alternate background brush that is used for example for text input controls or panels like a side bar.

#### alternate-foreground

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the foreground brush that is used for content that is displayed on `alternate-background` brush.

#### control-background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the default background brush for controls, such as push buttons, combo boxes, etc.

#### control-foreground

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the foreground brush that is used for content that is displayed on `control-background` brush.

#### accent-background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the background brush for highlighted controls such as primary buttons.

#### accent-foreground

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the foreground brush that is used for content that is displayed on `accent-background` brush.

#### selection-background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the background brush that is used to highlight a selection such as a text selection.

#### selection-foreground

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the foreground brush that is used for content that is displayed on `selection-background` brush.

#### border

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) `(out)` default: `a transparent brush`

Defines the brush that is used for borders such as separators and widget borders.

#### color-scheme

[enum ColorScheme](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#colorscheme) `(in-out)` default: `the first enum value`

Read this property to determine the color scheme used by the palette. Set this property to force a dark or light color scheme. All styles except for the Qt style support setting a dark or light color scheme.

`ColorScheme`

This enum indicates the color scheme used by the widget style. Use this to explicitly switch between dark and light schemes, or choose Unknown to fall back to the system default.

- **`unknown`** : The scheme is not known and a system wide setting configures this. This could mean that the widgets are shown in a dark or light scheme, but it could also be a custom color scheme.
- **`dark`** : The style chooses light colors for the background and dark for the foreground.
- **`light`** : The style chooses dark colors for the background and light for the foreground.

## StyleMetrics

Source: `reference/std-widgets/globals/stylemetrics/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/stylemetrics/

Use `StyleMetrics` to create custom widgets that match the layout settings of the selected style e.g. fluent, cupertino, material, or qt.

See [Widget Styles](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/index.html) for details on the available styles.

### Properties

#### layout-spacing

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

Defines the default layout spacing. This spacing is also used by `VerticalBox`, `HorizontalBox` and `GridBox`.

#### layout-padding

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

Defines the default layout padding. This padding is also used by `VerticalBox`, `HorizontalBox` and `GridBox`.

## GridBox

Source: `reference/std-widgets/layouts/gridbox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/layouts/gridbox/

A `GridBox` is a [GridLayout](https://docs.slint.dev/latest/docs/slint/reference/layouts/gridlayout/index.html) with default `padding` and `spacing` from [StyleMetrics](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/stylemetrics/index.html), providing theme-appropriate values. Use `GridLayout` directly when you need full control over spacing and padding.

## GroupBox

Source: `reference/std-widgets/layouts/groupbox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/layouts/groupbox/

![groupbox example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-groupbox-example.BuvLxhRj_n5AJH.webp)

A `GroupBox` is a container that groups its children together under a common title.

### Properties

#### content-padding

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `Depends on the style`

The padding within the layout of the content as a whole. This single value is applied to all sides.

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When false, the groupbox can’t be interacted with

#### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

A text written as the title of the group box.

## HorizontalBox

Source: `reference/std-widgets/layouts/horizontalbox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/layouts/horizontalbox/

A `HorizontalBox` is a [HorizontalLayout](https://docs.slint.dev/latest/docs/slint/reference/layouts/horizontallayout/index.html) with default `padding` and `spacing` from [StyleMetrics](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/stylemetrics/index.html), providing theme-appropriate values. Use `HorizontalLayout` directly when you need full control over spacing and padding.

## VerticalBox

Source: `reference/std-widgets/layouts/verticalbox/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/layouts/verticalbox/

A `VerticalBox` is a [VerticalLayout](https://docs.slint.dev/latest/docs/slint/reference/layouts/verticallayout/index.html) with default `padding` and `spacing` from [StyleMetrics](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/stylemetrics/index.html), providing theme-appropriate values. Use `VerticalLayout` directly when you need full control over spacing and padding.

## AboutSlint

Source: `reference/std-widgets/misc/aboutslint/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/misc/aboutslint/

![About Slint component](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-about-slint.CrRTnUJ3_1dbKaS.webp)

This element displays a “Made with Slint” badge.

## DatePickerPopup

Source: `reference/std-widgets/misc/datepicker/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/misc/datepicker/

Use a date picker to let the user select a date.

### Properties

#### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text that is displayed at the top of the picker.

#### date

[struct Date](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#date) default: `a struct with all default values`

Set the initial displayed date.

```slint
DatePickerPopup {
    date: { year: 2024, month: 11 };
}
```

`Date`

Defines a date with day, month, and year.

- **`day`(int)** : The day value (range from 1 to 31).
- **`month`(int)** : The month value (range from 1 to 12).
- **`year`(int)** : The year value.

### Callbacks

#### canceled()

Invoked when the cancel button is clicked.

```slint
date-picker := DatePickerPopup {
    canceled() => {
        date-picker.close();
    }
}
```

#### accepted(Date)

Invoked when the ok button is clicked.

```slint
date-picker := DatePickerPopup {
    accepted(date) => {
        debug("Selected date: ", date);
        date-picker.close();
    }
}
```

## TimePickerPopup

Source: `reference/std-widgets/misc/timepicker/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/misc/timepicker/

Use the timer picker to select the time, in either 24-hour or 12-hour mode (AM/PM).

![std-widgets timepicker example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-timepicker.DYUJged__183XjG.webp)

### Properties

#### use-24-hour-format

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `system default`

If set to `true` 24 hours are displayed otherwise it is displayed in AM/PM mode. (default: system default, if cannot be determined then `true`)

#### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text that is displayed at the top of the picker.

#### time

[struct Time](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#time) default: `a struct with all default values`

Set the initial displayed time.

```slint
TimePickerPopup {
    time: { hour: 12, minute: 24 };
}
```

`Time`

Defines a time with hours, minutes, and seconds.

- **`hour`(int)** : The hour value (range from 0 to 23).
- **`minute`(int)** : The minute value (range from 1 to 59).
- **`second`(int)** : The second value (range form 1 to 59).

### Callbacks

#### canceled()

The cancel button was clicked.

```slint
time-picker := TimePickerPopup {
    canceled() => {
        time-picker.close();
    }
}
```

#### accepted(Time)

The ok button was clicked.

```slint
time-picker := TimePickerPopup {
    accepted(time) => {
        debug("Selected time: ", time);
        time-picker.close();
    }
}
```

## Overview

Source: `reference/std-widgets/overview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/overview/

Slint provides a series of built-in widgets that can be imported from `"std-widgets.slint"`.

The widget appearance depends on the selected style. See [Selecting a Widget Style](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/index.html) for details how to select the style and how to use the `Palette` and `StyleMetrics` properties. If no style is selected, `fluent` is the default on all platforms.

All widgets support all [properties common to builtin elements](https://docs.slint.dev/latest/docs/slint/reference/common/index.html).

## Widget Styles

Source: `reference/std-widgets/style/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/

You can modify the look of these widgets by choosing a style.

The styles available include:

| Style Name | Light Variant | Dark Variant | Description |
| --- | --- | --- | --- |
| `fluent` | `fluent-light` | `fluent-dark` | These variants belong to the **Fluent** style, which is based on the [Fluent Design System↗](https://fluent2.microsoft.design/). |
| `material` | `material-light` | `material-dark` | These variants are part of the **Material** style, which follows the [Material Design↗](https://m3.material.io/). |
| `cupertino` | `cupertino-light` | `cupertino-dark` | The **Cupertino** variants emulate the style used by macOS. |
| `cosmic` | `cosmic-light` | `cosmic-dark` | The **Cosmic** variants emulate the style used by [Cosmic Desktop↗](https://github.com/pop-os/cosmic). |
| `qt` |  |  | The **Qt** style uses [Qt↗](https://en.wikipedia.org/wiki/Qt_(software)) to render widgets. This style requires Qt to be installed on your system. |
| `native` |  |  | This is an alias to one of the other styles depending on the platform. It is `cupertino` on macOS, `fluent` on Windows, `material` on Android, `qt` on linux if Qt is available, or `fluent` otherwise. |

By default, the styles automatically adapt to the system’s dark or light color setting. Select a `-light` or `-dark` variant to override the system setting and always show either dark or light colors.

The widget style is determined at your project’s compile time. The method to select a style depends on how you use Slint.

If no style is selected, `native` is the default.

**Rust**

You can select the style before starting your compilation by setting the `SLINT_STYLE` environment variable to the name of your chosen style.

When using the `slint_build` API, call the [`slint_build::compile_with_config()`↗](https://docs.rs/slint-build/newest/slint_build/fn.compile_with_config.html) function.

When using the `slint_interpreter` API, call the [`slint_interpreter::ComponentCompiler::set_style()`↗](https://docs.rs/slint-interpreter/newest/slint_interpreter/struct.ComponentCompiler.html#method.set_style) function.

**C++**

Define a `SLINT_STYLE` CMake cache variable to contain the style name as a string. This can be done, for instance, on the command line:

**NodeJS**

You can select the style by setting the `style` property in [`LoadFileOptions`](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/slint-node_interfaces/LoadFileOptions.html) passed to [`loadFile`](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/slint-node_functions/loadFile.html):

```js
import * as slint from "slint-ui";
let ui = slint.loadFile("main.slint", { style: "fluent" });
let main = new ui.Main();
main.greeting = "Hello friends";
```

**Python**

You can specify the style by setting the `SLINT_STYLE` environment variable in the beginning of your Python script:

```python
import slint
import os

os.environ["SLINT_STYLE"] = "material"
# or
# os.environ.setdefault("SLINT_STYLE", "material")

class MainWindow(slint.loader.ui.app.AppWindow): ...

main_window = MainWindow()
main_window.show()
main_window.run()
```

### Using Style Properties In Your Own Components

The global [Palette](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/palette/index.html) and [StyleMetrics](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/stylemetrics/index.html) properties can be accessed and will be set to the appropriate values of the current style.

```slint
import { Palette, StyleMetrics } from "std-widgets.slint";

export component Example inherits Window {
    Rectangle {
        border-radius: StyleMetrics.layout-padding;
        border-width: 2px;
        border-color: Palette.border;
        background: Palette.background;
    }
}
```

In situations where the specific property is not available you can detect the style via `Platform.style-name`.

```slint
import { Palette } from "std-widgets.slint";

export component Example inherits Window {
    Rectangle {
        border-radius: Platform.style-name == "fluent" ? 4px : 2px;
        border-width: Platform.style-name == "fluent" ? 2px : 1px;
        border-color: Palette.border;
        background: Palette.background;
    }
}
```

### Previewing Designs With `slint-viewer`

Select the style either by setting the `SLINT_STYLE` environment variable, or by passing the style name with the `--style` argument:

slint-viewer —style material /path/to/design.slint

### Previewing Designs With The Slint Visual Studio Code Extension

To select the style, first open the Visual Studio Code settings editor:

**Windows**

File > Preferences > Settings

**macOS**

Code > Preferences > Settings

**Linux**

File > Preferences > Settings

Then enter the style name in Extensions > Slint > Preview:Style

### Previewing Designs With The Generic LSP Process

Choose the style by setting the `SLINT_STYLE` environment variable before launching the process. Alternatively, if your IDE integration allows for command line parameters, you can specify the style using `--style`.

## LineEdit

Source: `reference/std-widgets/views/lineedit/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/lineedit/

![lineedit example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-lineedit-example.BDQNSHL-_ZhPGD5.webp)

A widget used to enter a single line of text. See [TextEdit](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/textedit/index.html) for a widget able to handle several lines of text.

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When false, nothing can be entered.

#### font-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The size of the font of the input text

#### font-family

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The font family of the input text

#### font-italic

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

The italic state of the font of the input text

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the line edit currently has the focus

#### horizontal-alignment

[enum TextHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#texthorizontalalignment) default: `left`

The horizontal alignment of the text.

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

#### input-type

[enum InputType](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#inputtype) default: `text`

The way to allow special input viewing properties such as password fields.

```slint
LineEdit {
    input-type: password;
}
```

`InputType`

This enum is used to define the type of the input field.

- **`text`** : The default value. This will render all characters normally
- **`password`** : This will render all characters with a character that defaults to ”*”
- **`number`** : This will only accept and render number characters (0-9)
- **`decimal`** : This will accept and render characters if it’s valid part of a decimal

#### placeholder-text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

A placeholder text being shown when there is no text in the edit field

#### read-only

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

When set to true, text editing via keyboard and mouse is disabled but selecting text is still enabled as well as editing text programmatically.

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) `(in-out)` default: `""`

The text being edited

```slint
LineEdit {
    text: "Initial text";
}
```

### Functions

#### focus()

Call this function to focus the LineEdit and make it receive future keyboard events.

#### clear-focus()

Call this function to remove keyboard focus from this `LineEdit` if it currently has the focus. See also [FocusHandling](https://docs.slint.dev/latest/docs/slint/guide/development/focus/index.html).

#### set-selection-offsets(int, int)

Selects the text between two UTF-8 offsets.

#### select-all()

Selects all text.

#### clear-selection()

Clears the selection. This function takes effect regardless of the `read-only` and `enabled` properties.

#### copy()

Copies the selected text to the clipboard.

#### cut()

Copies the selected text to the clipboard and removes it from the editable area. This function takes effect regardless of the `read-only` and `enabled` properties.

#### paste()

Pastes the text content of the clipboard at the cursor position. This function takes effect regardless of the `read-only` and `enabled` properties.

### Callbacks

#### accepted(string)

Invoked when the enter key is pressed.

```slint
LineEdit {
    accepted(text) => {
        debug("Accepted: ", text);
    }
}
```

#### edited(string)

Emitted when the text has changed because the user modified it

```slint
LineEdit {
    edited(text) => {
        debug("Text edited: ", text);
    }
}
```

#### key-pressed(KeyEvent) -> EventResult

Invoked when a key is pressed, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. Use this callback to handle keys before `LineEdit` does. Return `accept` to indicate that you’ve handled the event, or return `reject` to let `LineEdit` handle it.

#### key-released(KeyEvent) -> EventResult

Invoked when a key is released, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. Use this callback to handle keys before `LineEdit` does. Return `accept` to indicate that you’ve handled the event, or return `reject` to let `LineEdit` handle it.

## ListView

Source: `reference/std-widgets/views/listview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/listview/

A ListView is like a Scrollview but it should have a `for` element, and the content are automatically laid out in a list. Elements are only instantiated if they are visible

![listview example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-listview-example.bamRhNxT_1suHu2.webp)

### Properties

Same as [ScrollView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/scrollview/index.html).

### Callbacks

Same as [ScrollView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/scrollview/index.html).

## ScrollView

Source: `reference/std-widgets/views/scrollview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/scrollview/

![scrollview example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-scrollview.CurC16H7_Z2aGS3y.webp)

A Scrollview contains a viewport that is bigger than the view and can be scrolled. It has scrollbar to interact with.

The ScrollView is scrollable if the `viewport-width` and `viewport-height` properties are bigger than the size of the visible area.

If the ScrollView contains a layout, the default value for the `viewport-width` and `viewport-height` is the minimum size of that layout.

### Properties

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

Used to render the frame as disabled or enabled, but doesn’t change behavior of the widget.

#### mouse-drag-pan-enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in)` default: `true for Material style, false for all others`

When true, the view can be scrolled by dragging with the mouse.

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in-out)` default: `false`

Used to render the frame as focused or unfocused, but doesn’t change the behavior of the widget.

#### viewport-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The width of the viewport of the scrollview.

#### viewport-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The height of the viewport of the scrollview.

#### viewport-x

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The `x` position of the scrollview relative to the viewport. This is usually a negative value.

#### viewport-y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The `y` position of the scrollview relative to the viewport. This is usually a negative value.

#### visible-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

The width of the visible area of the ScrollView (not including the scrollbar)

#### visible-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

The height of the visible area of the ScrollView (not including the scrollbar)

#### vertical-scrollbar-policy

[enum ScrollBarPolicy](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#scrollbarpolicy) default: `as-needed`

The vertical scroll bar visibility policy.

`ScrollBarPolicy`

This enum describes the scrollbar visibility

- **`as-needed`** : Scrollbar will be visible only when needed
- **`always-off`** : Scrollbar never shown
- **`always-on`** : Scrollbar always visible

#### horizontal-scrollbar-policy

[enum ScrollBarPolicy](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#scrollbarpolicy) default: `as-needed`

The horizontal scroll bar visibility policy.

`ScrollBarPolicy`

This enum describes the scrollbar visibility

- **`as-needed`** : Scrollbar will be visible only when needed
- **`always-off`** : Scrollbar never shown
- **`always-on`** : Scrollbar always visible

### Callbacks

#### scrolled()

Invoked when `viewport-x` or `viewport-y` is changed by a user action (dragging, scrolling).

```slint
ScrollView {
    width: 200px;
    height: 200px;
    viewport-width: 300px;
    viewport-height: 300px;
    Rectangle { width: 30px; height: 30px; x: 275px; y: 50px; background: blue; }
    Rectangle { width: 30px; height: 30px; x: 175px; y: 130px; background: red; }
    Rectangle { width: 30px; height: 30px; x: 25px; y: 210px; background: yellow; }
    Rectangle { width: 30px; height: 30px; x: 98px; y: 55px; background: orange; }

    scrolled() => {
        debug("viewport-x: ", self.viewport-x);
        debug("viewport-y: ", self.viewport-y);
    }
}
```

## StandardListView

Source: `reference/std-widgets/views/standardlistview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/standardlistview/

![std-widgets standardlistview example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-standardlistview.DNGsmM9e_UMhWr.webp)

Like ListView, but with a default delegate, and a `model` property.

### Properties

Same as [ListView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/listview/index.html), and in addition:

#### current-item

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) `(in-out)` default: `0`

The index of the currently active item. -1 mean none is selected, which is the default

#### model

[struct StandardListViewItem](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#standardlistviewitem) default: `a struct with all default values`

The model.

```slint
StandardListView {
    model: [{ text: "Blue" }, { text: "Red" }, { text: "Green" }];
}
```

`StandardListViewItem`

Represents an item in a StandardListView and a StandardTableView.

- **`text`** ( *string* ): The text content of the item

### Functions

#### set-current-item(int)

Sets the current item by the specified index and brings it into view.

### Callbacks

#### current-item-changed(int)

Emitted when the current item has changed because the user modified it

```slint
StandardListView {
    model: [{ text: "Blue" }, { text: "Red" }, { text: "Green" }];
    current-item-changed(index) => {
        debug("Current item: ", index);
    }
}
```

#### item-pointer-event(int, PointerEvent, Point)

Emitted on any mouse pointer event similar to `TouchArea`. Arguments are item index associated with the event, the `PointerEvent` itself and the mouse position within the listview.

## StandardTableView

Source: `reference/std-widgets/views/standardtableview/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/standardtableview/

The `StandardTableView` represents a table of data with columns and rows. Cells are organized in a model where each row is a model of

[struct StandardListViewItem](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#standardlistviewitem) default: `a struct with all default values`

The model of items in each row.

`StandardListViewItem`

Represents an item in a StandardListView and a StandardTableView.

- **`text`** ( *string* ): The text content of the item

![std-widgets standardtableview example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-standardtableview.BxTVdxTQ_Z2gHWsW.webp)

### Properties

Same as [ListView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/listview/index.html), and in addition:

#### current-sort-column

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) `(out)` default: `0`

Indicates the sorted column. -1 mean no column is sorted.

#### columns

[[struct]](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html) `(in-out)` default: `a struct with all default values`

Defines the model of the table columns.

```slint
StandardTableView {
    columns: [{ title: "Header 1" }, { title: "Header 2" }];
    rows: [[{ text: "Item 1" }, { text: "Item 2" }]];
}
```

`TableColumn`

This is used to define the column and the column header of a TableView

- **`title`** ( *string* ): The title of the column header
- **`min_width`** ( *length* ): The minimum column width (logical length)
- **`horizontal_stretch`** ( *float* ): The horizontal column stretch
- **`sort_order`** ( *SortOrder* ): Sorts the column
- **`width`** ( *length* ): the actual width of the column (logical length)

#### rows

[[[struct]]](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html) `(in-out)` default: `a struct with all default values`

Defines the model of table rows.

```slint
StandardTableView {
    columns: [{ title: "Header 1" }, { title: "Header 2" }];
    rows: [[{ text: "Item 1" }, { text: "Item 2" }]];
}
```

`StandardListViewItem`

Represents an item in a StandardListView and a StandardTableView.

- **`text`** ( *string* ): The text content of the item

#### current-row

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) `(in-out)` default: `0`

The index of the currently active row. -1 mean none is selected, which is the default.

### Callbacks

#### sort-ascending(int)

Emitted if the model should be sorted by the given column in ascending order.

#### sort-descending(int)

Emitted if the model should be sorted by the given column in descending order.

#### row-pointer-event(int, PointerEvent, Point)

Emitted on any mouse pointer event similar to `TouchArea`. Arguments are row index associated with the event, the `PointerEvent` itself and the mouse position within the tableview.

#### current-row-changed(int)

Emitted when the current row has changed because the user modified it

```slint
StandardTableView {
    columns: [{ title: "Header 1" }, { title: "Header 2" }];
    rows: [[{ text: "Item 1" }, { text: "Item 2" }]];

    current-row-changed(index) => {
        debug("Current row: ", index);
    }
}
```

### Functions

#### set-current-row(int)

Sets the current row by index and brings it into view.

## TabWidget

Source: `reference/std-widgets/views/tabwidget/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/tabwidget/

![std-widgets tabwidget example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-tabwidget.C13G1sQo_aX6Im.webp)

`TabWidget` is a container for a set of tabs. It can only have `Tab` elements as children and only one tab will be visible at a time.

### Properties

#### current-index

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The index of the currently visible tab.

```slint
TabWidget {
    current-index: 1;

    Tab {
        title: "First";
    }
    Tab {
        title: "Second";
    }
}
```

#### orientation

[enum Orientation](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#orientation) default: `horizontal`

The orientation of the tab bar. When set to `vertical`, the tab bar is placed to the left of the content. The property must be constant. For example:

```slint
TabWidget {
    in property <Orientation> foo;
    // orientation: foo; // error
    orientation: Orientation.vertical; //valid
    Tab {
        title: "First";
    }
}
```

`Orientation`

Represents the orientation of an element or widget such as the `Slider`.

- **`horizontal`** : Element is oriented horizontally.
- **`vertical`** : Element is oriented vertically.

### Properties of the `Tab` element

#### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The text written on the tab.

```slint
TabWidget {
    Tab {
        title: "First";
    }
}
```

## TextEdit

Source: `reference/std-widgets/views/textedit/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/textedit/

![std-widgets textedit example](https://docs.slint.dev/latest/docs/slint/_astro/std-widgets-textedit.ClIET09k_Z2fRoaM.webp)

Similar to [LineEdit](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/lineedit/index.html), but can be used to enter several lines of text

### Properties

#### font-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0px`

The size of the font of the input text.

#### font-family

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The name of the font family selected for rendering the text.

#### font-italic

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

The italic state of the font of the input text

#### text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) `(in-out)` default: `""`

The text being edited

```slint
TextEdit {
    text: "Initial text";
}
```

#### has-focus

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(out)` default: `false`

Set to true when the widget currently has the focus.

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When false, nothing can be entered.

#### read-only

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

When set to true, text editing via keyboard and mouse is disabled but selecting text is still enabled as well as editing text programmatically.

#### wrap

[enum TextWrap](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#textwrap) default: `the first enum value`

The way the text wraps (default: word-wrap).

`TextWrap`

This enum describes the how the text wraps if it is too wide to fit in the width of a `Text` or `StyledText` element.

- **`no-wrap`** : The text won’t wrap, but instead will overflow.
- **`word-wrap`** : The text will be wrapped at word boundaries if possible, or at any location for very long words.
- **`char-wrap`** : The text will be wrapped at any character. Currently only supported by the Qt and Software renderers.

#### horizontal-alignment

[enum TextHorizontalAlignment](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#texthorizontalalignment) default: `the first enum value`

The horizontal alignment of the text.

`TextHorizontalAlignment`

This enum describes the different types of alignment of text along the horizontal axis of a `Text` or `StyledText` element.

- **`start`** : The text will be aligned with the start edge of the containing box. This could be left or right depending on the direction of the text.
- **`end`** : The text will be aligned with the end edge of the containing box. This could be left or right depending on the direction of the text.
- **`left`** : The text will be aligned with the left edge of the containing box.
- **`center`** : The text will be horizontally centered within the containing box.
- **`right`** : The text will be aligned to the right of the containing box.

#### placeholder-text

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) `(in)` default: `""`

A placeholder text being shown when there is no text in the edit field.

#### viewport-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The width of the viewport of the text edit.

#### viewport-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The height of the viewport of the text edit.

#### viewport-x

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The `x` position of the viewport relative to the text edit. This is usually a negative value.

#### viewport-y

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(in-out)` default: `0px`

The `y` position of the viewport relative to the text edit. This is usually a negative value.

#### visible-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

The width of the visible area of the text edit (not including the scrollbar)

#### visible-height

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) `(out)` default: `0px`

The height of the visible area of the text edit (not including the scrollbar)

### Functions

- **`focus()`** Call this function to focus the TextEdit and make it receive future keyboard events.
- **`clear-focus()`** Call this function to remove keyboard focus from this `TextEdit` if it currently has the focus. See also [focus handling](https://docs.slint.dev/latest/docs/slint/guide/development/focus/index.html) .
- **`set-selection-offsets(int, int)`** Selects the text between two UTF-8 offsets.
- **`select-all()`** Selects all text.
- **`clear-selection()`** Clears the selection. This function takes effect regardless of the `read-only` and `enabled` properties.
- **`copy()`** Copies the selected text to the clipboard.
- **`cut()`** Copies the selected text to the clipboard and removes it from the editable area. This function takes effect regardless of the `read-only` and `enabled` properties.
- **`paste()`** Pastes the text content of the clipboard at the cursor position. This function takes effect regardless of the `read-only` and `enabled` properties.

### Callbacks

#### edited(string)

Emitted when the text has changed because the user modified it

```slint
TextEdit {
    edited(text) => {
        debug("Edited: ", text);
    }
}
```

#### key-pressed(KeyEvent) -> EventResult

Invoked when a key is pressed, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. Use this callback to handle keys before `TextEdit` does. Return `accept` to indicate that you’ve handled the event, or return `reject` to let `TextEdit` handle it.

#### key-released(KeyEvent) -> EventResult

Invoked when a key is released, the argument is a [KeyEvent](https://docs.slint.dev/latest/docs/slint/reference/keyboard-input/overview/index.html) struct. Use this callback to handle keys before `TextEdit` does. Return `accept` to indicate that you’ve handled the event, or return `reject` to let `TextEdit` handle it.

## Timer

Source: `reference/timer/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/timer/

> **Note**
> Timer is not an actual element visible in the tree, therefore it doesn’t have the common properties such as `x`, `y`, `width`, `height`, etc. It also doesn’t take room in a layout and cannot have any children or be inherited from.

This example shows a timer that counts down from 10 to 0 every second:

Use the Timer pseudo-element to schedule a callback at a given interval. The timer is only running when the `running` property is set to `true`. To stop or start the timer, set that property to `true` or `false`. It can be also set to a binding expression. When already running, the timer will be restarted if the `interval` property is changed.

> **Caution**
> By default the `Timer` is always running `running: true`. This can result in constant CPU usage and power usage so ensure that you set `running` to `false` when you don’t want the timer to run.

```slint
property <int> count: 0;
Timer {
    interval: 8s; // every 8 seconds the timer will activate (tick)
    triggered() => { // The triggered callback activates every time the timer ticks
        if count >= 5 {
            self.running = false; // stop the timer after 5 ticks
        }
        count += 1;
    }
}
```

### Properties

#### interval

[duration](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#duration) default: `0ms`

The interval between timer ticks. This property is mandatory.

```slint
Timer {
    property <int> count: 0;
    interval: 250ms;
    triggered() => {
        debug("count is:", count);
        count += 1;
    }
}
```

#### running

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

`true` if the timer is running.

```slint
Timer {
    property <int> count: 0;
    interval: 250ms;
    running: false; // timer is not running
    triggered() => {
        debug("count is:", count);
    }
}
```

### Callbacks

#### triggered()

Invoked every time the timer ticks (every `interval`).

```slint
Timer {
    property <int> count: 0;
    interval: 250ms;
    triggered() => {
        debug("count is:", count);
    }
}
```

### Functions

#### start()

Start the timer (equivalent to setting `running` to true).

#### stop()

Stop the timer (equivalent to setting `running` to false).

#### restart()

Restarts the timer if it was previously started.

## ContextMenuArea

Source: `reference/window/contextmenuarea/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/window/contextmenuarea/

Use the non-visual `ContextMenuArea` element to declare an area where the user can show a context menu.

The context menu is shown if the user right-clicks on the area covered by the `ContextMenuArea` element, or if the user presses the “Menu” key on their keyboard while a `FocusScope` within the `ContextMenuArea` has focus. On Android, the menu is shown with a long press. Call the `show()` function on the `ContextMenuArea` element to programmatically show the context menu.

One of the children of the `ContextMenuArea` must be a `Menu` element, which defines the menu to be shown. There can be at most one `Menu` child, all other children must be of a different type and will be shown as regular visual children. Define the structure of the menu by placing `MenuItem` or `Menu` elements inside that `Menu`.

### Function

#### show(Point)

Call this function to programmatically show the context menu at the given position relative to the `ContextMenuArea` element.

### close()

Close the context menu if it’s currently open.

#### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When disabled, the `Menu` is not showing.

### `Menu`

Place the `Menu` element in a [MenuBar](https://docs.slint.dev/latest/docs/slint/reference/window/menubar/index.html), a `ContextMenuArea`, or within another `Menu`. Use `MenuItem` children of individual menu items, `Menu` children to create sub-menus, and `MenuSeparator` to create separators.

#### Properties of `Menu`

##### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

This is the label of the menu as written in the menu bar or in the parent menu.

##### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When disabled, the `Menu` can be selected but not activated.

#### icon

[image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) default: `the empty image`

The icon shown next to the title when in a parent menu.

### `MenuItem`

A `MenuItem` represents a single menu entry. It must be a child of a `Menu` element.

#### Properties of `MenuItem`

##### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The title shown for this menu item.

##### enabled

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `true`

When disabled, the `MenuItem` can be selected but not activated.

##### checkable

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

When true, the `MenuItem` can be checked. The value of the `checked` property is toggled when the user activates the menu item.

##### checked

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in-out)` default: `false`

When true, a checkmark will be shown next to the title of the `MenuItem`.

#### icon

[image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) default: `the empty image`

The icon shown next to the title.

#### shortcut

[keys](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#keys) default: `@keys()`

The keyboard shortcut for this `MenuItem`.

This property can only be set in a `MenuItem` that is part of a [MenuBar](https://docs.slint.dev/latest/docs/slint/reference/window/menubar/index.html).

#### Callbacks of `MenuItem`

##### activated()

Invoked when the menu entry is activated.

### `MenuSeparator`

A `MenuSeparator` represents a separator in a menu. It cannot have children, and doesn’t have properties or callbacks. MenuSeparator at the beginning or end of a menu will not be visible. Consecutive `MenuSeparator`s will be merged into one.

### Example

## Dialog

Source: `reference/window/dialog/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/window/dialog/

![dialog example](https://docs.slint.dev/latest/docs/slint/_astro/dialog-example.BIhbSMTr_ldrA5.webp)

Dialog can be used in place of [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html), but it has buttons that are automatically laid out.

A Dialog should have one main element as child, that isn’t a button. The dialog can have any number of `StandardButton` widgets or other buttons with the `dialog-button-role` property. The buttons will be placed in an order that depends on the target platform at run-time.

The `kind` property of the `StandardButton`s and the `dialog-button-role` properties need to be set to a constant value, it can’t be an arbitrary variable expression. There can’t be several `StandardButton`s of the same kind.

A callback `<kind>_clicked` is automatically added for each `StandardButton` which doesn’t have an explicit callback handler, so it can be handled from the native code: For example if there is a button of kind `cancel`, a `cancel_clicked` callback will be added. Each of these automatically-generated callbacks is an alias for the `clicked` callback of the associated `StandardButton`.

### Properties

Same as [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html).

### Functions

Same as [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html).

## MenuBar

Source: `reference/window/menubar/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/window/menubar/

Use the `MenuBar` element in a [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html) to declare the structure of a menu bar, including the actual menus and sub-menus.

> **Note**
> There can only be one `MenuBar` element in a `Window` and it must not be in a `for` or a `if`.

The `MenuBar` doesn’t have properties, but it must contain [Menu](https://docs.slint.dev/latest/docs/slint/reference/window/contextmenuarea/index.html#menu) as children that represent top level entries in the menu bar.

Depending on the platform, the menu bar might be native or rendered by Slint. This means that for example, on macOS, the menu bar will be at the top of the screen. The `width` and `height` property of the [Window](https://docs.slint.dev/latest/docs/slint/reference/window/window/index.html) define the client area, excluding the menu bar. The `x` and `y` properties of `Window` children are also relative to the client area.

#### Example

## PopupWindow

Source: `reference/window/popupwindow/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/window/popupwindow/

Use this element to show a popup window like a tooltip or a popup menu.

> **Note**
> It isn’t allowed to access properties of elements within the popup from outside of the `PopupWindow`. See [#4438↗](https://github.com/slint-ui/slint/issues/4438).

### Properties

#### close-policy

[enum PopupClosePolicy](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#popupclosepolicy) default: `close-on-click`

By default, a PopupWindow closes when the user clicks. Set this to false to prevent that behavior and close it manually using the `close()` function.

`PopupClosePolicy`

- **`close-on-click`** : Closes the `PopupWindow` when user clicks or presses the escape key.
- **`close-on-click-outside`** : Closes the `PopupWindow` when user clicks outside of the popup or presses the escape key.
- **`no-auto-close`** : Does not close the `PopupWindow` automatically when user clicks.

### Functions

#### show()

Show the popup on the screen.

#### close()

Closes the popup. Use this if you set the `close-policy` property to `no-auto-close`.

## Window

Source: `reference/window/window/`
Official URL: https://docs.slint.dev/latest/docs/slint/reference/window/window/

`Window` is the root of the tree of elements that are visible on the screen.

The `Window` geometry will be restricted by its layout constraints: Setting the `width` will result in a fixed width, and the window manager will respect the `min-width` and `max-width` so the window can’t be resized bigger or smaller. The initial width can be controlled with the `preferred-width` property. The same applies to the `Window`s height.

Use the [MenuBar](https://docs.slint.dev/latest/docs/slint/reference/window/menubar/index.html) element to declare a menu bar for the window.

### Properties

#### always-on-top

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the window should be placed above all other windows on window managers supporting it.

#### full-screen

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) `(in-out)` default: `true if 'SLINT_FULLSCREEN' environment variable is set, otherwise false`

Whether to display the Window in full-screen mode. In full-screen mode the Window will occupy the entire screen, it will not be resizable, and it will not display the title bar.

#### background

[brush](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#brush) default: `depends on the style`

The background brush of the `Window`.

#### default-font-family

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The font family to use as default in text elements inside this window, that don’t have their `font-family` property set.

#### default-font-size

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0`

The font size to use as default in text elements inside this window, that don’t have their `font-size` property set. The value of this property also forms the basis for relative font sizes.

#### default-font-weight

[int](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#int) default: `0`

The font weight to use as default in text elements inside this window, that don’t have their `font-weight` property set. The values range from 100 (lightest) to 900 (thickest). 400 is the normal weight. Use the [FontWeight](https://docs.slint.dev/latest/docs/slint/reference/global-namespaces/font-weight/index.html) namespace for predefined constants.

#### icon

[image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#image) default: `the empty image`

The window icon shown in the title bar or the task bar on window managers supporting it.

#### no-frame

[bool](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#bool) default: `false`

Whether the window should be borderless/frameless or not.

#### resize-border-width

[length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#length) default: `0`

> **Caution**
> This property is `winit` only for now.

Size of the resize border in borderless/frameless windows.

#### title

[string](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/index.html#string) default: `""`

The window title that is shown in the title bar.

#### safe-area-insets

[struct Edges](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#edges) `(out)` default: `a struct with all default values`

Some devices, such as mobile phones, allow programs to overlap the system UI. A few examples for this are the notch on iPhones, the window buttons on macOS on windows that extend their content over the titlebar and the system bar on Android. This property exposes the amount of space at the edges of the window that can be drawn to but where no interactive elements should be placed. On most devices, this is 0 for all sides.

`Edges`

A structure representing the four edges of an axis-aligned rectangle

- **`left`** ( *length* ): The left edge value
- **`top`** ( *length* ): The top edge value
- **`right`** ( *length* ): The right edge value
- **`bottom`** ( *length* ): The bottom edge value

#### virtual-keyboard-position

[struct Point](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#point) `(out)` default: `a struct with all default values`

On mobile devices, virtual keyboards (aka software keyboards or onscreen keyboards) are displayed on top of the application. When such a keyboard is shown, this property denotes the position of the top left boundary of the rectangle covered by it in window coordinates.

`Point`

This structure represents a point with x and y coordinate

- **`x`** ( *length* ):
- **`y`** ( *length* ):

#### virtual-keyboard-size

[struct Size](https://docs.slint.dev/latest/docs/slint/reference/global-structs-enums/index.html#size) `(out)` default: `a struct with all default values`

On mobile devices, virtual keyboards (aka software keyboards or onscreen keyboards) are displayed on top of the application. When such a keyboard is shown, this property denotes the width and height of the rectangle covered by it in window coordinates.

`Size`

This structure represents a size with width and height

- **`width`** ( *length* ):
- **`height`** ( *length* ):

### Functions

#### hide()

Hide this window.

# Tutorial

## Conclusion

Source: `tutorial/conclusion/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/conclusion/

This tutorial showed you how to combine built-in Slint elements with C++, Rust, or NodeJS code to build a game. There is much more to Slint, such as layouts, widgets, or styling.

We recommend the following links to continue:

- [Examples↗](https://github.com/slint-ui/slint/tree/master/examples) : The Slint repository has several demos and examples. These are a great starting point to learn how to use many Slint features.
  - [Todo Example↗](https://github.com/slint-ui/slint/tree/master/examples/todo) : This is one of the examples that implements a classic use-case.
  - [Memory Puzzle↗](https://github.com/slint-ui/slint/tree/master/examples/memory) : This is a slightly more polished version of the code in this example and you can [play the wasm version](https://slint.dev/demos/memory/) in your browser.

## Creating the tiles

Source: `tutorial/creating_the_tiles/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/creating_the_tiles/

This step places the game tiles randomly.

**C++**

Change the `main` function and includes in `src/main.cpp` to the following:

The code takes the list of tiles, duplicates it, and shuffles it, accessing the `memory_tiles` property through the C++ code.

For each top-level property, Slint generates a getter and a setter function. In this case `get_memory_tiles` and `set_memory_tiles`. Since `memory_tiles` is a Slint array, it’s represented as a [`std::shared_ptr<slint::Model>`](https://docs.slint.dev/latest/docs/cpp/api/classslint_1_1model).

You can’t change the model generated by Slint, but you can extract the tiles from it and put them in a [`slint::VectorModel`](https://docs.slint.dev/latest/docs/cpp/api/classslint_1_1vectormodel) which inherits from `Model`. `VectorModel` lets you make changes and you can use it to replace the static generated model.

**NodeJS**

Change `main.js` to the following:

```js
import * as slint from "slint-ui";
const ui = slint.loadFile(new URL("./ui/app-window.slint", import.meta.url));
const mainWindow = new ui.MainWindow();

const initial_tiles = mainWindow.memory_tiles;
const tiles = initial_tiles.concat(
    initial_tiles.map((tile) => Object.assign({}, tile)),
);

for (let i = tiles.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * i);
    [tiles[i], tiles[j]] = [tiles[j], tiles[i]];
}

const model = new slint.ArrayModel(tiles);
mainWindow.memory_tiles = model;

await mainWindow.run();
```

The code takes the list of tiles, duplicates it, and shuffles it, accessing the `memory_tiles` property through the JavaScript code.

As `memory_tiles` is an array, it’s represented as a JavaScript `Array`. You can’t change the model generated by Slint, but you can extract the tiles from it and put them in a [`slint.ArrayModel.html`](https://docs.slint.dev/latest/docs/node/classes/ArrayModel) which implements the [`Model`](https://docs.slint.dev/latest/docs/node/classes/Model.html) interface. `ArrayModel` allows you to make changes and you can use it to replace the static generated model.

**Rust**

The code uses the `rand` dependency for the randomization. Add it to the `Cargo.toml` file using the `cargo` command.

Change the main function to the following:

```rust
fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    main_window.run().unwrap();
}
```

The code takes the list of tiles, duplicates it, and shuffles it, accessing the `memory_tiles` property through the Rust code.

For each top-level property, Slint generates a getter and a setter function. In this case `get_memory_tiles` and `set_memory_tiles`. Since `memory_tiles` is a Slint array represented as a [`Rc<dyn slint::Model>`](https://docs.slint.dev/latest/docs/rust/slint/trait.Model).

You can’t change the model generated by Slint, but you can extract the tiles from it and put them in a [`VecModel`](https://docs.slint.dev/latest/docs/rust/slint/struct.VecModel) which implements the `Model` trait. `VecModel` lets you make changes and you can use it to replace the static generated model.

**Python**

Change `main.py` to the following:

```python
import slint
import sys
import os
import random
import itertools
import copy
import datetime

class MainWindow(slint.loader.ui.app_window.MainWindow):
    def __init__(self):
        super().__init__()
        initial_tiles = self.memory_tiles
        tiles = slint.ListModel(
            itertools.chain(
                map(copy.copy, initial_tiles), map(copy.copy, initial_tiles)
            )
        )
        random.shuffle(tiles)
        self.memory_tiles = tiles

main_window = MainWindow()
main_window.show()
main_window.run()
```

The code takes the list of tiles, duplicates it, and shuffles it, accessing the `memory_tiles` property through the Python code.

As `memory_tiles` is an array, it’s represented as a Slint `ListModel`. You can’t change the model generated by Slint, but you can extract the tiles from it and put them in a [`slint.ListModel`](https://docs.slint.dev/latest/docs/python/slint#ListModel) which is a subclass of [`Model`](https://docs.slint.dev/latest/docs/python/slint#Model) interface. `ListModel` allows you to make changes and you can use it to replace the static generated model.

Running this code opens a window that now shows a 4 by 4 grid of rectangles, which show or hide the icons when a player clicks on them.

There’s one last aspect missing now, the rules for the game.

[Video](https://slint.dev/blog/memory-game-tutorial/creating-the-tiles-from-rust.mp4)

## From One To Multiple Tiles

Source: `tutorial/from_one_to_multiple_tiles/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/from_one_to_multiple_tiles/

After modeling a single tile, this step creates a grid of them. For the grid to be a game board, you need two features:

1. **A data model**: An array created as a model in code, where each element describes the tile data structure, such as:
  - URL of the image
  - Whether the image is visible
  - If the player has solved this tile.
2. A way of creating multiple instances of the tiles.

With Slint you declare an array of structures based on a model using square brackets. Use a for loop to create multiple instances of the same element.

The for loop is declarative and automatically updates when the model changes. The loop instantiates all the MemoryTile elements and places them on a grid based on their index with spacing between the tiles.

First, add the tile data structure definition at the top of the `ui/app-window.slint` file:

Next, replace the _export component MainWindow inherits Window { … } section at the bottom of the `ui/app-window.slint` file with the following:

```slint
export component MainWindow inherits Window {
    width: 326px;
    height: 326px;

    in property <[TileData]> memory_tiles: [
        { image: @image-url("icons/at.png") },
        { image: @image-url("icons/balance-scale.png") },
        { image: @image-url("icons/bicycle.png") },
        { image: @image-url("icons/bus.png") },
        { image: @image-url("icons/cloud.png") },
        { image: @image-url("icons/cogs.png") },
        { image: @image-url("icons/motorcycle.png") },
        { image: @image-url("icons/video.png") },
    ];
    for tile[i] in memory_tiles : MemoryTile {
        x: mod(i, 4) * 74px;
        y: floor(i / 4) * 74px;
        width: 64px;
        height: 64px;
        icon: tile.image;
        open_curtain: tile.image_visible || tile.solved;
        // propagate the solved status from the model to the tile
        solved: tile.solved;
        clicked => {
            tile.image_visible = !tile.image_visible;
        }
    }
}
```

The `for tile[i] in memory_tiles:` syntax declares a variable `tile` which contains the data of one element from the `memory_tiles` array, and a variable `i` which is the index of the tile. The code uses the `i` index to calculate the position of a tile, based on its row and column, using modulo and integer division to create a 4 by 4 grid.

Running the code opens a window that shows 8 tiles, which a player can open individually.

[Video](https://slint.dev/blog/memory-game-tutorial/from-one-to-multiple-tiles.mp4)

## Game Logic

Source: `tutorial/game_logic/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/game_logic/

This step implements the rules of the game in your coding language of choice.

Slint’s general philosophy is that you implement the user interface in Slint and the business logic in your favorite programming language.

The game rules enforce that at most two tiles have their curtain open. If the tiles match, then the game considers them solved and they remain open. Otherwise, the game waits briefly so the player can memorize the location of the icons, and then closes the curtains again.

**C++**

Add the following code inside the MainWindow component to signal to the C++ code when the user clicks on a tile.

This change adds a way for the MainWindow to call to the C++ code that it should check if a player has solved a pair of tiles. The Rust code needs an additional property to toggle to disable further tile interaction, to prevent the player from opening more tiles than allowed. No cheating allowed!

The last change to the code is to act when the MemoryTile signals that a player clicked it.

Add the following handler in the MainWindow `for` loop `clicked` handler:

```slint
        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                // old: tile.image_visible = !tile.image_visible;
                // new:
                if (!root.disable_tiles) {
                    tile.image_visible = true;
                    root.check_if_pair_solved();
                }
            }
        }
```

On the C++ side, you can now add a handler to the `check_if_pair_solved` callback, that checks if a player opened two tiles. If they match, the code sets the `solved` property to true in the model. If they don’t match, start a timer that closes the tiles after one second. While the timer is running, disable every tile so a player can’t click anything during this time.

Insert this code before the `main_window->run()` call:

```cpp
    main_window->on_check_if_pair_solved(
            [main_window_weak = slint::ComponentWeakHandle(main_window)] {
                auto main_window = *main_window_weak.lock();
                auto tiles_model = main_window->get_memory_tiles();
                int first_visible_index = -1;
                TileData first_visible_tile;
                for (int i = 0; i < tiles_model->row_count(); ++i) {
                    auto tile = *tiles_model->row_data(i);
                    if (!tile.image_visible || tile.solved)
                        continue;
                    if (first_visible_index == -1) {
                        first_visible_index = i;
                        first_visible_tile = tile;
                        continue;
                    }
                    bool is_pair_solved = tile == first_visible_tile;
                    if (is_pair_solved) {
                        first_visible_tile.solved = true;
                        tiles_model->set_row_data(first_visible_index,
                                                  first_visible_tile);
                        tile.solved = true;
                        tiles_model->set_row_data(i, tile);
                        return;
                    }
                    main_window->set_disable_tiles(true);

                    slint::Timer::single_shot(std::chrono::seconds(1),
                        [=]() mutable {
                            main_window->set_disable_tiles(false);
                            first_visible_tile.image_visible = false;
                            tiles_model->set_row_data(first_visible_index,
                                                      first_visible_tile);
                            tile.image_visible = false;
                            tiles_model->set_row_data(i, tile);
                        });
                }
            });
```

The code uses a [`ComponentWeakHandle`](https://docs.slint.dev/latest/docs/cpp/api/classslint_1_1ComponentWeakHandle) pointer of the `main_window`. This is important because capturing a copy of the `main_window` itself within the callback handler would result in circular ownership. The `MainWindow` owns the callback handler, which itself owns a reference to the `MainWindow`, which must be weak instead of strong to avoid a memory leak.

**NodeJS**

Change the contents of `memory.slint` to signal to the JavaScript code when the user clicks on a tile.

```slint
    export component MainWindow inherits Window {
        width: 326px;
        height: 326px;

        callback check_if_pair_solved(); // Added
        in property <bool> disable_tiles; // Added

        in-out property <[TileData]> memory_tiles: [
           { image: @image-url("icons/at.png") },
```

This change adds a way for the MainWindow to call to the JavaScript code that it should check if a player has solved a pair of tiles. The Rust code needs an additional property to toggle to disable further tile interaction, to prevent the player from opening more tiles than allowed. No cheating allowed!

The last change to the code is to act when the MemoryTile signals that a player clicked it.

Add the following handler in the MainWindow `for` loop `clicked` handler:

```slint
        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                // old: tile.image_visible = !tile.image_visible;
                // new:
                if (!root.disable_tiles) {
                    tile.image_visible = true;
                    root.check_if_pair_solved();
                }
            }
        }
```

On the JavaScript side, now add a handler to the `check_if_pair_solved` callback, that checks if a player opened two tiles. If they match, the code sets the `solved` property to true in the model. If they don’t match, start a timer that closes the tiles after one second. While the timer is running, disable every tile so a player can’t click anything during this time.

Insert this code before the `mainWindow.run()` call:

```js
mainWindow.check_if_pair_solved = function () {
    const flipped_tiles = [];
    tiles.forEach((tile, index) => {
        if (tile.image_visible && !tile.solved) {
            flipped_tiles.push({
                index,
                tile,
            });
        }
    });

    if (flipped_tiles.length === 2) {
        const { tile: tile1, index: tile1_index } = flipped_tiles[0];

        const { tile: tile2, index: tile2_index } = flipped_tiles[1];

        const is_pair_solved = tile1.image.path === tile2.image.path;
        if (is_pair_solved) {
            tile1.solved = true;
            model.setRowData(tile1_index, tile1);
            tile2.solved = true;
            model.setRowData(tile2_index, tile2);
        } else {
            mainWindow.disable_tiles = true;
            setTimeout(() => {
                mainWindow.disable_tiles = false;
                tile1.image_visible = false;
                model.setRowData(tile1_index, tile1);
                tile2.image_visible = false;
                model.setRowData(tile2_index, tile2);
            }, 1000);
        }
    }
};
```

**Rust**

Add the following code inside the MainWindow component to signal to the Rust code when the user clicks on a tile.

```slint
    export component MainWindow inherits Window {
        width: 326px;
        height: 326px;

        callback check_if_pair_solved(); // Added
        in property <bool> disable_tiles; // Added

        in-out property <[TileData]> memory_tiles: [
           { image: @image-url("icons/at.png") },
```

This change adds a way for the MainWindow to call to the Rust code that it should check if a player has solved a pair of tiles. The Rust code needs an additional property to toggle to disable further tile interaction, to prevent the player from opening more tiles than allowed. No cheating allowed!

The last change to the code is to act when the MemoryTile signals that a player clicked it.

Add the following handler in the MainWindow `for` loop `clicked` handler:

```slint
        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                // old: tile.image_visible = !tile.image_visible;
                // new:
                if (!root.disable_tiles) {
                    tile.image_visible = true;
                    root.check_if_pair_solved();
                }
            }
        }
```

On the Rust side, you can now add a handler to the `check_if_pair_solved` callback, that checks if a player opened two tiles. If they match, the code sets the `solved` property to true in the model. If they don’t match, start a timer that closes the tiles after one second. While the timer is running, disable every tile so a player can’t click anything during this time.

Add this code before the `main_window.run().unwrap();` call:

```rust
    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles =
            tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });
```

The code uses a [Weak](https://docs.slint.dev/latest/docs/rust/slint/struct.Weak) pointer of the `main_window`. This is important because capturing a copy of the `main_window` itself within the callback handler would result in circular ownership. The `MainWindow` owns the callback handler, which itself owns a reference to the `MainWindow`, which must be weak instead of strong to avoid a memory leak.

**Python**

Change the contents of `memory.slint` to signal to the Python code when the user clicks on a tile.

```slint
    export component MainWindow inherits Window {
        width: 326px;
        height: 326px;

        callback check_if_pair_solved(); // Added
        in property <bool> disable_tiles; // Added

        in-out property <[TileData]> memory_tiles: [
           { image: @image-url("icons/at.png") },
```

This change adds a way for the MainWindow to call to the Python code that it should check if a player has solved a pair of tiles. The Rust code needs an additional property to toggle to disable further tile interaction, to prevent the player from opening more tiles than allowed. No cheating allowed!

The last change to the code is to act when the MemoryTile signals that a player clicked it.

Add the following handler in the MainWindow `for` loop `clicked` handler:

```slint
        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                // old: tile.image_visible = !tile.image_visible;
                // new:
                if (!root.disable_tiles) {
                    tile.image_visible = true;
                    root.check_if_pair_solved();
                }
            }
        }
```

On the Python side, now add a handler to the `check_if_pair_solved` callback, that checks if a player opened two tiles. If they match, the code sets the `solved` property to true in the model. If they don’t match, start a timer that closes the tiles after one second. While the timer is running, disable every tile so a player can’t click anything during this time.

Insert this function in the `MainWindow` class, annotated with `@slint.callback` to associated it with `check_if_pair_solved`:

```python
class MainWindow(slint.loader.ui.app_window.MainWindow):
    def __init__(self):
        super().__init__()
        initial_tiles = self.memory_tiles
        tiles = slint.ListModel(
            itertools.chain(
                map(copy.copy, initial_tiles), map(copy.copy, initial_tiles)
            )
        )
        random.shuffle(tiles)
        self.memory_tiles = tiles

    @slint.callback
    def check_if_pair_solved(self):
        flipped_tiles = [
            (index, copy.copy(tile))
            for index, tile in enumerate(self.memory_tiles)
            if tile.image_visible and not tile.solved
        ]
        if len(flipped_tiles) == 2:
            tile1_index, tile1 = flipped_tiles[0]
            tile2_index, tile2 = flipped_tiles[1]
            is_pair_solved = tile1.image.path == tile2.image.path
            if is_pair_solved:
                tile1.solved = True
                self.memory_tiles[tile1_index] = tile1
                tile2.solved = True
                self.memory_tiles[tile2_index] = tile2
            else:
                self.disable_tiles = True

                def reenable_tiles():
                    self.disable_tiles = False
                    tile1.image_visible = False
                    self.memory_tiles[tile1_index] = tile1
                    tile2.image_visible = False
                    self.memory_tiles[tile2_index] = tile2

                slint.Timer.single_shot(datetime.timedelta(seconds=1), reenable_tiles)
```

These were the last changes and running the code opens a window that allows a player to play the game by the rules.

## Getting started

Source: `tutorial/getting_started/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/getting_started/

This tutorial shows you how to use the languages that Slint supports as the host programming language.

We recommend using [our editor integrations for Slint↗](https://github.com/slint-ui/slint/tree/master/editors) for following this tutorial.

Slint has application templates you can use to create a project with dependencies already set up that follows recommended best practices.

### Prerequisites

**C++**

Before using the template, you need a C++ compiler that supports C++ 20 and to install [CMake↗](https://cmake.org/download/) 3.21 or newer.

1. Download and extract the [ZIP archive↗](https://github.com/slint-ui/slint-cpp-template/archive/refs/heads/main.zip) of the [C++ Template↗](https://github.com/slint-ui/slint-cpp-template/) .
2. Rename the extracted directory and change into it:

#### Configure the project

The `CMakeLists.txt` uses the line `add_executable(my_application src/main.cpp)` to set `src/main.cpp` as the main C++ code file.

Replace the content of `src/main.cpp` with the following:

Also in `CMakeLists.txt` the line `slint_target_sources(my_application ui/app-window.slint)` is a Slint function used to add the `app-window.slint` file to the target.

Replace the contents of `ui/app-window.slint` with the following:

```slint
export component MainWindow inherits Window {
    Text {
        text: "hello world";
        color: green;
    }
}
```

Configure with CMake:

```sh
cmake -B build
```

> **Tip**
> When configuring with CMake, the FetchContent module fetches the source code of Slint via git. This may take some time when building for the first time, as the process needs to build the Slint runtime and compiler.

Build with CMake:

```sh
cmake --build build
```

#### Run the application

Run the application binary on Linux or macOS:

```sh
./build/my_application
```

Or on Windows:

```sh
build\my_application.exe
```

This opens a window with a green “Hello World” greeting.

If you are stepping through this tutorial on a Windows machine, you can run the application at each step with:

```sh
my_application
```

**NodeJS**

1. Download and extract the [ZIP archive↗](https://github.com/slint-ui/slint-nodejs-template/archive/refs/heads/main.zip) of the [Node.js Template↗](https://github.com/slint-ui/slint-nodejs-template) .
2. Rename the extracted directory and change into it:

```sh
mv slint-nodejs-template-main memory
cd memory
```

1. Install dependencies with npm:

```sh
npm install
```

#### Configure the project

The `package.json` file references `src/main.js` as the entry point for the application and `src/main.js` references `memory.slint` as the UI file.

Replace the contents of `src/main.js` with the following:

```js
import * as slint from "slint-ui";

const ui = slint.loadFile(new URL("./ui/app-window.slint", import.meta.url));
const mainWindow = new ui.MainWindow();
await mainWindow.run();
```

The `slint.loadFile` method resolves files from the process’s current working directory, so from the `package.json` file’s location.

Replace the contents of `ui/app-window.slint` with the following:

```slint
export component MainWindow inherits Window {
    Text {
        text: "hello world";
        color: green;
    }
}
```

#### Run the application

Run the example with `npm start` and a window appears with the green “Hello World” greeting.

**Rust**

We recommend using [rust-analyzer↗](https://rust-analyzer.github.io/) and [our editor integrations for Slint↗](https://github.com/slint-ui/slint/tree/master/editors) for following this tutorial.

1. Download and extract the [ZIP archive↗](https://github.com/slint-ui/slint-rust-template/archive/refs/heads/main.zip) of the [Rust Template↗](https://github.com/slint-ui/slint-rust-template) .
2. Rename the extracted directory and change into it:

```sh
mv slint-rust-template-main memory
cd memory
```

#### Configure the project

Replace the contents of `src/main.rs` with the following:

```rust
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}
```

Replace the contents of `ui/app-window.slint` with the following:

```slint
export component MainWindow inherits Window {
    Text {
        text: "hello world";
        color: green;
    }
}
```

#### Run the application

Run the example with `cargo run` and a window appears with the green “Hello World” greeting.

**Python**

1. Download and extract the [ZIP archive↗](https://github.com/slint-ui/slint-python-template/archive/refs/heads/main.zip) of the [Python Template↗](https://github.com/slint-ui/slint-python-template) .
2. Rename the extracted directory and change into it:

```sh
mv slint-python-template-main memory
cd memory
```

1. Install dependencies with uv:

```sh
uv sync
```

#### Configure the project

The entry point for the application is `main.py`, the UI file is `app-window.slint`.

Replace the contents of `main.py` with the following:

```python
import slint

class MainWindow(slint.loader.ui.app_window.MainWindow):
    pass

main_window = MainWindow()
main_window.show()
main_window.run()
```

The `slint.loadFile` method resolves files from the process’s current working directory, so from the `package.json` file’s location.

Replace the contents of `ui/app-window.slint` with the following:

```slint
export component MainWindow inherits Window {
    Text {
        text: "hello world";
        color: green;
    }
}
```

#### Run the application

Run the example with `uv run main.py` and a window appears with the green “Hello World” greeting.

![Screenshot of initial tutorial app showing Hello World](https://docs.slint.dev/slint.dev/blog/memory-game-tutorial/getting-started.png)

## Ideas For The Reader

Source: `tutorial/ideas_for_the_reader/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/ideas_for_the_reader/

The game is visually bare. Here are some ideas on how you could make further changes to enhance it:

- The tiles could have rounded corners, to look less sharp. Use the [border-radius](https://docs.slint.dev/latest/docs/slint/reference/elements/rectangle/index.html#border-radius-properties) property of [Rectangle](https://docs.slint.dev/latest/docs/slint/reference/elements/rectangle/index.html) to achieve that.
- In real-world memory games, the back of the tiles often have some common graphic. You could add an image with the help of another [Image](https://docs.slint.dev/latest/docs/slint/reference/elements/image/index.html) element. Note that you may have to use *Rectangle*’s *clip property* element around it to ensure that the image is clipped away when the curtain effect opens.

## Memory Tile

Source: `tutorial/memory_tile/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/memory_tile/

With the skeleton code in place, this step looks at the first element of the game, the memory tile. It’s the visual building block that consists of an underlying filled rectangle background, the icon image. Later steps add a covering rectangle that acts as a curtain.

You declare the background rectangle as 64 logical pixels wide and tall filled with a soothing tone of blue.

Lengths in Slint have a unit, here, the `px` suffix. This makes the code easier to read and the compiler can detect when you accidentally mix values with different units attached to them.

Copy the following code into `ui/app-window.slint` file, replacing the current content:

This exports the MainWindow component so that the game logic code can access it later.

Inside the Rectangle place an Image element that loads an icon with the @image-url() macro. The path is relative to the location of `ui/app-window.slint`.

You need to install this icon and others you use later first. You can download a pre-prepared [Zip archive↗](https://slint.dev/blog/memory-game-tutorial/icons.zip) to the `ui` folder,

If you are on Linux or macOS, download and extract it with the following commands:

**Windows**

If you are on Windows, use the following commands:

This unpacks an `icons` directory containing several icons.

**macOS**

```sh
cd ui
curl -O https://slint.dev/blog/memory-game-tutorial/icons.zip
unzip icons.zip
cd ..
```

This unpacks an `icons` directory containing several icons.

**Linux**

```sh
cd ui
curl -O https://slint.dev/blog/memory-game-tutorial/icons.zip
unzip icons.zip
cd ..
```

This unpacks an `icons` directory containing several icons.

**C++**

Compiling the program with `cmake --build build` and running with the `./build/my_application` opens a window that shows the icon of a bus on a blue background.

**NodeJS**

Running the program with `npm start` opens a window that shows the icon of a bus on a blue background.

**Rust**

Running the program with `cargo run` opens a window that shows the icon of a bus on a blue background.

**Python**

Running the program with `uv run main.py` opens a window that shows the icon of a bus on a blue background.

![Screenshot of the first tile](https://docs.slint.dev/slint.dev/blog/memory-game-tutorial/memory-tile.png)

## Polishing the Tile

Source: `tutorial/polishing_the_tile/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/polishing_the_tile/

In this step, you add a curtain-like cover that opens when clicked. Slint files have an implicit z order for drawing items. Each subsequent item is drawn above the previous one. So a `Rectangle` on line 10 would be underneath another declared later in the file on line 50. To give the impression of curtains that cover the image, declare two rectangles after the Image, so that Slint draws them over the Image.

The TouchArea element declares a transparent rectangular region that allows reacting to user input such as a mouse click or tap. The element forwards a callback to the *MainWindow* indicating that a user clicked the tile.

The *MainWindow* reacts by flipping a custom *open_curtain* property. Property bindings for the animated width and x properties also use the custom *open_curtain* property.

The following table shows more detail on the two states:

| *open_curtain* value: | false | true |
| --- | --- | --- |
| Left curtain rectangle | Fill the left half by setting the width *width* to half the parent’s width | Width of zero makes the rectangle invisible |
| Right curtain rectangle | Fill the right half by setting *x* and *width* to half of the parent’s width | *width* of zero makes the rectangle invisible. *x* moves to the right, sliding the curtain open when animated |

To make the tile extensible, replace the hard-coded icon name with an *icon* property that can be set when instantiating the element.

For the final polish, add a *solved* property used to animate the color to a shade of green when a player finds a pair.

Replace the code inside the `ui/app-window.slint` file with the following:

The code uses `root` and `self`. `root` refers to the outermost element in the component, the MemoryTile in this case. `self` refers to the current element.

The code exports the MainWindow component. This is necessary so that you can later access it from application business logic.

Running the code opens a window with a rectangle that opens up to show the bus icon when clicked. Subsequent clicks close and open the curtain again.

[Video](https://slint.dev/blog/memory-game-tutorial/polishing-the-tile.mp4)

## Intro

Source: `tutorial/quickstart/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/quickstart/

This tutorial introduces you to the Slint framework in a playful way by implementing a memory game. It combines the Slint language for the graphics with the game rules implemented in C++, Rust, NodeJS, or Python.

The game consists of a grid of 16 rectangular tiles. Clicking on a tile uncovers an icon underneath. There are 8 different icons in total, so each tile has a sibling somewhere in the grid with the same icon. The objective is to locate all icon pairs. The player can uncover two tiles at the same time. If they aren’t the same, the game obscures the icons again. If the player uncovers two tiles with the same icon, then they remain visible - they’re solved.

This is how the game looks in action:

[Video](https://slint.dev/blog/memory-game-tutorial/memory_clip.mp4)

## Running In A Browser

Source: `tutorial/running_in_a_browser/`
Official URL: https://docs.slint.dev/latest/docs/slint/tutorial/running_in_a_browser/

> **Caution**
> Only Rust supports using Slint with WebAssembly.

If you’re using Rust, the tutorial so far used `cargo run` to build and run the code as a native application. Native applications are the primary target of the Slint framework, but it also supports WebAssembly for demonstration purposes. This section uses the standard rust tool `wasm-bindgen` and `wasm-pack` to run the game in the browser. Read the [wasm-bindgen documentation↗](https://wasm-bindgen.github.io/wasm-bindgen/examples/without-a-bundler.html) for more about using wasm and rust.

Install `wasm-pack` using cargo:

Edit the `Cargo.toml` file to add the dependencies.

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = { version = "0.2" }
getrandom = { version = "0.3.4", features = ["wasm_js"] }
```

`'cfg(target_arch = "wasm32")'` ensures that these dependencies are only active when compiling for the wasm32 architecture. Note that the `rand` dependency is now duplicated, to enable the `"wasm-bindgen"` feature.

While you are editing the `Cargo.toml`, make one last change. To turn the binary into a library by adding the following:

```toml
[lib]
path = "src/main.rs"
crate-type = ["cdylib"]
```

This is because wasm-pack requires Rust to generate a `"cdylib"`.

You also need to change `main.rs` by adding the `wasm_bindgen(start)` attribute to the `main` function and export it with the `pub` keyword:

```rust
#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    //...
}
```

Compile the program with `wasm-pack build --release --target web`. This creates a `pkg` directory containing several files, including a `.js` file named after the program name that you need to import into an HTML file.

Create a minimal `index.html` in the top level of the project that declares a `<canvas>` element for rendering and loads the generated wasm file. The Slint runtime expects the `<canvas>` element to have the id `id = "canvas"`. (Replace `memory.js` with the correct file name).

```html
<html>
    <body>
        <!-- canvas required by the Slint runtime -->
        <canvas id="canvas"></canvas>
        <script type="module">
            // import the generated file.
            import init from "./pkg/memory.js";
            init();
        </script>
    </body>

<!-- Mirrored from docs.slint.dev/latest/docs/slint/tutorial/running_in_a_browser/ by HTTrack Website Copier/3.x [XR&CO'2014], Tue, 12 May 2026 23:57:46 GMT -->
</html>
```

Unfortunately, loading ES modules isn’t allowed for files on the file system when accessed from a `file://` URL, so you can’t load the `index.html`. Instead, you need to serve it through a web server. For example, using Python, by running:

```sh
python3 -m http.server
```

Now you can access the game at [http://localhost:8000↗](http://localhost:8000/).

# Language Integrations

## Language Integrations

Source: `language-integrations/`
Official URL: https://docs.slint.dev/latest/docs/slint/language-integrations/

Slint provides first class integrations to various programming languages allowing you to implement the business logic of your Slint application in your preferred language.

[C++](https://docs.slint.dev/latest/docs/cpp/)

Browse C++ API docs

[Rust](https://docs.slint.dev/latest/docs/rust/slint/)

Browse Rust API docs

[TypeScript (beta)](https://docs.slint.dev/latest/docs/node/)

Browse TypeScript / JavaScript API docs.

[Python (beta)](https://docs.slint.dev/latest/docs/python/)

Browse Python API docs.
