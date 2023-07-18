'use strict'

module.exports = async function (fastify, opts) {
  fastify.get('/:delay', async function (request, reply) {
    // console.log("yuh")
    let delay = request.params.delay
    await new Promise(r => setTimeout(r, delay));
    reply.send(delay)
  })
}
