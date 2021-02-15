# Kayvee-rs
Kayvee is an in-memory key-value store written in Rust. It implements three methods: `GET`, `PUT`, and `DELETE` all of which are accessible via it's GraphQL endpoint.

## Methods
+ ### PUT
  ```graphql
    mutation {
      put("name", "Michæl") {
        status
        message
      }
    }
  ```
  ```json
    {
      "data": {
        "put": {
          "status": "success",
          "message": null
        }
      }
    }
  ```

+ ### GET
  ```graphql
    query {
      get("name")
    }
  ```
  ```json
    {
      "data": {
        "get": "Michæl"
      }
    }
  ```

+ ### DELETE
  ```graphql
    mutation {
      delete("name") {
        status
        message
      }
    }
  ```
  ```json
    {
      "data": {
        "delete": {
          "status": "success",
          "message": null
        }
      }
    }
  ```
