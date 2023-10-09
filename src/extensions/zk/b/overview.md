| Instruction | 32-bit | 64-bit | Description |
|-|-|-|-|
| `ror` | x | x | Rotate Right by register value |
| `rol` | x | x | Rotate Left by register value |
| `rori` | x | x | Rotate Right by immediate value |
| `rorw` |  | x | Rotate Word Right by register value |
| `rolw` |  | x | Rotate Word Left by register value |
| `roriw` |  | x | Rotate Word Right by immediate value |
| `andn` |  x | x | Bitwise And & Negate |
| `orn` |  x | x | Bitwise Or & Negate |
| `xnor` |  x | x | Exclusive-Not-Or |
| `pack` |  x | x | Pack register from two register low-halves | 
| `packh` |  x | x | Pack register halfword from two register low-bytes |
| `packw` |  | x | Pack register word from two register low-halfwords |
| `brev8` |  x | x | Reverse bits within bytes |
| `rev8` |  x | x | Reverse bytes within register |
| `zip` |  x | | Zip upper and lower register halves into odd and even bits |
| `unzip` |  x | Unzip odd and even bits into upper and lower register halves |
| `clmul` |  x | x |
| `clmulh` |  x | x |
| `xperm8` |  x | x |
| `xperm4` |  x | x |
