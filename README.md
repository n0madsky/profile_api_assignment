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

## Designs (+ Assumptions made in the process)

The code is roughly split into three layers
* Web layer, consisting of the controllers, routes, etc
* Service layer, where most of the logic lies, some tests are included here
* Repository layer, this is meant to abstract the database, but a somewhat correct in memory implementation has been included, with some tests.

In the web layer, as there are potentially hundreds or thousands of profiles or product registrations, simple pagination has been implemented
for the `GET /profiles` and `GET /profiles/:profile/product_registrations` endpoint.
For a more production ready app I'd consider using cursor value based pagination

It is assumed that in the profiles page we do not require product registrations, although I should have clarified if this is the case.

For product update (`POST /product`), to simplify the endpoint, I've made the following assumptions
* There is no overwriting of existing products, this should more or less happen in the actual production service, as we'd want to preserve a snapshot of an old product
an user bought in history. I couldn't be bothered overwriting, so I've made it that submitting a product with an existing SKU an error in the API
* As no API was provided, I've assumed we have a product SKU, a product valid duration (as we have expiration in the registration), as well as what a product bundles
* Submitting non-existent products is assumed to be a mistake, I think this should prevent cycles, but I haven't rigorously tested this.

For getting product registrations, as product children has no further children, so I've made it that we fetch a list of leaf products, and register each leaf product.
i.e. if `ARIE4` contains `ARCC4`, and `ARCC4` contains more, we wouldn't create an `ARCC4` registration, but in the end we'd fetch the end products. 

It is assumed that most of the data is either passed in via JSON or via query variables, as the APIs weren't defined, for simple things I've used query URLs,
for more sophisticated endpoints, such as registrations, I've used JSON input.

The repository layer uses dashmaps (Concurrent HashMap in Rust) and also Mutexes around a List to store data.


## Future improvements

* Better pagination, rather than offsets in db
* Implement row delete in data layer
* Add an actual postgres db
* Add a gRPC server, I couldn't be bothered including it here, but I did once work at Larry and Sergey's protobuf moving service.
