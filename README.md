# NoteApp_Note_Command_Service

This microservice was developed with Rust and using CQRS DDD Pattern. 

## Install docker of eventstore and kafka
```console
make generate-all
```

## Run microservice
```console
make run
```

## TODO
- [x] Settings 
- [x] Web Server with warp
- [x] Eventstoredb 
- [x] Kafka
- [x] Note Create event
- [x] Note Parent Change event
- [x] Note Basic Info Change event
- [ ] File upload event
- [ ] Image upload event
- [ ] File Delete event
- [ ] Image delete event
- [ ] Note Delete event
- [ ] Healthcheck
- [ ] Api version mechanism
- [ ] Docker file

## HTTP-API
### Create Note
```console
curl --location --request POST 'http://localhost:8080/add-note' \
--header 'Content-Type: application/json' \
--header 'user-id: 7faaed09-331d-40e1-8048-a122b98ea9af' \
--data-raw '
{
    "commandId": "ef256002-8b46-4ee3-bd6c-9e8acd791c6a",
    "commandTimestamp": 1617472845,
    "commandCreatorId":"7faaed09-331d-40e1-8048-a122b98ea9af",
    "commandCreatorAppType":2,
    "id":"2e219229-ff2f-42db-9887-d7f18984bf2f",
    "pid":"",
    "userId":"7faaed09-331d-40e1-8048-a122b98ea9af",
    "createDate":1617472845,
    "text":"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry'\''s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
    "title":"What is Lorem Ipsum?",
    "image":"",
    "file":""
}'
```