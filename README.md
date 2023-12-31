

# Fair Squares (FS) &middot; [![Twitter URL](https://img.shields.io/twitter/follow/fairsquares?style=social)](https://twitter.com/fairsquares) [![License](https://img.shields.io/github/license/fair-Squares/fair-squares)](https://github.com/Fair-Squares/fair-squares/blob/main/LICENSE) [![Discord](https://img.shields.io/badge/Discord-gray?logo=discord)](https://discord.gg/5u3dxE49V5)

<div align=center>
    <img align=top src="assets/img/FS.png" width=30%/>
    <img align=top src="assets/img/web3_foundation_grants_badge_white.svg" width=40%/>
</div>
</br>

**Fair Squares** connects supply and demand of house-owners & renters and houses & investors. Our motive is that we want to create an more affordable housing market. Investors of the house get a social return while renters can have cheaper housing. We want to remove the financial barrier of investing in real estate for investors that don't have the means to fully invest in a house themselves for a social return. In between the end-users, there is coordination taking place between different stakeholders to achieve the desired outcome. This is where the runtime and the logic of all pallets come together, orchestrating while adhering to strict rules set for an equitable system. The orchestration towards an equitable housing market is configurable and governable by the stakeholders that are concerend with it and are willing to work for it. 

We are zooming much more on the problem definition, stakeholders and the solution in our paper on our [website](https://www.fair-squares.nl/). To learn more and get in touch with us, please join our [discord channel FS](https://discord.gg/5u3dxE49V5)

Our current development is funded by [Web3 Foundation Grants Program](https://github.com/w3f/Grants-Program)

</br>

## Run & build
### Running locally
1. complete the [basic Rust setup instructions](./docs/rust-setup.md).
1. `cargo run  --release -- --dev --tmp` in the root of the fs-node repo.
### Build locally

The `cargo build` command will perform an initial build. 

```sh
cargo build --release
```
The binary will be present in create the binary in `./target/release/fs-node` if not other argument is passed. 

### Docker build & run
We added a [Dockerfile](https://github.com/Fair-Squares/fair-squares/blob/main/Dockerfile) in the repo, you can build an image yourself with the following command `docker build .`

### Docker images

The images that are tagged starting with `v0.x.x` generate a docker image. You can see the available images [here](https://github.com/Fair-Squares/fair-squares/pkgs/container/fs-node)

run command: </br> 
`docker run  --publish=127.0.0.1:9944:9944/tcp ghcr.io/fair-squares/fs-node:{$VERSION} fs-node  --dev --ws-external`

You have to change the `$VERSION` in the line above.

### Run in Docker in linux

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.
```bash
./scripts/docker_run.sh
```
The script above will need a folder in the root of this project called `.local` , you will have to create this folder yourself.


This command will firstly compile your code, and then start a local development network. You can also replace the default command
(`cargo build --release && ./target/release/fs-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

### Connect with Polkadot-Js apps front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end to interact with your chain. [Polkadot.js](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connects a front-end is the app that can interact with the node by means of extensics calls and can read the chain state of the blockchain. Click [here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) to connect to the local blockchain

## Run all tests

```
cargo test
```