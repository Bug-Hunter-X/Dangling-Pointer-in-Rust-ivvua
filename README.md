# Dangling Pointer Bug in Rust

This repository demonstrates a common bug in Rust involving dangling pointers.  The code attempts to modify a vector through a raw pointer after the vector's length has changed, leading to undefined behavior. The solution showcases how to avoid this issue using safer methods.