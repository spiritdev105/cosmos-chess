# **cosmos-chess**

**cosmos-chess** is a CosmWasm smart contract built for playing chess games on the blockchain. It enables users to create challenges, accept or cancel them, play chess games, and view player ratings, all managed through smart contract logic.

## **Overview**

The contract supports various interactions, enabling users to:
- **Create Challenges:** Initiate a chess challenge, specifying parameters such as preferred color (black, white, or random) and whether the challenge is open to a specific opponent or any player.
- **Accept or Cancel Challenges:** Players can accept open challenges or cancel their own challenges if they no longer wish to play.
- **Play Chess Games:** Once a challenge is accepted, players can make moves, offer or accept a draw, or resign. The game is turn-based, with each player's move recorded on the blockchain.
- **Game Time Limit:** Optionally, challenges can include a per-player block time limit, functioning as a chess clock to add a layer of strategy.

### **Query Methods**
- **Challenges and Games:** Users can query the contract to retrieve summaries or detailed information about challenges and games.
- **Paging and Filtering:** Summary queries are designed to be concise to manage result sizes effectively. They also support an "after" parameter, allowing users to page through results efficiently.
- **View Ratings:** Players can view the ratings of other participants, which reflect their skill and performance in past games.

## **Development Notes**

- **Data Storage:**
  - The contract uses `cw-storage-plus` to manage data, specifically utilizing indexed maps to store challenges and games.
  - Player addresses are indexed to optimize queries related to a player's games or challenges.

- **Optimization of Storage and Gas Usage:**
  - The contract uses a Forsyth-Edwards Notation (FEN) string to store and load the board state directly. This method is more efficient, keeping gas usage under 300,000 per move, even in long games.

## **Local Testing**

To test the contract locally, follow these steps:

1. **Build the WASM Binary:**
   - Compile the contract to WebAssembly:
     ```bash
     cargo wasm
     ```

2. **Deploy to a Local Node:**
   - Use the provided script to deploy the contract to your local **Juno** blockchain:
     ```bash
     scripts/local_deploy.sh | tee junod_env.sh
     source junod_env.sh
     ```

3. **Interact with the Contract:**
   - After deploying the contract, you can start testing it by executing commands:
     ```bash
     junod_execute '{"create_challenge": {"play_as": "white"}}' --from test-user
     junod_execute '{"accept_challenge": {"challenge_id": 1}}' --from test-user2
     junod_query '{"get_games":{}}'
     ```
   - These commands create a new challenge, accept the challenge, and query the list of games, respectively.
   - Execute an end-to-end test with a real [chess challenge](https://en.wikipedia.org/wiki/World_Chess_Championship_1972#Game_6:_Fischer%E2%80%93Spassky,_1%E2%80%930_(QGD_Tartakower)).
      ```bash
      scripts/local_game.sh "-- contract address --"
      ```

## **UI**

The front-end for interacting with the cosmos-chess contract is developed as a separate project named **cosmos-chess-ui**.

## **Deployment to Mainnet**

### **Setting Up for Deployment**

1. **Run Juno docker image:**
   - Use `junod` version 2.1.0 within a Docker container to ensure compatibility:
     ```bash
     docker run --rm -it --platform linux/amd64 -v "$(pwd)/artifacts:/artifacts:ro" ghcr.io/cosmoscontracts/juno:v2.1.0 /bin/sh
     ```

2. **Configure the Environment:**
   - Set up environment variables for deployment:
     ```bash
     RPC="https://juno-rpc.stakeandrelax.net"
     CHAIN_ID="juno-1"
     GAS_PRICES="0.025ujuno"
     export NODE="${RPC}"
     export TXFLAG="--node ${NODE} --chain-id ${CHAIN_ID} --gas-prices ${GAS_PRICES} --gas auto --gas-adjustment 1.3"
     ```

3. **Set Up the Chess Key:**
   - Create a `chess` key or recover an existing one with sufficient JUNO for deployment:
     ```bash
     junod keys add chess --recover
     # Verify the balance
     junod query bank balances "addr"
     ```
   - Ensure you have at least 1 JUNO for deployment costs. It is recommended to have around 3 JUNO to cover potential costs.

### **Deploying the Smart Contract**

1. **Store the Wasm Binary:**
   - Upload the compiled WASM binary to the blockchain:
     ```bash
     junod tx wasm store /artifacts/cosmos_chess.wasm --from chess $TXFLAG
     ```
   - Record the `code_id` and transaction details (e.g., 0.934757 JUNO spent, code_id `165`).

2. **Instantiate the Contract:**
   - Instantiate the contract on the network:
     ```bash
     junod tx wasm instantiate 165 '{}' --from chess --label "cosmoschess 0.0.1" $TXFLAG --admin juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
     ```