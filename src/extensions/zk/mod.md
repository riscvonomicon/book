# Overview of RISC-V Scalar Cryptography (Zk) Extension

The RISC-V Scalar Cryptography is a small extension that helps embedded and application processors to reduce code size, reduce the energy consumption and reduce the execution time of cryptographic code. The extension consists of five parts.

- Bit-manipulation instructions for cryptography (`Zbkx`, `Zbkc` and `Zbkb`).
- `Zks` which defines the instructions relating to the ShangMi Suite. This includes the [SM3 hash function][sm3] and the [SM4 block cipher][SM4].
- `Zkn` defines instructions for the NIST Suite cryptographic primitives including [AES] block cipher and the [SHA-2][SHA2] hash function.
- `Zkr` defines a [CSR] for a hardware entropy source.  This can be used as a secure source of randomness.
- `Zkt` specification for constant time execution of specific instructions.

This chapter talks about these parts and how they can be used.











## Sources

[sm4]: https://en.wikipedia.org/wiki/SM4_(cipher)
[sm3]: https://en.wikipedia.org/wiki/SM3_(hash_function)
[aes]: https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
[sha2]: https://en.wikipedia.org/wiki/SHA-2
[CSR]: ./csr.md