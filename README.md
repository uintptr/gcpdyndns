## Update Dns Record

### Install

```
cargo install --git https://github.com/uintptr/udyndns
```

### Google Cloud Platform (GCP)

Only costs a few cents a month

Example:
```
/path/to/dyndns gcp \
                --auth-file /path/to/service_account.json \
                --project acme \
                --zone acme \
                --hostname hello.acme.com
```

### Digital Ocean

Free

```
/path/to/udyndns  digital-ocean \
                  --api-key-file /path/to/acme.token \
                  --hostname hello.acme.com
```

## Automation

Run Every 5 minutes in crontab

```
crontab -e
```

```
 */5 * * * * /path/to/udyndns digital-ocean  --api-key-file /path/to/acme.token --hostname hello.acme.com
```
