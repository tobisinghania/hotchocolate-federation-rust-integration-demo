### Rust `async-graphql` with HotChocolate schema federation

Sample app for integrating an `async-graphql` rust service
with the HotChocolate federation gateway.


Run the HotChocolate services
`accounts`, `inventory`, `products` and `gateway`.

For the `reviews` service an equivalent implementation in Rust and C# exists.
Either one can be started to be used by the federation gateway.


Open `http://localhost:5000/graphql` in your browser and also set `http://localhost:5000/graphql` as
your schema endpoint.

