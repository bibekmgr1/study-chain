# StudyChain

A Solana program that lets students commit to study goals on-chain.

## Problem

Students struggle to stay accountable. There's no verifiable proof of commitment.

## Solution

StudyChain lets anyone:
1. Create a study commitment with a deadline
2. Complete it before the deadline
3. Anyone can mark it as **failed** after the deadline passes — the blockchain clock is the arbiter, not self-reporting

## Why Solana

The deadline is enforced on-chain. No oracle, no trusted third party. `fail_commitment` is permissionless — anyone can call it after the deadline, and the program checks `Clock::get()` to verify.

## Live Program

- **Network**: Devnet
- **Program ID**: `3R3RyYdY3yZJbSPi4pduJc7wabLEhrPE7Y14JYSusZug`
- **Explorer**: https://explorer.solana.com/address/3R3RyYdY3yZJbSPi4pduJc7wabLEhrPE7Y14JYSusZug?cluster=devnet

## Instructions

### `create_commitment(commitment: String, deadline: i64)`
Creates a new `StudyRecord` with the student's pubkey, commitment text, deadline, `completed = false`, `failed = false`.

### `complete_commitment()`
Marks the commitment as completed. Fails if already marked failed.

### `fail_commitment()`
Permissionless — anyone can call it after the deadline has passed. Fails if already completed or if the deadline hasn't passed yet.

## Test
Running client...
  client.ts:

Running tests...
  anchor.test.ts:
  StudyChain Test
     Created commitment: FnRWjupG2GZvm9ss53K9JmD6TCm446D1WKhEqv1ScZxX
     Completed commitment
     Created commitment with past deadline
$      Failed commitment (permissionless)
     All tests passed
    ✔  Creates, completes, and fails commitments (4003ms)
  1 passing (4s)
$ 

Team
Bibek Thapa Magar (Nepal) — BBS student learning Rust and Solana

## Built For

Colosseum Crypto World's Fair Hackathon — Superteam Nepal Track

## Roadmap

**v1 (current):** Permissionless failure enforced by on-chain clock.

**v2 (next):** Stake SOL on commitments. Lock it in a PDA. Complete the commitment, get it back. Fail it, and the stake goes to a designated recipient (friend, charity, or burn address). This makes StudyChain a true commitment device with real skin in the game — something only possible on-chain.
