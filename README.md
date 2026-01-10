# DurableObjects-on-Rust


## Mission!

The [official Durable Objects docs](https://developers.cloudflare.com/durable-objects/) only cover JavaScript/TypeScript. However, it is possible to develop for DO in Rust. This project shows, how.

![](.images/Village-on-Rust.png)

*Imaginary town of Durable Objects-upon-Rust*


## Requirements

- `wrangler`

	- ...tbd.

## Preparations

```
$ cargo install worker-build
```

<!--```
$ worker-build --version
0.7.2
```-->

This step is taken by `npx wrangler dev` but the author likes to do it explicitly, at first.


## Steps

```
$ worker-build --dev
[...]
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
[INFO]: ✨  Done in 1.04s
[INFO]: 📦   Your wasm pkg is ready to publish at /home/ubuntu/DurableObjects-on-Rust/build.

  index.js  25.0kb

⚡ Done in 45ms
```

```
$ worker-build --release
```


<!--
```
wrangler build
wrangler dev
wrangler deploy
```
-->

## References

- Cloudflare Docs > Workers > Languages > [Rust](https://developers.cloudflare.com/workers/languages/rust/)

