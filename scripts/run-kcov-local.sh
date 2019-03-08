# https://docs.docker.com/engine/reference/commandline/build/#parent-command
docker build --file ../Docker/kcov.Dockerfile --tag raspberry-kcov .

# Create directory for storing cargo registry on host, if it does not exist already
REGISTRY="$(echo $HOME)"/raspberry-kcov/cargo/registry
mkdir -p $REGISTRY

# https://hackernoon.com/seamlessly-cross-compiling-rust-for-raspberry-pis-ede5e2bd3fe2
# https://matthiasnoback.nl/2017/04/docker-build-patterns/
# https://stackoverflow.com/questions/8426058/getting-the-parent-of-a-directory-in-bash
# https://hub.docker.com/r/ragnaroek/kcov/
docker run \
	--name raspberry-kcov \
	--rm \
	--mount type=bind,source="$(dirname "$(pwd)")",target=/shared \
	--mount type=bind,source=$REGISTRY,target=/usr/local/cargo/registry \
	--shm-size=256m \
	--security-opt seccomp=unconfined \
	raspberry-kcov