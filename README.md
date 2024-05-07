This contract will make any CW404s able to handle reroll, by leveraging the basic functionality of the CW404 technology.

Key Features:
The contract interaction with users is built in 2 messages:
1) The first message will accept the set amount of fees requested by the instantiator of the contract, and create a pending reroll request inside the contract.
2) The second message will listen to a receive NFT, which will be a CW404 NFTs and, upon request validated, will send back 1 CW404 Token, resulting in the burn of the incoming NFT and a mint of a brand new NFT.

There are also messages to retrieve the fees money, update the config of the contract and other useful queries to know the contract state.
