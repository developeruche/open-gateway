import { runTypeChain, glob } from 'typechain'

async function main() {
  const cwd = process.cwd()
  // find all files matching the glob
  const allFiles = glob(cwd, [
    `!./out/!(build-info)/**/*.dbg.json`,
    `./out/!(build-info)/**/+([a-zA-Z0-9_]).json`,
  ])

  const result = await runTypeChain({
    cwd,
    filesToProcess: allFiles,
    allFiles,
    outDir: '../sdks/ts-sdk/src/opengateway-contract-types',
    target: 'ethers-v6',
  })

  console.log(result)
  console.log('DONE!')
}

main().catch(console.error)
