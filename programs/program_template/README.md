# Program example

This is an example program that transfer funds from one account to another account and vise versa, and allows authorized address to update the config of both forwarders.

## Program parameters

- owner - The address that is the owner of the program
- denom - The denom we are using in our transfers
- max_first_forward_amount - Maximum amount allowed to be transfered in a single message on first forwarder
- max_second_forward_amount - Maximum amount allowed to be transfered in a single message on second forwarder
- authorized_addr - Authorized address that can change config of the forwaders

## Accounts

We have 2 accounts that funds can be transfered between them:
- First account
- Second account

## Libraries

We have 2 forwarder libraries that enable transferring funds from first account to the second account:

- First forwarder
- Second forwarder

## Authorizations

We have 4 authorizations:

- Forward from first to second - Forward funds from first account to second account
- Forward from second to first - Forward funds from second account to first account
- Secure update first forwarder config - Authorized update config for the first forwarder
- Secure uodate second forwarder config - Authorized update config for the second forwarder