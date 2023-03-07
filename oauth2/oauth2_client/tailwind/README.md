# Tailwind CSS

General install instructions here: <https://crates.io/crates/tailwindcss-to-rust>

Caveats:

- Install tailwind css with npm globally

- add to PATH: `export PATH=/usr/lib/node_modules/tailwindcss:${PATH}`

- Install tailwindcss-to-rust with Cargo

- Add tailwindcss-to-rust and related macros to cargo.toml

Command to generate rust from tailwind will look similar to below (paths may need to change, directories need to exist):

```sh
tailwindcss-to-rust \
     --tailwind-config tailwind/tailwind.config.js \
     --input tailwind/tailwind.css \
     --output src/css/generated.rs \
     --rustfmt
```
