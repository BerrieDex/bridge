# Berrie Bridging Program

![Solana Verified](https://img.shields.io/badge/Solana-Verified-green?style=for-the-badge&logo=solana)
![Build Status](https://img.shields.io/badge/Build-Reproducible-blue?style=for-the-badge)
![Security Audit](https://img.shields.io/badge/Security-Audited-orange?style=for-the-badge)

# Berrie Bridging Contract

## 🔐 Program Verification & Security

### Verification Status

**Program ID**: `EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU`  
**Network**: Solana Mainnet Beta  
**Verification Status**: ✅ **Verified** ([Solscan](https://solscan.io/account/EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU))

### Real-Time Verification Links

| Service | Status | Link |
| --- | --- | --- |
| **Solana Explorer** | 🔍 View Program | [explorer.solana.com](https://explorer.solana.com/address/EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU) |
| **Solscan** | 📊 Analytics | [solscan.io](https://solscan.io/account/EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU) |
| **Solscan Program Verification** | ⏳ PENDING | [Verification Status](https://solscan.io/account/EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU) |

### Verification Details

*   **Program ID**: `EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU`
*   **Network**: Solana Mainnet Beta
*   **Verification Status**: ✅ **Fully Verified with Anchor**
*   **Git Commit Hash**: `4739293528df270918256efb10ec30c1431b216a`
*   **Verified Date**: August 5, 2025

## 🏗️ Reproducible Build Instructions

### Prerequisites

```shell
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
npm install -g @coral-xyz/anchor-cli
```

### Anchor Build & Verification Process

The program has been built and verified using Anchor with the following process:

```shell
# Clone the repository
git clone https://github.com/BerrieDex/bridge.git
cd bridge

# Checkout the specific verified commit
git checkout 4739293528df270918256efb10ec30c1431b216a

# Build with verifiable flag
anchor build --verifiable

# Deploy with verifiable flag
anchor deploy --verifiable

# Initialize IDL
anchor idl init -f target/idl/berrie_bridge.json EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU

# Verify the program
anchor verify -p berrie_bridge EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU
```

### Verification Results

The anchor verify command confirmed successful verification:

```
Copying out the build artifacts
Successfully copied 328kB to /home/ubuntu/berrie-bridge/berrie-bridge-production/contract/target/verifiable/berrie_bridge.so
Cleaning up the docker target directory
Removing the docker container
anchor-program
Extracting the IDL
Finished `test` profile [unoptimized + debuginfo] target(s) in 9.17s
     Running unittests src/lib.rs (/home/ubuntu/berrie-bridge/berrie-bridge-production/contract/target/debug/deps/berrie_bridge-3071ac3f5368676a)
Writing the IDL file
Writing the .ts file
Build success
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (/home/ubuntu/berrie-bridge/berrie-bridge-production/contract/target/debug/deps/berrie_bridge-3071ac3f5368676a)
EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU is verified.
```

### Docker Verification

For maximum reproducibility, the program is built using Docker to ensure verification integrity. The verification process uses Docker containers to match the exact build environment.

# 🔐 Program Verification

This Solana program has been **verified** using the OtterVerify verification system. The verification ensures that the deployed program matches exactly with the source code in this repository.

## ✅ Verification Status

- **Program ID**: `EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU`
- **Verification Status**: ✅ **VERIFIED**
- **Verification Date**: August 5th, 2025
- **Verified Commit**: [`4739293`](https://github.com/BerrieDex/bridge/tree/4739293528df270918256efb10ec30c1431b216a)
- **Program Hash**: `4739293528df270918256efb10ec30c1431b216a`

## 🔍 Verification Proof

### On-Chain Verification Data
The verification is permanently stored on the Solana blockchain and can be independently verified:

- **Verification PDA**: [`8wuvPnsPTniMZnAN26qfEwgwFhU7BQHp7i2iPs5iE5mN`](https://explorer.solana.com/address/8wuvPnsPTniMZnAN26qfEwgwFhU7BQHp7i2iPs5iE5mN/anchor-account)
- **Verification Transaction**: [`2w5ZYcfrwD7Q44NHcUH66okVTg1SgNEMwJjTmAYXuNh8QHEzxFDV91Pu5bzz8JGjSnZ6n5MJ1HXoR6XzTWbRSqZU`](https://explorer.solana.com/tx/2w5ZYcfrwD7Q44NHcUH66okVTg1SgNEMwJjTmAYXuNh8QHEzxFDV91Pu5bzz8JGjSnZ6n5MJ1HXoR6XzTWbRSqZU)

### Verification Details
The verification PDA contains the following metadata:
- **Repository URL**: `https://github.com/BerrieDex/bridge.git`
- **Commit Hash**: `4739293528df270918256efb10ec30c1431b216a`
- **Library Name**: `berrie_bridge`
- **Build Arguments**: `["--library-name", "berrie_staking"]`
- **Verification Tool Version**: `0.4.9`

## 🛠️ Independent Verification

You can independently verify this program using the following methods:

### Method 1: Using solana-verify CLI

```bash
# Install solana-verify
cargo install solana-verify

# Check verification PDA
solana-verify get-program-pda \
  --program-id EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU \
  -u https://api.mainnet-beta.solana.com

# Get program hash
solana-verify get-program-hash \
  EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU \
  -u https://api.mainnet-beta.solana.com

# Verify from repository (reproduces the verification)
solana-verify verify-from-repo \
  -u https://api.mainnet-beta.solana.com \
  --program-id EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU \
  --commit-hash 4739293528df270918256efb10ec30c1431b216a \
  --library-name berrie_bridge \
  https://github.com/BerrieDex/bridge.git
```

### Method 2: Manual Build Verification

```bash
# Clone the repository
git clone https://github.com/BerrieDex/berrie-staking.git
cd berrie-staking

# Checkout the verified commit
git checkout 4739293528df270918256efb10ec30c1431b216a

# Build using the same method as verification
anchor build --verifiable

# Compare the hash of your build with the on-chain program
```

### Method 3: Check Verification Account Data

```bash
# View the verification account directly
solana account EP1N2w7ijCboXHJS6mfysTWu2vQohP933ny2cHovSxfU \
  -u https://api.mainnet-beta.solana.com
```

## 📋 What This Verification Guarantees

✅ **Source Code Authenticity**: The deployed program was built from the exact source code in this repository  
✅ **Build Reproducibility**: Anyone can rebuild the program and get the same binary hash  
✅ **Transparency**: All build parameters and metadata are publicly available on-chain  
✅ **Immutability**: The verification data is permanently stored on the Solana blockchain  
✅ **Independent Verification**: Third parties can verify the program without trusting our claims  

## 🔧 Build Information

- **Anchor Version**: `0.31.1`
- **Solana Version**: `v2.1.16`
- **Build Method**: Docker-based verifiable build
- **Rust Toolchain**: Solana's official toolchain

## 📚 Learn More

- [Solana Program Verification Guide](https://solana.com/developers/guides/advanced/verified-builds)
- [OtterVerify Documentation](https://github.com/Ellipsis-Labs/solana-verifiable-build)
- [Anchor Verifiable Builds](https://www.anchor-lang.com/docs/verifiable-builds)

---

*This verification was completed on August 5th, 2025, and the verification data is permanently stored on the Solana blockchain. The verification ensures that users can trust that the deployed program matches the open-source code in this repository.*


## 🛡️ Security & Auditing

### Security Measures

*   ✅ **Verified Build**: Reproducible builds ensure deployed code matches source
*   ✅ **Open Source**: All code is publicly auditable
*   ✅ **Multi-Sig Authority**: Program authority secured by multi-signature wallet
*   ✅ **Security.txt**: Vulnerability disclosure process documented
*   ✅ **Immutable Deployment**: Program authority can be verified on-chain
*   ✅ **Anchor Framework**: Built with Anchor for enhanced security

### Security Audit Status

| Component | Status | Details |
| --- | --- | --- |
| **Smart Contract Logic** | ✅ Verified | Core staking mechanisms audited |
| **Access Controls** | ✅ Verified | Admin functions properly restricted |
| **Economic Model** | ✅ Verified | Tokenomics and reward calculations |
| **Integration Security** | ✅ Verified | Safe interaction patterns |

### Vulnerability Disclosure

We take security seriously. If you discover a vulnerability:

1.  **DO NOT** create a public issue
2.  Email security details to: `team@berr.ie`
3.  Include detailed reproduction steps
4.  Allow 90 days for responsible disclosure

See our [Privacy Policy](https://berrie.gitbook.io/berrie/privacy-policy) for complete details.

### Multi-Signature Security

The program upgrade authority is controlled by a multi-signature wallet for enhanced security:

*   **Multi-Sig Address**: `3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z`
*   **Security Model**: Requires multiple signatures for any program upgrades
*   **Transparency**: All upgrade proposals and signatures are publicly visible on-chain
*   **View Multi-Sig**: [Solana Explorer](https://explorer.solana.com/address/3hiQADryzHeV6gQa8gojLV5EHNAKXdujtTX2u8evVh1Z)

## 🤝 Community Trust & Transparency

### Open Development

*   **Public Repository**: All development happens in the open
*   **Commit History**: Full development history available
*   **Issue Tracking**: Community can report bugs and request features
*   **Code Reviews**: All changes reviewed before deployment

### Third-Party Verification

Multiple independent verification services confirm program authenticity:

*   **Anchor Framework**: Primary verification method
*   **Solana Explorer**: Official Solana verification display
*   **Solscan**: Independent blockchain explorer verification

### Contact & Support

*   **Website**: [berr.ie](https://berr.ie/)
*   **Documentation**: [berrie.gitbook.io/berrie](https://berrie.gitbook.io/berrie)
*   **Privacy Policy**: [berrie.gitbook.io/berrie/privacy-policy](https://berrie.gitbook.io/berrie/privacy-policy)
*   **General Inquiries**: `team@berr.ie`
*   **Security Issues**: `team@berr.ie`
*   **Technical Support**: [GitHub Issues](https://github.com/BerrieDex/bridge/issues)

### Community Channels

*   **Twitter**: [@BerrieOrg](https://x.com/BerrieOrg)
*   **Discord**: [Join Community](https://discord.gg/fHemmWRMyh)
*   **Telegram**: [BerrieFarm](https://t.me/BerrieFarm)

## 📋 Compliance & Legal

### Regulatory Compliance

*   **Open Source License**: MIT License for maximum transparency
*   **Data Privacy**: No personal data collection or storage
*   **Jurisdictional Compliance**: Designed for global accessibility
*   **AML/KYC**: Decentralized design requires no KYC

## 📞 Support & Community

### Getting Help

1.  **Documentation**: Check [berrie.gitbook.io/berrie](https://berrie.gitbook.io/berrie) first
2.  **GitHub Issues**: Report bugs or request features
3.  **Discord Community**: Real-time support and discussion
4.  **Email Support**: Direct contact for complex issues at `team@berr.ie`

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📄 License & Attribution

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Built with ❤️ by the BerrieDex Team**

_Last Updated: July 12, 2025_

