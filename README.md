# DurableObjects-on-Rust


## Mission!

The [official Durable Objects docs](https://developers.cloudflare.com/durable-objects/) only cover JavaScript/TypeScript. However, it is possible to develop for DO in Rust. This project shows, how.

![](.images/Village-on-Rust.png)

*Imaginary town of Durable Objects-upon-Rust*


## Requirements

- `rust` with WASM target
- `npm`

	Cloudflare's CLI tool `wrangler` is an `npm` package.


### Using Multipass (optional)

If you develop with Multipass VM's, there are two things to consider.

#### Forward port

Use this command instead of `multipass shell`, to have the worker's port forwarded:

```
$ ./.mp.dive.sh
```

>You should study the contents of that script, and edit it to your liking.

#### Cache the `.wrangler` folder

Running local SQLite session uses `.wrangler/state` for storage. This **does not work** if mounted to the host. 

>`disk I/O error: SQLITE_IOERR`

Instead, mount the whole `.wrangler` folder, within your VM, by adding this to your `/etc/fstab`:

```
/home/ubuntu/.cache/_wrangler/DurableObjects-on-Rust /home/ubuntu/DurableObjects-on-Rust/.wrangler none user,bind,noauto,exec,rw,noatime,nodiratime 0 0
```

```
$ install -d ~/.cache/_wrangler/DurableObjects-on-Rust
```

```
$ sudo systemctl daemon-reload
```

```
$ mount .wrangler
```

Now, [Miniflare](https://github.com/cloudflare/workers-sdk/tree/main/packages/miniflare) is able to store SQLite database, within `.wrangler`.

>[! NOTE]
>
>If you intend to use `npm` modules, do the same also for `node_modules`:
>
>```
>/home/ubuntu/.cache/node_modules/DurableObjects-on-Rust /home/ubuntu/DurableObjects-on-Rust/node_modules none user,bind,noauto,exec,rw,noatime,nodiratime 0 0
>```
>
>```
>$ install -d ~/.cache/node-modules/DurableObjects-on-Rust
>```
>
>This improves performance, notably! (10x or more)


## Preparations

```
$ cargo install worker-build
```

>```
>$ worker-build --version
0.7.2
>```

This step would also be taken by `wrangler dev` but the author likes to do it explicitly.

```
$ npm install
```

## Steps

We use `npm` for consolidating the development commands. This makes sense, since though most of the code is Rust, the environment it ends up running in is Node.

**Build**

```
$ npm run build
[...]
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
[INFO]: ✨  Done in 1.04s
[INFO]: 📦   Your wasm pkg is ready to publish at /home/ubuntu/DurableObjects-on-Rust/build.

  index.js  25.0kb

⚡ Done in 45ms
```

>Note: `wrangler dev` (below) would do this step automatically, but it does not show the colors. Colors help!


**Launch locally**

```
$ npm run dev
[...]
⎔ Starting local server...
[wrangler:info] Ready on http://localhost:8787
```

<!-- hidden
>Note: `wrangler dev` still does a `--release` build of the WASM part.
-->

Command-double-click (macOS) on the URL. 

If all went fine, you should see some meaningful message on the browser.

- e.g. `Hey: xyz`


<!--
**Test**

*tbd.*

`cargo test` or `vitest`??
-->

<!--
## Deploy

```
$ wrangler deploy
```
-->

<!--
## Observe

...
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

