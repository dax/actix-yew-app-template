# {{project-name | capitalize}}

[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)  
[![Coverage Status](https://coveralls.io/repos/github/{{gh-username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{gh-username}}/{{project-name}}?branch=main)
[![CI](https://github.com/{{gh-username}}/{{project-name}}/workflows/CI/badge.svg)](https://github.com/{{gh-username}}/{{project-name}}/actions)

{{project-name}} is ...

## Features

- [ ] 
 
## Installation

### Using cargo (for development)

```bash
cargo make run
```

### Manual

1. Get the code

```bash
git clone https://github.com/{{gh-username}}/{{project-name}}
```

2. Build api and web release assets

```bash
cargo make build-release
```

It will produce a `target/release/{{project-name}}-api` backend binary and frontend assets in the `web/dist` directory.

3. Deploy assets

```bash
mkdir -p $DEPLOY_DIR/config
cp -a target/release/{{project-name}}-api $DEPLOY_DIR
cp -a web/dist/* $DEPLOY_DIR
cp -a api/config/{default.toml, prod.toml} $DEPLOY_DIR/config
```

4. Run server

```bash
cd $DEPLOY_DIR
env CONFIG_FILE=$DEPLOY_DIR/config/prod.toml ./{{project-name}}-api
```

### Using Docker

#### Build Docker image

```bash
docker build -t {{project-name}} .
```

#### Run {{project-name | capitalize}} using Docker

```bash
docker run --rm -ti -p 8000:8000 project-name
```

## Usage

Access {{project-name | capitalize}} using [http://localhost:8000](http://localhost:8000)

## License

[AGPL](LICENSE)
