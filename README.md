# Canvas

This is a general infinite canvas that can be used as a starting point for making tools and itself started as a tool draft.
It is not a library, so any changes need to be done directly in the code.

## Features & use

- Canvas movement: Drag with right mouse button
- Canvas zoom: Scroll with the mouse wheel
- Insert: Choose block direction by pressing the coreresponding arrow and placement by left click
  (pressing an arrow without placing the prewious block will place it at the current cusor position)
- Select: Click on a block or define selection corners by draging (starting on the canvas)
- Block movement: Drag with left mouse button (starting on unselected block moves the block, starting on selected block moves all selected blocks)
- Duplicate selection: Press Ctrl + D
- Delete a selection: Press the delete button

## Building

`cargo build --release`
