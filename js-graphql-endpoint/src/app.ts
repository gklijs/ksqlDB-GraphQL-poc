import { connect } from 'http2'
import { ApolloServer, addResolveFunctionsToSchema } from 'apollo-server'
import { generateGraphQL } from '@confluentinc/ksqldb-graphql'

const session = connect('http://localhost:8088')
const options = {
  hostname: 'localhost',
  port: 8088
}

generateGraphQL({ options }).then(
  ({ schemas, queryResolvers, subscriptionResolvers, mutationResolvers }) => {
    const server = new ApolloServer({
      context: async () => ({
        ksqlDB: {
          options,
          session
        }
      }),
      schema: addResolveFunctionsToSchema({
        schema: schemas,
        resolvers: {
          Subscription: subscriptionResolvers,
          Query: queryResolvers,
          Mutation: mutationResolvers
        }
      }),
      subscriptions: {
        keepAlive: 1000
      }
    })
    server.listen()
  }
)
