/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

//  for less verbose test output. use TRYORAMA_LOG_LEVEL=error hc test -s

const path = require('path')
const {  Orchestrator, Config, combine, localOnly, tapeExecutor } = require('@holochain/tryorama')

const dnaPath = path.join(__dirname, "../dist/example-dna.dna.json")

// Instatiate a test orchestrator.
// It comes loaded with a lot default behavior which can be overridden, including:
const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,
  )
})

const dna = Config.dna(dnaPath, 'example_dna')
const conductorConfig = Config.gen(
  {
    example_dna: dna
  },
  {
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000',
    },
  })

require('./profiles')(orchestrator.registerScenario, conductorConfig)

orchestrator.run()
