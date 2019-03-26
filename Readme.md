# Demo subscriber

This repository hosts a demo implementation of a `tokio-trace` subscriber.

It is intended to demonstrate a very basic subscriber implementation and how to extract the necessary information for logging purposes.

It is **not** intended to be efficient or comprehensive.

Things it **doesn't** do (but might in the future):
 - Use Metadata other than the spans name
 - Respect the environments log settings
 - Correctly lock stdout
 - Neither reuses spans or checks for overflow