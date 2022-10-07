# Nosana Jobs

## Program Information

| Info            | Description                                                                                                                         |
|-----------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Type            | [Solana Program](https://docs.solana.com/developing/intro/programs#on-chain-programs)                                               |
| Source Code     | [GitHub](https://github.com/nosana-ci/nosana-programs)                                                                              |
| Build Status    | [Anchor Verified](https://www.apr.dev/program/nosJhNRqr2bc9g1nfGDcXXTXvYUmxD4cVwy2pMWhrYM)                                          |
| Accounts        | [`3`](#accounts)                                                                                                                    |
| Instructions    | [`9`](#instructions)                                                                                                                |
| Types           | [`4`](#types)                                                                                                                       |
| Domain          | `nosana-jobs.sol`                                                                                                                   |
|  Address        | [`nosJhNRqr2bc9g1nfGDcXXTXvYUmxD4cVwy2pMWhrYM`](https://explorer.solana.com/address/nosJhNRqr2bc9g1nfGDcXXTXvYUmxD4cVwy2pMWhrYM)    |

## Instructions

A number of 9 instruction are defined in the Nosana Jobs program.

To load the program with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
const programId = new PublicKey('nosJhNRqr2bc9g1nfGDcXXTXvYUmxD4cVwy2pMWhrYM');
const idl = await Program.fetchIdl(programId.toString());
const program = new Program(idl, programId);
```

### Open

The `open()` instruction initializes a [MarketAccount](#market-account) and an
associated [VaultAccount](#vault-account) for token deposits.

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `mint`                 | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The token Mint address for this instruction.                                                      |
| `market`               | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The [MarketAccount](#market-account) address.                                                     |
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `job`                  | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [JobAccount](#job-account) address.                                                           |
| `authority`            | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing authority of the program invocation.                                                  |
| `accessKey`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The Node Access Key address.                                                                      |
| `rent`                 | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official Solana rent address. Responsible for lamports.                                       |
| `systemProgram`        | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official Solana system program address. Responsible for system CPIs.                          |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |

#### Arguments

| Name                   | Type              | Size    | Offset  | Description                                               |
|------------------------|-------------------|---------|---------|-----------------------------------------------------------|
| `jobPrice`             | `u64`             | `8`     | `0`     | The price for jobs in this market.                        |
| `jobTimeout`           | `i64`             | `16`    | `8`     | The timeout time in seconds for jobs.                     |
| `jobType`              | `u8`              | `1`     | `24`    | The [JobType](#job-type) number.                          |
| `nodeStakeMinimum`     | `u64`             | `8`     | `25`    | The number of tokens a node needs to stake to qualify.    |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .open(
    jobPrice,          // type: u64
    jobTimeout,        // type: i64
    jobType,           // type: u8
    nodeStakeMinimum,  // type: u64
  )
  .accounts({
    mint,              // 𐄂 writable, 𐄂 signer
    market,            // ✓ writable, ✓ signer
    vault,             // ✓ writable, 𐄂 signer
    job,               // ✓ writable, 𐄂 signer
    authority,         // ✓ writable, ✓ signer
    accessKey,         // 𐄂 writable, 𐄂 signer
    rent,              // 𐄂 writable, 𐄂 signer
    systemProgram,     // 𐄂 writable, 𐄂 signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
  })
  .signers([marketKey, authorityKey])
  .rpc();
```

### Close

The `close()` instruction closes a [MarketAccount](#market-account) and an
associated [VaultAccount](#vault-account).
The vault has to be empty of tokens.

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `market`               | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [MarketAccount](#market-account) address.                                                     |
| `vault`                | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The [VaultAccount](#vault-account) address.                                                       |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .close()
  .accounts({
    market,            // ✓ writable, 𐄂 signer
    vault,             // 𐄂 writable, 𐄂 signer
    authority,         // 𐄂 writable, ✓ signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
  })
  .signers([authorityKey])
  .rpc();
```

### Update

The `update()` instruction update a [MarketAccount](#market-account).

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `market`               | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [MarketAccount](#market-account) address.                                                     |
| `accessKey`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The Node Access Key address.                                                                      |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |

#### Arguments

| Name                   | Type              | Size    | Offset  | Description                                               |
|------------------------|-------------------|---------|---------|-----------------------------------------------------------|
| `jobPrice`             | `u64`             | `8`     | `0`     | The price for jobs in this market.                        |
| `jobTimeout`           | `i64`             | `16`    | `8`     | The timeout time in seconds for jobs.                     |
| `jobType`              | `u8`              | `1`     | `24`    | The [JobType](#job-type) number.                          |
| `nodeStakeMinimum`     | `u64`             | `8`     | `25`    | The number of tokens a node needs to stake to qualify.    |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .update(
    jobPrice,          // type: u64
    jobTimeout,        // type: i64
    jobType,           // type: u8
    nodeStakeMinimum,  // type: u64
  )
  .accounts({
    market,            // ✓ writable, 𐄂 signer
    accessKey,         // 𐄂 writable, 𐄂 signer
    authority,         // 𐄂 writable, ✓ signer
  })
  .signers([authorityKey])
  .rpc();
```

### List

The `list()` instruction lists a job, with its required data.
When there is a node ready in the queue it will immediately start running.
The [JobAccount](#job-account) is optionally created

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `job`                  | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [JobAccount](#job-account) address.                                                           |
| `seed`                 | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | n/a                                                                                               |
| `market`               | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [MarketAccount](#market-account) address.                                                     |
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `user`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The user token account that will debit/credit the tokens.                                         |
| `feePayer`             | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing Fee Payer address.                                                                    |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |
| `rewardsReflection`    | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The Nosana Rewards Program's [ReflectionAccount](/programs/rewards#reflection-account) address.   |
| `rewardsVault`         | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The Nosana Rewards Program's [VaultAccount](/programs/rewards#vault-account) address.             |
| `rewardsProgram`       | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The [Nosana Rewards](/programs/rewards) Program address.                                          |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |
| `systemProgram`        | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official Solana system program address. Responsible for system CPIs.                          |

#### Arguments

| Name                   | Type              | Size    | Offset  | Description                                               |
|------------------------|-------------------|---------|---------|-----------------------------------------------------------|
| `ipfsJob`              | `[u8; 32]`        | `32`    | `0`     | The byte array representing the IPFS hash to the job.     |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .list(
    ipfsJob,           // type: [u8; 32]
  )
  .accounts({
    job,               // ✓ writable, 𐄂 signer
    seed,              // 𐄂 writable, 𐄂 signer
    market,            // ✓ writable, 𐄂 signer
    vault,             // ✓ writable, 𐄂 signer
    user,              // ✓ writable, 𐄂 signer
    feePayer,          // ✓ writable, ✓ signer
    authority,         // 𐄂 writable, ✓ signer
    rewardsReflection, // ✓ writable, 𐄂 signer
    rewardsVault,      // ✓ writable, 𐄂 signer
    rewardsProgram,    // 𐄂 writable, 𐄂 signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
    systemProgram,     // 𐄂 writable, 𐄂 signer
  })
  .signers([feePayerKey, authorityKey])
  .rpc();
```

### Clean

The `clean()` instruction closes an existing [JobAccount](#job-account).
When the job was still queued the tokens will be returned to the user.

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `job`                  | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [JobAccount](#job-account) address.                                                           |
| `market`               | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The [MarketAccount](#market-account) address.                                                     |
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `user`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The user token account that will debit/credit the tokens.                                         |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .clean()
  .accounts({
    job,               // ✓ writable, 𐄂 signer
    market,            // 𐄂 writable, 𐄂 signer
    vault,             // ✓ writable, 𐄂 signer
    user,              // ✓ writable, 𐄂 signer
    authority,         // 𐄂 writable, ✓ signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
  })
  .signers([authorityKey])
  .rpc();
```

### Work

With the `work()` instruction a node enters the [MarketAccount](#market-account) queue.

A few requirements are enforced:

- A node needs to have a minimum stake in Nosana Staking.
- A node needs to hold an official Nosana NFT.
- A node can only enter the queue once

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `job`                  | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [JobAccount](#job-account) address.                                                           |
| `seed`                 | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | n/a                                                                                               |
| `market`               | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [MarketAccount](#market-account) address.                                                     |
| `stake`                | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The [StakeAccount](/programs/staking#stake-account) address.                                      |
| `nft`                  | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The Token Account address that holds the NFT.                                                     |
| `metadata`             | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The Metaplex Metadata address, that belongs to the NFT.                                           |
| `feePayer`             | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing Fee Payer address.                                                                    |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |
| `systemProgram`        | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official Solana system program address. Responsible for system CPIs.                          |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .work()
  .accounts({
    job,               // ✓ writable, 𐄂 signer
    seed,              // 𐄂 writable, 𐄂 signer
    market,            // ✓ writable, 𐄂 signer
    stake,             // 𐄂 writable, 𐄂 signer
    nft,               // 𐄂 writable, 𐄂 signer
    metadata,          // 𐄂 writable, 𐄂 signer
    feePayer,          // ✓ writable, ✓ signer
    authority,         // 𐄂 writable, ✓ signer
    systemProgram,     // 𐄂 writable, 𐄂 signer
  })
  .signers([feePayerKey, authorityKey])
  .rpc();
```

### Stop

With the `stop()` instruction a node exits the node queue
from a [MarketAccount](#market-account).

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `market`               | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [MarketAccount](#market-account) address.                                                     |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .stop()
  .accounts({
    market,            // ✓ writable, 𐄂 signer
    authority,         // 𐄂 writable, ✓ signer
  })
  .signers([authorityKey])
  .rpc();
```

### Finish

With the `finish()` instruction a node can can post the result for a job it has finished,
and be reimbursed for the work.

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `job`                  | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [JobAccount](#job-account) address.                                                           |
| `market`               | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The [MarketAccount](#market-account) address.                                                     |
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `user`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The user token account that will debit/credit the tokens.                                         |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |

#### Arguments

| Name                   | Type              | Size    | Offset  | Description                                               |
|------------------------|-------------------|---------|---------|-----------------------------------------------------------|
| `ipfsResult`           | `[u8; 32]`        | `32`    | `0`     | The byte array representing the IPFS hash to the results. |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .finish(
    ipfsResult,        // type: [u8; 32]
  )
  .accounts({
    job,               // ✓ writable, 𐄂 signer
    market,            // 𐄂 writable, 𐄂 signer
    vault,             // ✓ writable, 𐄂 signer
    user,              // ✓ writable, 𐄂 signer
    authority,         // 𐄂 writable, ✓ signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
  })
  .signers([authorityKey])
  .rpc();
```

### Quit

With the `quit()` instruction a node can quit a job that it has started.

#### Accounts

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `job`                  | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [JobAccount](#job-account) address.                                                           |
| `authority`            | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="#3EAF7C" />     | The signing authority of the program invocation.                                                  |

#### Example

To run the instructions with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .quit()
  .accounts({
    job,               // ✓ writable, 𐄂 signer
    authority,         // 𐄂 writable, ✓ signer
  })
  .signers([authorityKey])
  .rpc();
```

## Accounts

A number of 3 accounts make up for the Nosana Jobs Program's state.

### Market Account

The `MarketAccount` struct holds all the information about jobs and the nodes queue.

| Name                        | Type                        | Size    | Offset  | Description                                                                                       |
|-----------------------------|-----------------------------|---------|---------|---------------------------------------------------------------------------------------------------|
| `authority`                 | `publicKey`                 | `32`    | `8`     | The signing authority of the program invocation.                                                  |
| `jobPrice`                  | `u64`                       | `8`     | `40`    | The price for jobs in this market.                                                                |
| `jobTimeout`                | `i64`                       | `16`    | `48`    | The timeout time in seconds for jobs.                                                             |
| `jobType`                   | `u8`                        | `1`     | `64`    | The [JobType](#job-type) number.                                                                  |
| `vault`                     | `publicKey`                 | `32`    | `65`    | The [VaultAccount](#vault-account) address.                                                       |
| `vaultBump`                 | `u8`                        | `1`     | `97`    | The bump for the [VaultAccount](#vault-account).                                                  |
| `nodeAccessKey`             | `publicKey`                 | `32`    | `98`    | n/a                                                                                               |
| `nodeStakeMinimum`          | `u64`                       | `8`     | `130`   | The number of tokens a node needs to stake to qualify.                                            |
| `queueType`                 | `u8`                        | `1`     | `138`   | n/a                                                                                               |
| `queue`                     | `Vec<undefined>`            | `undefined`| `139`   | n/a                                                                                               |

### Job Account

The `JobAccount` struct holds all the information about any individual jobs.

| Name                        | Type                        | Size    | Offset  | Description                                                                                       |
|-----------------------------|-----------------------------|---------|---------|---------------------------------------------------------------------------------------------------|
| `authority`                 | `publicKey`                 | `32`    | `8`     | The signing authority of the program invocation.                                                  |
| `ipfsJob`                   | `[u8; 32]`                  | `32`    | `40`    | The byte array representing the IPFS hash to the job.                                             |
| `ipfsResult`                | `[u8; 32]`                  | `32`    | `72`    | The byte array representing the IPFS hash to the results.                                         |
| `market`                    | `publicKey`                 | `32`    | `104`   | The [MarketAccount](#market-account) address.                                                     |
| `node`                      | `publicKey`                 | `32`    | `136`   | n/a                                                                                               |
| `price`                     | `u64`                       | `8`     | `168`   | n/a                                                                                               |
| `status`                    | `u8`                        | `1`     | `176`   | n/a                                                                                               |
| `timeEnd`                   | `i64`                       | `16`    | `177`   | n/a                                                                                               |
| `timeStart`                 | `i64`                       | `16`    | `193`   | n/a                                                                                               |

### Vault Account

The `VaultAccount` is a regular Solana Token Account.

## Types

A number of 4 type variants are defined in the Nosana Jobs Program's state.

### Order

The `Order` struct is type used to describe orders in the market.

A number of 3 variants are defined in this `struct`:
| Name                                  | Type                                  |
|---------------------------------------|---------------------------------------|
| `user`                                | `publicKey`                           |
| `ipfsJob`                             | `[object Object]`                     |
| `jobPrice`                            | `u64`                                 |

### Queue Type

The `QueueType` describes the type of queue

A number of 3 variants are defined in this `enum`:
| Name                                  | Number                                |
|---------------------------------------|---------------------------------------|
| `Job`                                 | `0`                                   |
| `Node`                                | `1`                                   |
| `Unknown`                             | `255`                                 |

### Job Status

The `JobStatus` describes the status of any job

A number of 3 variants are defined in this `enum`:
| Name                                  | Number                                |
|---------------------------------------|---------------------------------------|
| `Queued`                              | `0`                                   |
| `Running`                             | `1`                                   |
| `Done`                                | `2`                                   |

### Job Type

The `JobType` describes the type of any job.

A number of 6 variants are defined in this `enum`:
| Name                                  | Number                                |
|---------------------------------------|---------------------------------------|
| `Default`                             | `0`                                   |
| `Small`                               | `1`                                   |
| `Medium`                              | `2`                                   |
| `Large`                               | `3`                                   |
| `Gpu`                                 | `4`                                   |
| `Unknown`                             | `255`                                 |
