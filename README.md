# Disclaimer!!!

We will create a NEAR APP but using a template instead of building from the ground up. This is the new version, the old version is built using `npx create-near-app@latest` commands.

All credits go to [Mr. EamonDang](https://github.com/eamondang), Mr. EamonDang's original Github repository for this project is located [here](https://github.com/eamondang/started-near-app/) and the video instruction for his project is located [here](https://youtu.be/E8UQnfDC-pA). Follow the instructions in the video or the instructions in the repository's README file to build the project.

After you've cloned from Mr. EamonDang repository and before you build his project. If you are on Windows Subsystem for Linux, you **must** perform these steps:
- Go to Makefile.toml and change all `zsh` to `bash`.
- Install `pnpm` for admin by typing `sudo npm i pnpm`.

After performing all of the above steps, you can build and run Mr. EamonDang's project.

# My version of NEAR APP using Mr. EamonDang's project

## Requirements

This project was run and tested on WSL1, so I highly advise you to do the same. 

You **must** install Node.js and Rust. I highly recommend Node@v16 and Rust@v1.69 for stability.

You **must** install `pnpm` at **admin level** by performing the command `sudo npm i pnpm`. Check if `pnpm` is installed correctly by performing the command `npm list -g` if `pnpm` is listed then you have installed `pnpm` correctly. 

You **must** install Cargo Make by performing the command `cargo install cargo-make`.

You **must** install `near-cli` by performing the command `npm install -g near-cli`.

## Step by Step

- You go to the project's directory
`cd placeholder`
- Install the packages
`cargo make install`
- Prepare
`cargo make prepare`
- Build Contract
`cargo make build`
- Deploy Contract
`cargo make dev-deploy`
- Start Client
`cargo make run`

You have built, deployed and run your NEAR APP. You can access `localhost:3000` for the website.

## Commands

If you don't want to interact with the contract by using the website, you can perform using the commands that I have listed here.

##

- View current contract information
`cargo make view get_current_contract`
> Output: 'Counter: 0 - Message: Default message'
- View the contract counter
`cargo make view get_number`
> Output: 0
- View the contract message
`cargo make view get_message`
> Output: 'Default message'

##

- Increase the counter
`cargo make call plus '{"number": 50}'`
- Decrease the number
`cargo make call subtract '{"number": 10}'`
- Change the message
`cargo make call message '{"message": "hello world, near app!"}'`
- Revert the number and message to default
`cargo make call default`

If the commands are performed successfully, you will receive **a website link which is the receipt for the transaction**.

##

## Notice

When you deploy the NEAR APP successfully, the NEAR APP will create a dev account called `dev-...` while also creates a contract that is bound to the dev account, you will also receive a link which is the receipt for this transaction. All of the near commands can be used with this `dev-...` account.

Use the command `cargo make clean` to remove the env file, neardev file which contains your `dev-...` account, which basically wipe your contract clean. This is useful when you want to push it to your repository because the project when fully loaded is about 1GB, it is also useful when you want to create a new contract/account.

When a contract is deployed, or a contract is bound to a dev account, all of its properties are locked, the contract can only be renewed but cannot be updated. 
- If you add more functions within the properties range of the contract, you can just do `cargo make prepare`, `cargo make build`, `cargo make dev-deploy`, `cargo make run` to renew the contract (it is the same contract - its properties are not changed, but with some added functions). 
- But if you try to change the properties of the contract itself then run the same above commands, you will get errors. So, if you want to change the properties of the contracts (i.e. add the message property), you should use `cargo make clean` first then perform the **Step by Step** guide.

Each contract itself is its own entity. Once that entity is deployed, it is locked or immutable. You can only add more functions to it then deploy on the same contract (renew). But if you want to change the contract's properties then you should abandon the old contract and create a new one.