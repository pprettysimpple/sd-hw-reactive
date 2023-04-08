### Reactive microservice for managing users

#### How to build
- `cargo build` - build project

#### How to run
- `cargo run` - run project
    - this service is completely stateless, so you can easily run it on multiple machines
    - you can run it on the same machine multiple times, but you need to specify different ports for each instance
    - default port for service is 3000
- `docker-compose up` - run postgres in docker container with test data and settings
    - default port for postgres in container is 5555

#### How to stop
- `docker-compose down` - stop postgres container

##### How to run tests
Tests are not implemented yet