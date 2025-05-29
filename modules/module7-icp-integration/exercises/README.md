# Hello ICP

A simple greeting canister that runs on the Internet Computer.

## Overview

This canister provides the following functionality:

- Stores a greeting message
- Allows updating the greeting message
- Retrieves the current greeting message
- Retrieves the history of greeting messages

## Project Structure

```
hello_icp/
├── Cargo.toml             # Rust package configuration
├── dfx.json               # Internet Computer project configuration
├── hello_icp.did          # Candid interface definition
├── hello_icp.rs           # Rust canister implementation
└── README.md              # This file
```

## Setup and Deployment

1. Make sure you have the IC SDK (DFX) installed:

   ```bash
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

2. Start a local Internet Computer replica:

   ```bash
   dfx start --background
   ```

3. Deploy the canister:

   ```bash
   dfx deploy
   ```

## Interacting with the Canister

Once deployed, you can interact with the canister using the following commands:

1. Get the current greeting:

   ```bash
   dfx canister call hello_icp get_greeting
   ```

2. Update the greeting:

   ```bash
   dfx canister call hello_icp update_greeting '("Hello, Internet Computer!")'
   ```

3. Get the greeting history:

   ```bash
   dfx canister call hello_icp get_greeting_history
   ```

## Web Interface

You can also interact with the canister using the Candid UI:

1. Deploy the canister (if not already deployed):

   ```bash
   dfx deploy
   ```

2. Open the Candid UI in your browser:

   ```bash
   dfx canister id __Candid_UI
   ```

   Then open `http://localhost:8000/candid?canisterId=<__Candid_UI_canister_id>&id=<hello_icp_canister_id>` in your browser.

## Additional Resources

- [Internet Computer Documentation](https://internetcomputer.org/docs/current/developer-docs/)
- [Rust CDK Documentation](https://internetcomputer.org/docs/current/developer-docs/build/cdks/rust-cdk/) 