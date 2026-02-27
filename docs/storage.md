# Storage Options

Crabdrive currently supports two storage backends:
- The `SFS` (S~~tupid~~imple File System) is a very basic, synchronous storage backend, which implements minimal functionality.
- `C3` (please don't sue us) is an asynchronous, (hopefully) high-performance, storage backend which also includes caching. In some of my personal benchmarks, it outperforms `SFS` in downloading by 3-4 seconds (on a 750MB file). Keep in mind, that these tests are however not reliable.

## Configuration

To configure the storage directory, either use the environment variable `CRABDRIVE_STORAGE_DIR` or add this to your `crabdrive.toml` (or whatever your config file is):
```toml
[storage]
dir = # Your directory here
```

Keep in mind, that storage directories may not be automatically created. Furthermore, storage backends may overwrite already existing data, so make sure they are empty.

### Caching

When selecting `C3` as storage backend, you can customize some cache settings:

- `CACHE_SIZE`: The maximum capacity of the cache for the (in-memory) cache. Defaults to 20 chunks (20 * 17 MiB = ca. 350 MB)
- `CACHE_AHEAD`: This determines how many chunks should be cached ahead of transfer

### Garbage Collection

`C3` can garbage-collect staled uploads. By default uploads are marked as stale after receiving no data in 5 minutes and are hard-deleted after another 5 minutes.
