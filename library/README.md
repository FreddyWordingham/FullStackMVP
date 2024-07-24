<p align="center" style="font-size: 2.5em">
    MyLibrary
</p>
<p align="center">
  <img src="../assets/images/icon.png" alt="Repository icon" width="200" />
</p>
<p align="center" style="font-size: 1.5em">
</p>

This is a Rust library.

# Setup

As we'll be generating Python bindings for this library, we need access to Python.
We can do this by setting the `PYO3_PYTHON` environment variable to the path of the Python executable:

```bash
export PYO3_PYTHON=$(which python)
```
