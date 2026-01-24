import { describe, expect, it } from 'vitest';

type Sample = {
  when: string, // ISO8601
  temperature_c: number
}

// Each run supposedly gets a unique location (i.e. we don't clear them)
const loc = "test_"+ Math.random().toString(36).substring(2,7);
console.debug("loc: " + loc);

const data: Sample[] = Array.from({ length: 7 }, () => gen_fake_sample());

const PORT = "8787";  // tbd. allow 'PORT' env.var.
const BASE = `http://localhost:${PORT}`;

describe(`Access ${loc}`, () => {

  it('accepts samples', async () => {
    const res = await fetch(`${BASE}/${loc}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
    });

    expect(res.ok).toBe(true);
    // tbd. expect text reply, with empty body

    const body = await res.json();
    expect(body.success).toBe(true);
    expect(body.data).toEqual([]);
  });

  it('returns the same samples, in order', async () => {
    const res = await fetch(`${BASE}/${loc}`);  // GET

    expect(res.ok).toBe(true);
    // MIME to be 'application/json'

    const body = await res.json();
    expect(body.samples).toHaveLength(7);
    expect(body.samples).toEqual( data.sort((a, b) => Date.parse(a.when) - Date.parse(b.when)) );
  })
});

//--- Helper
function gen_fake_sample(): Sample {
  let a = Date.now() - Math.random() * (1000 * 60 * 60 * 24 * 365);
  let when = new Date(a).toISOString();

  return {
    when,
    temperature_c: -40 + Math.round(80 * Math.random()),
  };
}
