# Architecure

presenting layer <-[presentation models]- business logic

business logic <-[data models]- data base

business logic <-[input events]- input layer

- Models : presentation, data
- Logic: business, presentation, query
- View: console text


entry point : `main()`;

Data/Events flow:
synchronous input -> console input layer -[actions]-> business layer
business layer -[requests]-> data layer

## Layers responsabilities

### Input layer
- receive input from user
