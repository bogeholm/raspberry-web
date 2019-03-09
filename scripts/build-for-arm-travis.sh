# https://docs.docker.com/engine/reference/commandline/build/#parent-command
docker build --file ../Docker/build-for-arm.Dockerfile --tag raspberry-build-travis .

# https://hackernoon.com/seamlessly-cross-compiling-rust-for-raspberry-pis-ede5e2bd3fe2
# https://matthiasnoback.nl/2017/04/docker-build-patterns/
docker run \
	--name raspberry-build-travis \
	--rm \
	--shm-size=256m \
	raspberry-build-travis
