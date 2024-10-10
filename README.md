# Fortuna
Pseudo random number generator for all platforms.

If you are looking for a true CSPRNG you should use [Tyche](https://github.com/xqhare/tyche).

Fortuna is only available on x86_64, riscv64 and aarch64, but supports any OS.

## Naming
Fortuna is named after the ancient roman goddess of fortune. Her greek equivalent is Tyche and lends her name to my own CSPRNG.

## How Fortuna generates random numbers
Fortuna relies on the system it is executed on to provide entropy.

Fortuna is deterministic, so it will always return the same number for the same inputs. Because of this, some inputs have been chosen to never return the same number (e.g. System Time).
This means that if the complete state of the system is known it is possible to predict the next number.

Other inputs will always return the same number, like CPU features or the amount of files in the root directory.

Most, if not all variation is provided by the measured time spend building the pool, or parts of it.

All inputs are combined into one entropy pool. This pool is then used to scramble the pool itself, and then the scrambled pool is used to generate random numbers.

The pool is finite in size, and will be refilled as needed.

## Entropy sources
Fortuna uses the following entropy sources:
- System time
- UNIX Epoch
- File system properties
    - Main disk device ID and inode number
    - Directory nest depth to root
    - Current working directory properties
- Timing
- CPU features
