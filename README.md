# Takehome Assignment

This is a Rust writeup of a certain takehome assignment to produce a simplified version of a 
profile and product registration

## To build

The code is written in Rust 

If you have an existing Rust environment, you can start the service in debug mode, with pre-populated data, using
```bash
cargo run
```

Alternatively, a dockerfile is provided for both building the service and running the service in a docker container
```bash
# Build Docker image
docker build -t profile_backend .
# Run Docker image
docker run -it --rm -p 3000:3000 profile_backend
```

## Tests
An end to end test suite, written in Python, is included in the `e2e` folder of the repository, to start, try something like

To set up the python environment, run the following
```bash
python3 -m venv venv
source ./venv/bin/activate
pip3 install -r ./e2e/requirements.txt
```

To run the e2e suite
```bash
python3 ./e2e/main.py
```

## Design

## Assumptions made
