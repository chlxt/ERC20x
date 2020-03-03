# Customized ERC20 API Implementation

This project implements a customized ERC20 API demo, Which tries to deal with 2 problems of ERC20 standard,

    1. Security of `approve`/`transfer_from` methods, <https://docs.google.com/document/d/1YLPtQxZu1UAvO9cZ1O2RPXBbT0mooh4DYKjA_jp-RLM/edit>
    2. Token lost problem when wrongly using `transfer` method, see

        - [Critical problems of ERC20 token standard](https://medium.com/@dexaran820/erc20-token-standard-critical-problems-3c10fd48657b)
        - [ERC20_token_standard_vulnerability_classification.md](https://gist.github.com/Dexaran/ddb3e89fe64bf2e06ed15fbd5679bd20)
        - [Ethereum Tokens: ERC 20 vs. ERC 223 vs. ERC 777](https://www.blockchain-council.org/ethereum/ethereum-tokens-erc-20-vs-erc-223-vs-erc-777/)
        - [ERC777 — Is the new token standard replacing the ERC20?](https://hackernoon.com/erc777-is-the-new-token-standard-replacing-the-erc20-fd6319c3b13)

## Status

- [x] [problem1]()
- [ ] [problem2]()
