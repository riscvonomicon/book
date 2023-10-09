# NIST Suite: Encryption & Decryption (Zkned)

The *Zkned* set contains instructions for the [AES] block cipher. The extension
defines 4 instructions for `riscv32` and 7 instructions for `riscv64`. These
instructions can be used to implement AES-128, AES-196 and AES-256. The table
below lists all the instructions that are defined by the *Zkned* extension.

| 32-bit        | 64-bit      | Usage                   |
|---------------|-------------|-------------------------|
| `aes32dsi`    | `aes64ds`   | Decryption Final Round  |
| `aes32dsmi`   | `aes64dsm`  | Decryption Middle Round |
| `aes32esi`    | `aes64es`   | Encryption Final Round  |
| `aes32esmi`   | `aes64esm`  | Encryption Middle Round |
|               | `aes64ks1i` | Key Schedule            |
|               | `aes64ks2`  | Key Schedule            |
|               | `aes64im`   | Decryption Key Schedule |

This section contains usage examples for the [32-bit instructions](./32bit.md)
and for the [64-bit instructions](./64bit.md). These implementations can also
be found in the [GitHub repository with the examples for the entire *Zk*
extension](https://github.com/riscvonomicon/riscv-zk-in-rust).

[aes]: https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
