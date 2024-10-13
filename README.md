# Fortuna
Pseudo random number generator for all platforms.

If you are looking for a true CSPRNG check out my project [Tyche](https://github.com/xqhare/tyche).

Fortuna is available on x86_64, riscv64 and aarch64 CPU architectures, and supports any OS.

## Features

- Performance
    - If compiled with release options, Fortuna needs about 1 - 4 ms to generate an entire entropy pool

## Naming
Fortuna is named after the ancient roman goddess of fortune. Her Greek equivalent is Tyche and lends her name to my own CSPRNG.

## How Fortuna generates random numbers
Fortuna relies on the system it is executed on to provide entropy.

Fortuna is deterministic, so it will always return the same number for the same inputs. Because of this, some inputs have been chosen to never return the same number (e.g. System Time).
This means that if the complete state of the system is known it is possible to predict the next number.

Other inputs will always return the same number, like CPU features or the amount of files in the root directory.

Most, if not all variation is provided by the measured time spend building the pool, or parts of it.

Some inputs are combined into one entropy pool. The others are also pooled together to form a second pool. The second pool is then used to scramble the entropy pool itself.

The entropy pool is finite in size, but should be around 100k bytes of entropy.

The entropy pool will be refilled as needed.

## Entropy sources
Fortuna uses the following entropy sources:
- System time
- File system properties (if available)
    - Main disk device ID and inode number
    - Directory nest depth to root
    - Current working directory properties
- Timing
- CPU features
