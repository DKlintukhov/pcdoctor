# PCDoctor
PCDoctor is a comprehensive technical support platform combining a Telegram bot and Website. Users can describe their computer issues through either channel, and receive professional assistance from certified technicians via callback support.

## Installation
### Step 1: Clone the Repository
```bash
git clone git@github.com:DKlintukhov/pcdoctor.git
cd pcdoctor
```

### Step 2: Install Rust Dependencies
```bash
# Install Rust stable version
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add cargo to PATH
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

## Building the Application
```bash
# Build the backend
cargo build

# Run the backend
cargo run
```

## Testing
### Unit Tests
```bash
# Run tests
cargo test
```
