# Marbles

A distributed consensus ranked choice voting system.

> [!NOTE]
> This project is in the "idea" phase of development, expect the architecture/goals to shift drastically.

Prediction markets are great for aggregrating probability estimates, but they're epistemically very different from preference aggregation. The goal of this system is to be able easily propose and vote on arbitrary baskets of items to reveal preferences at scale. There are tons of incentive issues and practical problems to solve before this is actually a useable tool, so for now consider this an educational experiment.

Marbles provides a platform for conducting elections for abitrary groups of items across a distrubuted cluster of polling station nodes.
It supports elections from a number of voting families, such as [single-vote plurality](https://en.wikipedia.org/wiki/Ranked_voting), [Condorcet](https://en.wikipedia.org/wiki/Condorcet_method), [Positional](https://en.wikipedia.org/wiki/Positional_voting), and [Cardinal](https://en.wikipedia.org/wiki/Rated_voting) ([STAR](https://en.wikipedia.org/wiki/STAR_votin)) voting. Each polling node acts as a gatekeeper, verifying a user's identity via digital signature and accepts the ballot. Each node maintains a local chain of ballot hashes, broadcasting its hash list periodically to other nodes using [CRDTs](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type). These are stored in a global Merkle Tree, allowing voters to prove that their vote was cast in the final tally, and see its direct impact on the election without sacrificing privacy.

Marbles provides a RESTful API that prospective voters can use to cast their ballots. Eventually I'll setup a frontend for this, but for now the UI is `curl`.
```
curl -X POST http://marbles.local:8080/api/v1/elections/9f4a2c81d7e305b6f1a0/ballots \
  -H "Content-Type: application/json" \
  -d '{
        "method": "rcv",
        "voter": {
          "public_key": "ed25519:GkDzm3C9P2vXqLwT8nYsRfJbKoA4eHuV7MN1tZpQ6cWx",
          "signature": "3Qk7mP9vXnLwT2eYsRfJbKoA4eHuV7MN1tZpQ6cWxGkDzm3C9P2vBqLwT8nYsRfJb"
        },
        "ballot": {
        "choices": [
          { "rank": 1, "candidate_id": "middlemarch" },
          { "rank": 2, "candidate_id": "anna-karenina" },
          { "rank": 3, "candidate_id": "crime-and-punishment" },
          { "rank": 4, "candidate_id": "don-quixote" },
          { "rank": 5, "candidate_id": "moby-dick" }
        ]
      },
      "signed_at": "2025-04-19T14:32:00Z"
   }'
```

Alternatively, you can use the `marbles` cli to cast a ballot.
```
marbles vote 9f4a2c81d7e305b6f1a0 --identity ~/.ssh/id_ed25519.pub --ballot authors.json
```
