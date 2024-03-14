# example hurl file https://hurl.dev/
# This can be useful for exploring the LXD API and debugging.
GET http://lxd/1.0/instances
[Options]
unix-socket: /var/snap/lxd/common/lxd/unix.socket
HTTP 200
[Captures]
instance: jsonpath "$.metadata[0]"


GET http://lxd{{instance}}
[Options]
unix-socket: /var/snap/lxd/common/lxd/unix.socket
HTTP 200
