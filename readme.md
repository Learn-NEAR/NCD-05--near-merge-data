<div align="center">
  <h1><code>near-merge-data</code></h1>
  <p>
    <strong>Implementation payable check data (imitation via sort) on NEAR Protocol</strong>
  </p>
</div>

## Feature

1. Input parameters — arrays, at least 2
2. Sorting options for arrays: ascending (`true`), descending (`false`), do not sort (`undefined`)
3. Make sure that all arrays are not empty — otherwise throw an error
4. Check that attached deposit is not zero — otherwise it throws an error
5. Sort each array separately — depending on the input sort parameter
6. Merge arrays according to their order of passing in function parameters
7. Transfer funds in the amount of 1 yocto per array to the one who called the contract (predecessor)
8. Return the resulting array
9. Test coverage (unit, integration)

## Usage

```shell
accountId=ilyar.testnet
contractName="merge-data.ilyar.testnet"
near --accountId $accountId --amount 0.25 call $contractName merge '{"data": [[102, 111], [98, 97, 114]]}'
ear --accountId $accountId --amount 0.25 call $contractName merge '{"data": [[102, 111], [98, 97, 114]], "sort": true}'
ear --accountId $accountId --amount 0.25 call $contractName merge '{"data": [[102, 111], [98, 97, 114]], "sort": false}'
```

## Develop

```shell
make fix 
make qa
make build
make clean
```

### Run CI local

Installation [act](https://github.com/nektos/act):
```shell
brew install act
```

Setup env vars:
```shell
echo "GITHUB_TOKEN=%GITHUB_TOKEN%" | tee .secrets
```

Run
```shell
act --help
```

## Deploy test

```shell
accountId=ilyar.testnet
make build
near dev-deploy
contractName=$(cat neardev/dev-account)
near state $contractName
near delete $contractName $accountId
near clean
```

## Deploy test

```shell
accountId=ilyar.testnet
make build
near dev-deploy
contractName=$(cat neardev/dev-account)
near state $contractName
near delete $contractName $accountId
near clean
```

## Deploy test

```shell
make qa
make build
export NEAR_ENV=testnet
accountId=ilyar.testnet
contractName="merge-data.$accountId"
near create-account --masterAccount $accountId $contractName 
near deploy $contractName
contractName=$(cat neardev/dev-account)
near state $contractName
```
