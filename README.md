What this is
============

This is an experimental use of nickel with a mongo database to test my ability to use Rust.

The final product will require you to visit http://localhost:6767 at this point. You will be able to see a small amount of text showing which db, collection, and values per entry you are looking at.

I allow for a small interface with the db as well.

SETUP
=====

I have some manual steps for setting up the DB unfortunately.

1. You need to clone the following repo for the db. I recommend cloning into another directory.
   `git clone --recursive https://github.com/pedroamador/ubuntu1404-mongodb26`
2. Inside the `ubuntu1404-mongodb26` directory, I recommend changing the IP address to `192.168.33.10` since that address is hard coded in.
3. You need to `vagrant ssh` and then edit `/etc/mongod.conf` so that `bind_ip = 192.168.33.10`.

Once that is done, you should be able to connect with the db via the app.

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
