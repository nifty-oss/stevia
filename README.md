<h1 align="center">
  Stevia
</h1>
<p align="center">
  <img width="400" alt="Stevia" src="https://github.com/nifty-oss/stevia/assets/729235/3406121f-b224-484a-899c-210766ad7e58" />
</p>
<p align="center">
  A collection of lightweight zero-copy types.
</p>

<p align="center">
  <a href="https://github.com/nifty-oss/stevia/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/nifty-oss/stevia/main.yml?logo=GitHub" /></a>
  <a href="https://crates.io/crates/stevia"><img src="https://img.shields.io/crates/v/stevia?logo=rust" /></a>
</p>

## Getting started

From your project folder:

```bash
cargo add stevia
```

## Structure

The library is divided into several modules:

- `collections`: zero-copy flexible data structures.
- `pod`: Pod-enabled types.
- `types`: zero-copy data types.

## License

Copyright (c) 2024 nifty-oss maintainers

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

This crate is based on [Podded](https://crates.io/crates/podded) under the [MIT license](./LICENSE.third-party).
