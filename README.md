## Por Hacer

This simple app was created to try out rust and get comfortable with all its features

# Usage 

`cargo run `
and then 
`curl localhost:8083/todo -d '{"title":"Practicar mate", "content": "Practique sus tareas de mate","present":true }'`

The request will return the uuid of the created todo which can then be queried with

`curl localhost:8083/todo?uuid=<insert-new-uuid-here>`

# Missing

This list is way to long to include everything, but on the close roadmap (no priority) we have:

- change from refinery to diesel
- use r2d2 for connection pooling
- return+accept json/protobuf in the api
- Nice parametrization with config file or command parameters (http port etc.)
- authentication for different users
- add error handler to tide
- add "present" to the db to mark if a task needs to be presented to be considered done (some leftovers are already in the api for put)
- use temp dir for database file
