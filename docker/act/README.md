# Testing GitHub Actions

First, install [act](https://github.com/nektos/act)

Then, build the custom act image

```bash
docker build -t provwasm/act-environments-ubuntu . -f docker/act/Dockerfile
```

Then, to run the provwasm GitHub actions locally

```bash
act -P ubuntu-latest=provwasm/act-environments-ubuntu:latest
```

To leave off the args to `act`, place the following in ~/.actrc

```bash
-P ubuntu-latest=provwasm/act-environments-ubuntu:latest
-P ubuntu-18.04=provwasm/act-environments-ubuntu:latest
```
