
## Preamble hacking on the web stuff steps


### Env Vars

Both the iron service and the react frontend employ `dotenv` for local 
development.

In `/.env` write the following vars (values are up to your specific needs).

```bash
DATABASE_URL="postgres://user:password@localhost/kaizen"
KAIZEN_API_ROOT="/api"
```

In `/web-frontend/.env` write

```bash
REACT_APP_KAIZEN_API_ROOT="/api"
```

Be sure that the two `API_ROOT` vars line up with each other.

### Database

The app runs on postgres, and uses `diesel`. As such, you should run `diesel
 setup` and `diesel run` as per the 
 [diesel docs](http://diesel.rs/guides/getting-started/).

### Code Style

Eh, there's a `rustfmt.toml` in the repo, but it's currently empty. Honor 
system. Run `cargo fmt` when you can. If you want to tweak the rules, go 
ahead and update the conf.

### Running

If you're like me, you will want the app(s) to auto-reload on code change. 
For the iron app, I use `watchexec` which you can install with cargo.

```bash
watchexec -rkcw server --exts rs "cargo run --bin kaizen"
```

This will recompile and re-run the web service on code changes in the 
`server/` dir.

Similarly, from the `web-frontend/` directory, after an initial `npm 
install`, you can run `npm run start` for a hot reloading dev server.


# Early domains to develop

* Pastes with annotations/comments
  * user accounts and auth (maybe try oauth from the get?) 
    * https://crates.io/crates/yup-oauth2
    * some crate for JWT?
  * repo interactions (create a repo on disk for a given paste)
  * comments wrt highlighted lines, partial lines, etc

