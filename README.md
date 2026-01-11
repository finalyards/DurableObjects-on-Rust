# DurableObjects-on-Rust


## Mission!

The [official Durable Objects docs](https://developers.cloudflare.com/durable-objects/) only cover JavaScript/TypeScript. However, it is possible to develop for DO in Rust. This project shows, how.

![](.images/Village-on-Rust.png)

*Imaginary town of Durable Objects-upon-Rust*


## Requirements

- `rust` with WASM target
- `npm` and/or `wrangler`

	Cloudflare's CLI tool `wrangler` is an `npm` package.


## Preparations

### Using Multipass (optional)

If you use Multipass VM, use the following commands instead of `multipass shell`, to have the port `8787` forwarded from the VM to the host:

```
$ ./.mp.dive.sh
```

---

```
$ cargo install worker-build
```

<!--```
$ worker-build --version
0.7.2
```-->

This step is taken by `npx wrangler dev` but the author likes to do it explicitly, at first.


## Steps

Build:

```
$ worker-build --dev
[...]
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
[INFO]: ✨  Done in 1.04s
[INFO]: 📦   Your wasm pkg is ready to publish at /home/ubuntu/DurableObjects-on-Rust/build.

  index.js  25.0kb

⚡ Done in 45ms
```

Launch locally:

```
$ npx wrangler dev
[...]
⎔ Starting local server...
[wrangler:info] Ready on http://localhost:8787
```

>Note: `wrangler dev` still does a `--release` build of the WASM part.

Command-double-click (macOS) on the URL. 


**Test???**

*tbd.*

<!--
## Release

```
$ worker-build --release
```
-->

<!--
```
wrangler build
wrangler dev
wrangler deploy
```
-->

## References

- Cloudflare Docs > Workers > Languages > [Rust](https://developers.cloudflare.com/workers/languages/rust/)

