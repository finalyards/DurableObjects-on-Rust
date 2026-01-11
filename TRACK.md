# Track

## Ability to do RPC, within Rust

In JavaScript/Typescript, Durable Objects can be reached (say, from a worker `fetch` simply by their methods:

```
let stub = env.MY_DO.get(id);
let result = await stub.abc({ name: "foo" });
```

```
export class MyDO { async abc(o) { ... } }
```

This is simple!

In Rust, as of Jan'26, this doesn't seem to be the case, yet.

- [ ]()

	Follow the entry.
	
	