# Vendored `prism-code`

This directory is a byte-for-byte copy of the runtime and journey modules from the
published `prism-code@0.11.1` npm package. Nothing here is rebuilt.

- Package: `prism-code@0.11.1`
- Source: <https://registry.npmjs.org/prism-code/-/prism-code-0.11.1.tgz>
- Tarball shasum (SHA-1): `c421439936d35eb1c3dbaa1a4654b3e2f20b1a1a`
- Tarball integrity:
  `sha512-yclUShNPIRE5Sk5g2qaW68njZmfeEVImqK/UZUfSL85SPjyLHhxHjwrXdtHhcEhHH5xmIQRI2dK4flv3T4JuYw==`
- Files taken: `src/`, `fixtures/Calculator.holo`, `LICENSE`.
- `package.json` here is a minimal replacement that keeps `version` and
  `type: module` (the published manifest is not needed for relative imports).
- `fixtures/Calculator.holo`: 24,801 bytes, SHA-256
  `84371aaac82af6b2a81a25a81d43d4097892985f55ae4919cfdcfa8b6e3486e7`.

The try page imports `src/browser.mjs` directly, and `bootstrap/worker-runtime.mjs`
packages the 13-module cold-boot archive/verifier closure (`worker-runtime/`) for
Holo v4, v5, and v6 archives and embedded DSSE kernel-proof records.
