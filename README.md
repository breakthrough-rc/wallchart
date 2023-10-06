# WallChart for Y'all

## Installing/Running
```bash
// First time you need to build the client (until automated, also should do on git pull)
./build-client.sh

// Run w. cargo
cargo run
```

## MiroBoard
This board contains our stories, kanban board and all other supplemental material.
https://miro.com/app/board/uXjVNffAsKM=/?share_link_id=874499028180

## Architecture & Tech Stack

### Frontend Client
This application required `bun` be installed. If you get an error regarding bun not being found, [please install bun](https://bun.sh/).

The `web-client` crate is responsible for building our client-side assets.
Assets for the client are reusable UI components, bespoke web controls (to extend our hypermedia client - the browser).
This crate leverage bun as our bundler for our TypeScript modules and TailwindCss (coming soon) for css.
See the `web-client` [README.md](./web-client/README.md) for more.

The `web-htmx` crate serves as "the backend for the frontend" using HTMX as the means to deliver a more rich UI w/out relying on custom JavaScript.

