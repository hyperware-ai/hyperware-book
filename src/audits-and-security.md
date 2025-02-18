# Audits and Security

Hyperdrive has been audited by [Enigma Dark](https://www.enigmadark.com/) (under our previous brandname: Kinode).
That report can be found [here](https://github.com/Enigma-Dark/security-review-reports/blob/main/2024-11-18_Architecture_Review_Report_Kinode.pdf).

However, the audit was not comprehensive and focused on the robustness of the networking stack and the kernel.
Therefore, other parts of the runtime, such as the filesystem modules and the ETH RPC layer, remain unaudited.
Hyperdrive remains a work in progress and will continue to be audited as it matures.

### Smart Contracts

Hyperdrive uses a number of smart contracts to manage global state.
Audits below:
- [Hypermap audit](https://cantina.xyz/portfolio/c2cbcbe7-727c-47cf-99f1-4e82ea8e5c77) by [Spearbit](https://spearbit.com/) (under our previous brandname: Kimap)
