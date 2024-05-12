# ListS3
File Browser for S3 buckets

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
