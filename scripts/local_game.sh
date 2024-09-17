#! /bin/bash

# scripts/local_deploy.sh | tee juno_env.sh
# source junod_env.sh
# junod_execute '{"create_challenge": {"play_as": "white"}}' --from test-user
# junod_execute '{"accept_challenge": {"challenge_id": 1}}' --from test-user2
# junod_query '{"get_games":{}}'

CONTRACT=$1
GAME_ID=1
PLAYER1="juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"
PLAYER2="juno102fjg5u62qkgsux9z9fl652mw8r98kgcgjv99m"

junod_execute () {
	MESSAGE=$1
	shift
	docker exec -i junod_local junod tx wasm execute "${CONTRACT}" "${MESSAGE}" --gas auto --gas-adjustment 1.3 --gas-prices 0.01ujunox -y --chain-id testing --output json "${@}"
}

junod_query() {
  MESSAGE=$1;
  shift;
  docker exec -i junod_local junod query wasm contract-state smart "juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8" "${MESSAGE}" --chain-id testing --output json;
}

((MOVE_COUNT=0));

domove() {
  ((MOVE_COUNT=MOVE_COUNT+1))
  echo
  echo "${MOVE_COUNT} ${2}"
  junod_execute "{\"turn\": {\"game_id\": ${GAME_ID}, \"action\": {\"move\": \"$2\"}}}" --from "${1}"
  # let block execute
  sleep 7
}

# https://en.wikipedia.org/wiki/World_Chess_Championship_1972#Game_6:_Fischer%E2%80%93Spassky,_1%E2%80%930_(QGD_Tartakower)
domove "${PLAYER1}" c4
domove "${PLAYER2}" e6
domove "${PLAYER1}" Nf3
domove "${PLAYER2}" d5
domove "${PLAYER1}" d4
domove "${PLAYER2}" Nf6
domove "${PLAYER1}" Nc3
domove "${PLAYER2}" Be7
domove "${PLAYER1}" Bg5
domove "${PLAYER2}" 0-0
domove "${PLAYER1}" e3
domove "${PLAYER2}" h6
domove "${PLAYER1}" Bh4
domove "${PLAYER2}" b6
domove "${PLAYER1}" cxd5
domove "${PLAYER2}" Nxd5
domove "${PLAYER1}" Bxe7
domove "${PLAYER2}" Qxe7
domove "${PLAYER1}" Nxd5
domove "${PLAYER2}" exd5
domove "${PLAYER1}" Rc1
domove "${PLAYER2}" Be6
domove "${PLAYER1}" Qa4
domove "${PLAYER2}" c5
domove "${PLAYER1}" Qa3
domove "${PLAYER2}" Rc8
domove "${PLAYER1}" Bb5
domove "${PLAYER2}" a6
domove "${PLAYER1}" dxc5
domove "${PLAYER2}" bxc5
domove "${PLAYER1}" 0-0
domove "${PLAYER2}" Ra7
domove "${PLAYER1}" Be2
domove "${PLAYER2}" Nd7
domove "${PLAYER1}" Nd4
domove "${PLAYER2}" Qf8
domove "${PLAYER1}" Nxe6
domove "${PLAYER2}" fxe6
domove "${PLAYER1}" e4
domove "${PLAYER2}" d4
domove "${PLAYER1}" f4
domove "${PLAYER2}" Qe7
domove "${PLAYER1}" e5
domove "${PLAYER2}" Rb8
domove "${PLAYER1}" Bc4
domove "${PLAYER2}" Kh8
domove "${PLAYER1}" Qh3
domove "${PLAYER2}" Nf8
domove "${PLAYER1}" b3
domove "${PLAYER2}" a5
domove "${PLAYER1}" f5
domove "${PLAYER2}" exf5
domove "${PLAYER1}" Rxf5
domove "${PLAYER2}" Nh7
domove "${PLAYER1}" Rcf1
domove "${PLAYER2}" Qd8
domove "${PLAYER1}" Qg3
domove "${PLAYER2}" Re7
domove "${PLAYER1}" h4
domove "${PLAYER2}" Rbb7
domove "${PLAYER1}" e6
domove "${PLAYER2}" Rbc7
domove "${PLAYER1}" Qe5
domove "${PLAYER2}" Qe8
domove "${PLAYER1}" a4
domove "${PLAYER2}" Qd8
domove "${PLAYER1}" R1f2
domove "${PLAYER2}" Qe8
domove "${PLAYER1}" R2f3
domove "${PLAYER2}" Qd8
domove "${PLAYER1}" Bd3
domove "${PLAYER2}" Qe8
domove "${PLAYER1}" Qe4
domove "${PLAYER2}" Nf6
domove "${PLAYER1}" Rxf6
domove "${PLAYER2}" gxf6
domove "${PLAYER1}" Rxf6
domove "${PLAYER2}" Kg8
domove "${PLAYER1}" Bc4
domove "${PLAYER2}" Kh8
domove "${PLAYER1}" Qf4

junod_execute "{\"turn\": {\"game_id\": ${GAME_ID}, \"action\": \"resign\"}}" --from "${PLAYER2}"

sleep 7

junod_query '{"get_ratings":{}}'
