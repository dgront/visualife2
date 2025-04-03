# VisuaLife v.2


## What VisuaLife?
VisuaLife is a siftware library for visualization with strong bias towards life sciences.
BioShell project, started in 2006, has been a command line toolkit for  structural bioinformatics.
Over time, it has also provided Coarse Grained simulations of proteins


## Project structure
The core of the project is comprised by the two low-level modules:

 - **basic_shapes**: provides elements as defined by the SVG specification
 - **styling**: provies graphical representation of those basic elements as well as utilities for color management, palettes etc.

High level functionaly provides different visualisation modules, such as:
 - **mindmap**: draws TiKZ-style mindmaps
 - **heatmap**: visualization of data matrices
 - **bioshell-clustering**: clustering methods, such as hierarchical clustering, K-means and OPTICS
 - **bioshell-io**: I/O utilities

## Building
You need to install `rust` toolchain to compile the package. You can:
 
 - **compile the whole project**, i.e. all its libraries (called crates) by executing the following command in the root folder of the project: 
```bash
cargo build --release --all
```
This will compile also all the executables across all the crates.

- **compile all examples**:
```bash
cargo build --all --examples --release
```

All the above commands compile the *release* (i.e. optimized) version. To obtain the *debug* build, just remove the ``--release`` flag.

- **compile python binding**: for this you need to install Maturin toolchan. Create a Python virtual environment
and install packages listed in the `requirements.txt`:

```bash
cd bindings/python
python3 -m venv .venv
source .venv/bin/activate
pip3 install -r requirements.txt
maturin develop
```
The ``python3 -m venv .venv`` and ``pip3 install -r requirements.txt`` commands should be run only once.

 - **build documentation**:
```
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open
```

 - **run all tests** by executing the following command in the root folder
```
cargo test --release
```
