Gain
====

This example applies gain to a WAV file and writes the result back out as a
new WAV file. It is intentionally small: the point is to show the basic DSP
step, the WAV parsing, and the same command-line workflow in C++ and Rust.

What it does
------------

The program reads 16-bit PCM sample data, multiplies every sample by a gain
value, applies a soft clipper, and writes the processed samples to a new file.

Gain can be supplied in either of two forms:

* ``--db <value>`` converts decibels to linear gain using ``10^(dB / 20)``.
* ``--linear <value>`` uses the value directly as the multiplier.

The soft clipper uses ``tanh`` so boosted samples bend toward the ceiling
instead of wrapping or hard-clipping abruptly.

Supported input
---------------

This example supports plain 16-bit PCM WAV files only.

It expects:

* ``RIFF`` / ``WAVE`` container tags
* a ``fmt`` chunk with PCM format code ``1``
* ``16`` bits per sample
* a ``data`` chunk containing little-endian ``i16`` samples

Compressed WAV files, floating-point WAV files, and 24-bit or 32-bit PCM files
are rejected.

Rust version
------------

The Rust implementation lives in ``rust/src/main.rs`` and uses only the Rust
standard library.

Run it with Cargo:

.. code-block:: powershell

   cd gain\rust
   cargo run -- input.wav output.wav --db 6
   cargo run -- input.wav output.wav --linear 0.5

The program prints the sample rate, channel count, sample count, and gain being
applied before writing the output file.

C++ version
-----------

The C++ implementation lives in ``cpp/main.cpp`` and uses only the C++ standard
library.

One simple way to build it is:

.. code-block:: powershell

   cd gain\cpp
   g++ -std=c++17 -O2 main.cpp -o gain.exe
   .\gain.exe input.wav output.wav --db 6
   .\gain.exe input.wav output.wav --linear 0.5

Use the equivalent compiler command for your environment if you are using MSVC,
Clang, or a different build setup.

Notes
-----

Both implementations keep the file handling deliberately direct. They scan WAV
chunks until they find ``fmt`` and ``data``, skip chunks they do not use, process
samples in memory, and then write a minimal WAV file with a standard ``fmt``
chunk and one ``data`` chunk.

The Rust version batches sample reads and writes instead of doing one system
call per sample, which keeps larger WAV files from feeling unnecessarily slow.