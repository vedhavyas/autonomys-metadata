#  Autonomys Metadata 🌗

Metadata Portal is a self-hosted web page that shows you the latest metadata for a given Autonomys network.

This is an important addition to Signer, which can update the metadata inside only through a special video QR code without going online.
Parity will host its own version of the page for all chains for which we sign the metadata.
External users (chain owners) will be able to deploy their versions of metadata portal if they want.

## Getting Started

### Generating Chain Spec and Metadat

1. Run update

```bash
cargo run --release -- update --signing-key "<private-key>"
```

2. Collect QRs

```bash
cargo run --release -- collect
```

### Metadata site Development

1. Install dependencies:

```bash
npm install
```

2. Run the development server:

```bash
npm run dev
```

Then visit `http://localhost:3000` in your browser.

### Static Build

To build the static export:

```bash
npm run build
```

The output will be in the `out/` directory.

## Deployment to GitHub Pages

This project is already configured for deployment to GitHub Pages using the `gh-pages` package.

### Steps

1. Ensure dependencies are installed:

```bash
npm install
```

2. Deploy the site:

```bash
npm run deploy
```