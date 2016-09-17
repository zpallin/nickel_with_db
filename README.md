What this is
============

This is an experimental use of nickel with a mongo database to test my ability to use Rust.

The final product will require you to visit http://localhost:6767 at this point. You will be able to see a small amount of text showing which db, collection, and values per entry you are looking at.

I allow for a small interface with the db as well.

SETUP
=====

DB
--

I am using a mongodb Dockerfile. To use it, it's pretty simple.

1. Install docker on your machine https://docs.docker.com/engine/installation/
2. run `docker run -dit -p 27017:27017 --name nwdb_mongo_ctr zpallin/nwdb_mongo:latest` to start the container
 * https://hub.docker.com/\_/mongo/
4. To connect to the db, run `mongo localhost:27017`

Running the App
===============

Three commands

1. `cargo run -- build <dbname> <collectionname>`
  Builds the db and collection in mongo

2. `cargo run -- add <dbname> <collectionnanme> <value>`
  Adds a simple BSON value in mongodb

3. `cargo run -- use <dbname> <collectionname>`
  Doesn't change the db, just runs the app. Visit localhost:6767 to view the app. It spits out the db data very... uh... inelegantly

If you enter these wrong, it should error out.
