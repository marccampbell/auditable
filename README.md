[![Build Status](https://travis-ci.org/auditable/auditable.svg?branch=master)](https://travis-ci.org/auditable/auditable)

## Contributing
You can use our development Dockerfile to quickly get a full dev environment up and running:  

```shell
sudo docker build -t auditable-dev -f scripts/Dockerfile.dev .
sudo docker run -p 3000:3000 -v `pwd`:/code -it -e REDIS_URI=redis://<redis-ip>/ auditable-dev /bin/bash
```

Be sure to replicated <redis-ip> with the ip address of a Redis server.  If you need to install one, consider:

```shell
sudo docker run -p 6379:6379 -d redis
```

These commands will leave you in a dev containers with all dependencies already installed, and at a prompt in `/code`.

Now, just run `make run` and you'll be running the code.  Edit and run `make run` as many times as you'd like.

Your dev server will be listening on port 3000.
