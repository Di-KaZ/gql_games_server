# graphiql UI
`http://localhost:8080/graphiql`


# Build docker image

docker build -t game_gql_server .


# Run docker image
docker run -it --rm --name game_gql_server -p 8080:8080 game_gql_server
