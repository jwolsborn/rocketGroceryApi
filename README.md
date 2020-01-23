# Rocket grocery list api

## Build and running the application

Since I the rocket crate it is important to run this application on the nightly build of rust. This application was built and
running on rust-1.42.0-nightly 2020-01-18

**rustup default nightly-2020-01-18** will download, and build, on the nightly version

**cargo build** to build the application

**cargo run** to run the application

The server operates on https://localhost:8000

To add an item make a POST request to https:localhost:8000/add/:item to insert an item into the grocery list

To remove an item make a PUT request to https://localhost:8000/remove/:item to remove an item from the grocery list

To retrieve a list of items in the grocery list make a GET request to https://localhost:8000/list


## Notes

Due to the removal of the actix-web crate from github I decided it was best to rebuild my previous grocery api using the rocket
crate.  The downside to this is that it runs on nightly, giving me a possible lack of stability when working with the application.
Since this application is being used as a teaching exercise the list data is kept in memory, thus it wont persist once the
application is terminated.  To ensure the data was safe I used a ```Mutex``` lock to keep the data thread safe in the event
that two routes were called at the same time.  I implemented my own custom ```ApiResponse``` struct that to handle all my
http responses.  I found the rocket crate to be far more intuitive than the actix-web crate as rocket provided mutliple macros
for easily crafting respones as well as easily crafting a custom response due to the ```Responder``` impl.  This allowed for 
much cleaner and simpler error handling.
