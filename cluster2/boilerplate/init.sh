#!/bin/bash

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit
fi

PROJ_NAME=$1

anchor init $PROJ_NAME
cd $PROJ_NAME
yarn install
anchor build

#solana-test-validator &
#anchor keys list
#anchor test --skip-local-validator
