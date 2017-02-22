rust-ldap
=========

A Pure-Rust LDAP Library.

The `ldap` crate itself is a meta-crate only exporting symbols from `ldap_protocol`, `ldap_client`,
`ldap_server` and `ldap_rfc4515`.

[![Crate](https://img.shields.io/crates/v/ldap.svg)](https://crates.io/crates/ldap)
[![Documentation](https://docs.rs/ldap/badge.svg)](https://docs.rs/ldap)

### ldap_protocol

This crate implements the low-level workings of RFC4511 and ASN.1's BER, and some shared structures
useful for both Server and Client abstractions (e.g. an Error type).

It makes no attempt at abstracting over the inner workings of LDAP or being comfortable to use at
all.

You will most likely never use this library directly in any of your project but use `ldap_client` or
`ldap_server` instead.

#### Status:

- [x] BER En-/Decoding. I would like to eventually offload this to eagre-asn or another library but
      there are no good ones I found so far.
- [x] Message Envelope (4.1.1)


### ldap_client

This library creates Client-opinionated abstractions over `ldap_protocol`.

Implemented *at all*:

- [x] Bind (4.2)
- [x] Unbind (4.3)
- [ ] Search (4.5)
- [ ] Modify (4.6)
- [ ] Add (4.7)
- [ ] Delete (4.8)
- [ ] Modify DN (4.9)
- [ ] Compare (4.10)
- [ ] Abandon (4.11)
- [ ] Extended Operation (4.12)
- [ ] TLS / STARTTLS (4.14 / 5)
- [ ] Anything actually useful that would make this crate comfortable to use. (i.e. the event queue)

##### Note that none of the functions are finalized. They will be reworked.


### ldap_server (Server-opinionated abstractions)

This library creates Server-opinionated abstractions over `ldap_protocol`.
It's not yet written though.


### ldap_rfc4515 (Search Filter String Representation)

Implemeting RFC 4515 Search Filter.

Example of a Search filter: `(& (objectClass=person) (| (cn=Username) (SAMAccountName=Username)))`.

Not yet written either.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
