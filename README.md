# Example Cross Contract (to-and-fro) - ink!

Two ink! smart contract call each other without directly importing each other.

## Project Structure
```
├── contracts       // contracts, individual cargo crates
│   ├── one
│   └── two
├── traits          // common traits definitions
│   ├── mod.rs
│   ├── one.rs
│   └── two.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
└── lib.rs          // root project to export traits
```

Learn more about how to organize ink!-openbrush projects - https://docs.openbrush.io/smart-contracts/example/setup_project