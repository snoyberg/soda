# soda

[![Rust](https://github.com/snoyberg/soda/workflows/rust.yml/badge.svg)](https://github.com/snoyberg/soda/actions/workflows/rust.yml)

Simple command line utility for encrypting and decrypting secrets.

More information to come if anyone actually cares. For now writing a bare minimum README.

## Usage

Sample session:

```shellsession
$ soda generate
Public key (send to others for encrypting): sodapub508f94aa48d0e6f5b7386e1b7eefc21125905ddc44889b5195d61864872d7e17
Private key (keep for yourself for decrypting): sodapriv0d06311a6b2cc82f9ba46cc52d53f32d21f7efcae795c61a05bdbc14b9d8d1f9
$ soda encrypt sodapub508f94aa48d0e6f5b7386e1b7eefc21125905ddc44889b5195d61864872d7e17 "this is a secret"
q0+BOjPHLKYW1DkePmWU90n0wjHbr6rKHZTKFiqlaUhfeh0h5ZDWPmmKY1+7FoHVjNZlAA39vsc7Q+HvNxFdDg==
$ soda decrypt sodapriv0d06311a6b2cc82f9ba46cc52d53f32d21f7efcae795c61a05bdbc14b9d8d1f9 "q0+BOjPHLKYW1DkePmWU90n0wjHbr6rKHZTKFiqlaUhfeh0h5ZDWPmmKY1+7FoHVjNZlAA39vsc7Q+HvNxFdDg=="
this is a secret
```

## The name

I started with the Hebrew word סוד, pronounced like "sod" in "soda." I realized if I spelled it like that, people would pronounce it "sahd". So for fun I called it soda. When I asked my brilliant wife where she thought the name came from, she guessed sodium. Given that this whole executable is built on the sodium library, it seems incredibly apropos.
