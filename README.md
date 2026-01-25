# DurableObjects-on-Rust


## Mission!

The [official Durable Objects docs](https://developers.cloudflare.com/durable-objects/) only cover JavaScript/TypeScript. However, it is possible to develop for DO in Rust. This project shows, how.

![](.images/Village-on-Rust.png)

*Imaginary town of Durable Objects-upon-Rust*


## Requirements

- `rust` with WASM target
- `npm`

	Cloudflare's CLI tool `wrangler` is an `npm` package.
	
	Also testing uses Cloudflare's [Vitest integration](https://developers.cloudflare.com/workers/testing/), i.e. `npm`.


### Using Multipass (optional)

If you develop with Multipass VM's, there are two things to consider.

<!-- not needed, if only APIs exposed; tests work!
#### Forward port

Use this command instead of `multipass shell`, to have the worker's port forwarded:

```
$ ./.mp.dive.sh
```

>You should study the contents of that script, and edit it to your liking.
-->

#### Cache the `.wrangler` folder

Running local SQLite session uses `.wrangler/state` for storage. This **does not work** if mounted to a network drive (i.e. Multipass mount).

> Gives:
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

>Note: Below, we'll map `node_modules` to a `tmpfs` (memory disk). This has turned out to be better than persisting the contents on the VM disk. It will work for `.wrangler` as well.


#### Mount the `node_modules` folder to `tmpfs`

For `node_modules`:

```
# append to /etc/fstab
tmpfs2201 /home/ubuntu/DurableObjects-on-Rust/node_modules tmpfs user,noauto,exec,rw,size=700m,uid=1000,gid=1000 0 0
```

This is *vital* for performance, but it also keep the VM side (Linux) modules away from host side (e.g. macOS), which matters for binary modules.

>Note: Using `tmpfs` is simpler, but the folder needs to get re-populated after each VM restart. If you prefer persistence, do the same as for `.wrangler`, above.

#### Restart

```
$ sudo systemctl daemon-reload
```

This allows you to actually mount, with the changed `/etc/fstab`:

```
$ mount .wrangler
$ mount node_modules
```

## Preparations

```
$ cargo install worker-build
```

>```
>$ worker-build --version
0.7.4
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


**Test**

```
$ npm test
```

>The tests are written using standard Vitest.
>
>This is because [`@cloudflare/vitest-pool-workers`](https://github.com/cloudflare/workers-sdk/tree/main/packages/vitest-pool-workers#readme) (GitHub) doesn't really provide any added benefit over *any* HTTP testing framework, in our case.
>
>This is because `workers-rs` requires us to route access to Durable Objects via a `http` proxy worker. Thus, we just need to test that API.
>
>In addition, if there were needs to test *internals* of the Rust implementation, it would be a job for Rust testing tools.


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

<!-- hidden; didn't use it
- Cloudflare Docs > Workers > Testing > Vitest integration > [Write your first test](https://developers.cloudflare.com/workers/testing/vitest-integration/write-your-first-test/)
-->

