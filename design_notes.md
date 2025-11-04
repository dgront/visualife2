# Notes on VisuaLife design

## How the crate is divided into modules?
VisuaLife comprises two core modules: `basic_elements`, which provides the low-level primitives for constructing SVG content, and `styling`, which defines how those elements look (colors, strokes, fills, etc.).
All other modules are independent, high-level graphing libraries, each dedicated to generating a specific plot or diagram type:
  - [`mindmap`](crate::mindmap) draws a mindmap
  - [`heatmap`](crate::heatmap) plots 2D array of data as a heatmap

## Why there is no CSS suppport?
Introducing this feature would require a global ``StyleManager``, most likely implementing the singleton pattern.
However, such a design could lead to undesirable interactions with Rust’s borrow checker.
This behavior is achieved by organizing elements into groups, which automatically propagate their style settings to all contained elements.

## Why it's not possible to change an element style after it has been created?
SVG elements are nested within groups, forming a tree-like data structure. Direct access to a specific element would therefore require an additional indexing mechanism to traverse and locate elements efficiently. Moreover, this functionality is unnecessary in practice, as users can define and apply the desired style prior to creating the element.
