[![crates.io][crates-logo]][crate]

[![build][gh-logo]][build]

#### Summary
[`lists3`][repo] is a self-hosted streaming engine, that can render media files via authenticated sessions.

# ListS3
File Browser for S3 buckets

<details>
<summary><strong>Download pre-compiled OS specific executable</strong></summary>

###### macOS
```shell
curl -o lists3-darwin-amd64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/lists3/releases/latest/download/lists3-darwin-amd64.tar.gz"
```

###### macOS - M1
```shell
curl -o lists3-darwin-arm64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/lists3/releases/latest/download/lists3-darwin-arm64.tar.gz"
```

###### Linux
```shell
curl -o lists3-linux-amd64.tar.gz -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/lists3/releases/latest/download/lists3-linux-amd64.tar.gz"
```

###### Windows
```shell
curl -o lists3-windows-amd64.zip -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/lists3/releases/latest/download/lists3-windows-amd64.zip"
```
</details>

### Arguments

- **--bucket / -b**: Bucket name for which listing has to be created.
- **--region / -r**: Region name where the bucket is present. **Fallback:** Default Region
- **--filter / -f**: S3 prefix to filter (**eg:** `["github/"]`) **Fallback:** `[]`
- **--ignore / -i**: Objects to be ignored (**eg:** `["github/.DS_Store"]`) **Fallback:** `[]`
- **--object / -o**: Object name to upload in s3 (**eg:** `list.html`) **Fallback:** `list`
- **--proxy / -p**: Proxy server's path (**eg:** https://example.com/proxy) **Fallback:** https://jarvis.vigneshrao.com/proxy
- **--style / -s**: Styling for the UI (**eg:** `vanilla`) **Fallback:** bootstrap

### Sample

```shell
./lists3 --bucket thevickypedia.com --object list --filter '["github/"]' --ignore '["github/.DS_Store"]'
```

```shell
./lists3 --bucket thevickypedia.com --object list --filter '["github/"]'
```

## Crate
[https://crates.io/crates/lists3][crate]

### Cargo Docs - Official Runbook
[https://docs.rs/lists3/latest/lists3/][docs]

**Generator**
```shell
cargo doc --document-private-items --no-deps
```

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix
```

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][license]

[repo]: https://github.com/thevickypedia/lists3
[license]: https://github.com/thevickypedia/lists3/blob/main/LICENSE
[build]: https://github.com/thevickypedia/lists3/actions/workflows/rust.yaml
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust
[gh-logo]: https://github.com/thevickypedia/lists3/actions/workflows/rust.yaml/badge.svg
[crate]: https://crates.io/crates/lists3
[gh-checks]: https://github.com/thevickypedia/lists3/actions/workflows/rust.yml
[crates-logo]: https://img.shields.io/crates/v/lists3.svg
[docs]: https://docs.rs/lists3/latest/
