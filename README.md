# seaography-mysql-example

## Things what I want to do

Find out how the following code is generated.
https://github.com/SeaQL/seaography/tree/4703b92e09ee9467c0b06ca87f0ed41e67560591/examples/mysql

## How to do

### 1. Set up database

```
# examples/mysql/sakila-data.sql
curl -sL https://raw.githubusercontent.com/SeaQL/seaography/refs/heads/main/examples/mysql/sakila-schema.sql | docker compose exec -T db mysql -h localhost -u root --password=password

# examples/mysql/sakila-schema.sql
curl -sL https://raw.githubusercontent.com/SeaQL/seaography/refs/heads/main/examples/mysql/sakila-data.sql | docker compose exec -T db mysql -h localhost -u root --password=password
```

### 2. Call `sea-orm-cli generate entity`

```
sea-orm-cli generate entity -o src/entities -u mysql://user:pwd@db/sakila --seaography
```


### 3. Call `seaography-cli`

```
seaography-cli ./ src/entities mysql://user:pwd@db/sakila seaography-mysql-example
```

### 4. cargo run

```
URL=0.0.0.0:8000 cargo run
```
