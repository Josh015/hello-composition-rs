# Rust HelloComposition sample

This sample is adapted from the code created in the [Using the Visual Layer with Win32](https://docs.microsoft.com/windows/uwp/composition/using-the-visual-layer-with-win32) and [Using the Visual Layer with Windows Forms](https://docs.microsoft.com/windows/uwp/composition/using-the-visual-layer-with-windows-forms) tutorials and the [minesweeper-rs](https://github.com/robmikh/minesweeper-rs) project. It's a simple user interface (UI) that demonstrates how to add Universal Windows Platform (UWP) [Visual Layer](https://docs.microsoft.com/windows/uwp/composition/visual-layer) content to a Rust app.

The Visual Layer APIs provide a high performance, retained-mode API for graphics, effects, and animations. It's the recommended  replacement for DirectComposition in apps that run on Windows 10.

This sample demonstrates how to set up the interop code needed to use these APIs in a Rust app.

![App user interface](app-ui-rust.png)
