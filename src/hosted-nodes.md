# Hosted Nodes User Guide

Sybil Technologies historically ran a Hyperware hosting service for users who did not want to run a node themselves.
However, as of May 2026 that service has been shut down.
Users can claim their node archives and transfer onchain ownership by navigating to [hosted.hyperware.ai](https://hosted.hyperware.ai) and logging in as normal.

## Tranferring onchain ownership

Hyperware nodes are NFTs.
Hosted nodes are owned onchain by the hosting service for user convenience.
To take control of a hosted node, log in, connect a wallet, and click `Transfer Ownership`.
This will send the node NFT from the hosting service wallet to the connected wallet.

## Downloading node archive

After logging in, `.tar.gz` archives are available for download.

A `.tar.gz` file is like a `.zip` file.
You can extract it with the following programs (amongst others):

| OS      | Program                             |
|---------|-------------------------------------|
| Windows | [7-zip](https://7-zip.org)          |
| macOS   | Finder, built-in                    |
| Linux   | `tar zxf` or built-in File explorer |

To run the node, first `Reset Password & Networking Info`.
The node must be owned by a connected wallet: see [Transferring onchain ownership](#transferring-onchain-ownership) for how to transfer the node onchain.
