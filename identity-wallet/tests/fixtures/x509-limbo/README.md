# x509-limbo test chain

`signer.json` is the certificate chain of the testcase `pathlen::ee-with-intermediate-pathlen-0` from
[C2SP/x509-limbo](https://github.com/C2SP/x509-limbo), under the Apache License 2.0 (`LICENSE`), as identity-kit
vendors it in `tests/x509-limbo/signer.json` (impierce/identity-kit#172):

- `x5c`: the leaf certificate, with the DNS name `example.com`, and its intermediate, base64 DER;
- `trust_anchor`: the root certificate, base64 DER;
- `private_jwk`: the leaf's private key, published with the test suite.

The tests in `src/x509.rs` sign signed metadata and credentials with it. The key is public test data and signs nothing
else.
