<script setup>
import { ref } from "vue";
import Web3 from "web3";
import abi from "./ABI.json";

const web3 = ref(null);
const address = ref(null);
const bankContract = ref(null);
const myDeposit = ref(null);

const depositAmount = ref(0);
const withdrawAmount = ref(0);
const transferAmount = ref(0);
const transferAddress = ref("");

const connectWallet = async () => {
  // get wallet account
  const accounts = await window.ethereum.request({
    method: "eth_requestAccounts",
  });
  address.value = accounts[0];

  // connect to web3
  web3.value = new Web3(window.ethereum);

  // get contract
  bankContract.value = new web3.value.eth.Contract(
    abi,
    ""
  );
};

const getMyDeposit = async () => {
  myDeposit.value = await bankContract.value.methods.myBalance().call({
    from: address.value,
  });
};

const deposit = async () => {
  await bankContract.value.methods.deposit(depositAmount.value).send({
    from: address.value,
  });
};

const withdraw = async () => {
  await bankContract.value.methods.withdraw(withdrawAmount.value).send({
    from: address.value,
  });
};

const transfer = async () => {
  await bankContract.value.methods
    .transfer(transferAddress.value, transferAmount.value)
    .send({
      from: address.value,
    });
};
</script>

<template>
  <h1>First Dapp</h1>

  <div class="item">
    <button @click="connectWallet">connect wallet</button>
  </div>
  <div class="item">wallet address: {{ address }}</div>
  <div class="item">
    <span>deposite: {{ myDeposit }}</span>
    <button @click="getMyDeposit">get my deposit</button>
  </div>
  <div class="item">
    <label for="fname">Amount:</label>
    <input v-model="depositAmount" />
    <button @click="deposit">Deposit</button>
  </div>

  <div class="item">
    <label for="fname">Amount:</label>
    <input v-model="withdrawAmount" />
    <button @click="withdraw">Withdraw</button>
  </div>

  <div class="item">
    <label for="fname">Address to:</label>
    <input v-model="transferAddress" />

    <label for="fname">Amount:</label>
    <input v-model="transferAmount" />
    <button @click="transfer">Transfer</button>
  </div>
</template>

<style scoped>
.item {
  margin-bottom: 8px;
}
</style>
