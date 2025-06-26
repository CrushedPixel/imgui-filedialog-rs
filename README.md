# Rust bindings to [ImGuiFileDialog]

Bindings to the [ImGuiFileDialog] module, a file browser interface for [Dear ImGui][imgui] interfaces, wrapped for use
with [imgui-rs][imgui-rs].

Updated to imgui 0.12
and [ImGuiFileDialog #cd5d354](https://github.com/aiekick/ImGuiFileDialog/tree/cd5d354cbfdda389422494fa47fced59ad4d25a7)
by @CrushedPixel.

[ImGuiFileDialog]: https://github.com/aiekick/ImGuiFileDialog

[imgui]: https://github.com/ocornut/imgui/

[imgui-rs]: https://github.com/imgui-rs/imgui-rs/

## Status

Functional bindings to most functionality.
The Example doesn't currently build.

## Features

`bevy_reflect`: Adds `#[derive(Reflect)]` to structs.
