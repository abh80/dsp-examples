DSP Examples
============

This repository contains small audio and DSP examples for learners. The goal is
to keep each example focused enough to read in one sitting, run locally, and
modify without needing a large framework first.

The examples are written as practical command-line programs. They are meant to
show the moving parts clearly: reading audio data, transforming samples, writing
results, and keeping the code close to the underlying signal-processing idea.

Who this is for
---------------

This repo is useful if you are learning how audio programs work and want code
that is small enough to inspect. The examples are aimed at people who want to
build intuition for digital audio, sample processing, file formats, and basic
DSP workflows.

You do not need to treat the code as production audio software. It is teaching
code first: direct, explicit, and intentionally light on dependencies.

What to expect
--------------

Examples may cover audio-related tasks such as:

* reading and writing simple audio files
* changing sample values
* applying basic DSP operations
* comparing implementations in different languages
* learning how command-line audio tools are structured

Each example should document its own assumptions, supported input formats, and
commands. Start with the README inside an example directory when you want the
exact build and run steps for that example.

Repository layout
-----------------

The top-level folders are individual examples. Some examples may include more
than one implementation so learners can compare the same idea across languages.

A typical example may contain:

* source code
* a short README
* build files for the language or toolchain used
* notes about supported input and expected output

Running examples
----------------

Use the instructions in each example directory. In general, the workflow is:

#. choose an example
#. build or run the implementation for your language
#. pass an input audio file and an output path
#. listen to or inspect the result
#. change the code and try again

Please obtain audio files from legitimate sources. Thank You