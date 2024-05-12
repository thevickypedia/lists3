# ListS3
File Browser for S3 buckets

### Arguments

- **bucket**: Bucket name for which listing has to be created.
- **region**: Region name where the bucket is present. **Fallback:** Default Region
- **prefix**: S3 prefix name to filter (**eg:** `["github/"]`) **Fallback:** `[]`
- **ignore**: Objects to be ignored (**eg:** `["github/.DS_Store"]`) **Fallback:** `[]`
- **filename**: Object name to upload in s3 (**eg:** `list.html`) **Fallback:** `list`
- **proxy**: Proxy server's path (**eg:** https://example.com/proxy) **Fallback:** https://jarvis.vigneshrao.com/proxy
- **style**: Styling for the UI (**eg:** `vanilla`) **Fallback:** bootstrap
- **version**: Get the package version.

### Sample

```shell
./lists3 --bucket thevickypedia.com --filename list --prefix '["github/"]' --ignore '["github/.DS_Store"]'
```

```shell
./lists3 --bucket thevickypedia.com --filename list --prefix '["github/"]'
```
