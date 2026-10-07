# Health Checks, Liveness & Readiness

> Runtime support: this project currently supports Proton only. The Wine target was removed because it failed the cold-volume install path; if you were on an older Wine-based tag, migrate to `mbround18/enshrouded-docker:proton-latest` and re-provision the data volume. See [migration-notice.md](./migration-notice.md).

The `monitor` process that runs alongside your server exposes a small HTTP API
so Docker, Kubernetes, or your own monitoring can tell whether the server is
up and whether it's ready for players.

You can't get this from the game ports themselves: Enshrouded's protocol has
no lightweight "are you there" query, and a plain TCP connect to `15636` tells
you nothing useful. The monitor already tracks the server process and the game
port, so it reports what it knows over HTTP instead.

## Endpoints

Served on `HTTP_PORT` (default `3000`), bound to `0.0.0.0`.

| Route                  | `200 OK` when                                   | Otherwise          |
| ---------------------- | ----------------------------------------------- | ------------------ |
| `/live`, `/liveness`   | the `enshrouded_server.exe` process is running  | `503` + JSON body  |
| `/ready`, `/readiness` | the game port is accepting connections          | `503` + JSON body  |
| `/health`              | both of the above are true                      | `503` + JSON body  |

Every response is JSON, so a probe can decide purely on the status code while
a human still gets the detail:

```console
$ curl -s localhost:3000/ready | jq
{
  "ready": true,
  "status": "accepting connections",
  "gamePort": 15636,
  "uptimeSeconds": 412
}
```

**Liveness vs. readiness.** Liveness answers "is the process alive, or should
this container be restarted?". Readiness answers "should players be sent
here?" — it stays `false` through the whole install/world-load phase and again
across a scheduled restart, which is exactly when you don't want traffic.
Don't use liveness to wait for startup; use a startup probe (below) for that,
or the long `start-period` the image's own `HEALTHCHECK` already sets.

Set `HTTP_PORT=0` to turn the HTTP server off entirely.

## Docker

The image ships a `HEALTHCHECK` already, so `docker ps` reports
`healthy`/`unhealthy` with no configuration. It shells out to the CLI rather
than `curl` (which the image doesn't include):

```bash
docker exec enshrouded enshrouded health          # exit 0 = healthy
docker exec enshrouded enshrouded health --route ready
```

To reach the endpoints from outside the container, publish the port:

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:proton-latest
    ports:
      - "15636:15636/tcp"
      - "15636:15636/udp"
      - "15637:15637/tcp"
      - "15637:15637/udp"
      - "3000:3000/tcp" # health endpoints
```

## Kubernetes

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: enshrouded
spec:
  serviceName: enshrouded
  replicas: 1
  selector:
    matchLabels: { app: enshrouded }
  template:
    metadata:
      labels: { app: enshrouded }
    spec:
      # The server saves the world on shutdown and that isn't instant.
      terminationGracePeriodSeconds: 120
      containers:
        - name: enshrouded
          image: mbround18/enshrouded-docker:proton-latest
          env:
            - name: NAME
              value: "My Enshrouded Server"
            - name: SET_GROUP_ADMIN_PASSWORD
              valueFrom:
                secretKeyRef: { name: enshrouded, key: admin-password }
          ports:
            - { name: game, containerPort: 15636, protocol: UDP }
            - { name: game-tcp, containerPort: 15636, protocol: TCP }
            - { name: query, containerPort: 15637, protocol: UDP }
            - { name: query-tcp, containerPort: 15637, protocol: TCP }
            - { name: http, containerPort: 3000, protocol: TCP }

          # First boot downloads the entire game through steamcmd, so give
          # startup a long leash. Until this succeeds the liveness and
          # readiness probes are not evaluated at all.
          startupProbe:
            httpGet: { path: /live, port: http }
            periodSeconds: 15
            failureThreshold: 80 # ~20 minutes

          livenessProbe:
            httpGet: { path: /live, port: http }
            periodSeconds: 30
            failureThreshold: 3

          readinessProbe:
            httpGet: { path: /ready, port: http }
            periodSeconds: 15
            failureThreshold: 3

          volumeMounts:
            - { name: data, mountPath: /home/steam/enshrouded }
  volumeClaimTemplates:
    - metadata:
        name: data
      spec:
        accessModes: ["ReadWriteOnce"]
        resources:
          requests:
            storage: 30Gi
```

A couple of things worth knowing:

- Keep `replicas: 1`. Two pods sharing one save directory will corrupt it.
- `terminationGracePeriodSeconds` must be long enough for the world save, the
  same reason `stop_grace_period: 120s` appears in the Compose examples.
- If you enable `SCHEDULED_RESTART`, readiness goes `false` for the duration of
  the restart and comes back on its own — that's the probe working, not a
  fault.
