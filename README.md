## Por Hacer

This simple app was created to try out rust and get comfortable with all its features

# Usage 

Install diesel_cli and do: 
- `diesel setup` -> that will create the file `test.db`
- `diesel migration run`

After that run the application: 
`cargo run `

and then you will be able to create a new todo

`curl localhost:8083/todo -d '{"title":"Practicar mate", "content": "Practique sus tareas de mate"}'`

The request will return the uuid of the created todo which can then be queried with

`curl localhost:8083/todo/<insert-new-uuid-here>`

or you can just query all todos

`curl localhost:8083/todos`

# Missing

This list is way too long to include everything, but on the close roadmap (no priority) we have:

- return+accept protobuf in the api
- authentication for different users
- improve error handler to tide
- add "present" to the db to mark if a task needs to be presented to be considered done (some leftovers are already in the api for put)
- use temp dir for database file
