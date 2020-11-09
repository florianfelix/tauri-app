<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <div class="counter">{{ count }}</div>
    <div class="incr" v-on:click="increment">Increment</div>
    <div class="incr" v-on:click="incrementPromisified">
      Increment Promisified
    </div>
  </div>
</template>


<script>
// eslint-disable-next-line no-unused-vars
import { invoke } from "tauri/api/tauri";
// eslint-disable-next-line no-unused-vars
import { promisified } from "tauri/api/tauri";

export default {
  name: "HelloWorld",
  data() {
    return {
      count: 0,
    };
  },
  props: {
    msg: String,
  },
  methods: {
    increment() {
      invoke({
        cmd: "increment",
        payload: {
          count: this.count,
        },
      });
    },
    incrementPromisified() {
      promisified({
        cmd: "incrementPromise",
        payload: {
          count: this.count,
        },
      })
        .then((response) => {
          // do something with the Ok() response
          // eslint-disable-next-line no-unused-vars
          const { newcount, message } = response;
          console.log(newcount, message);
          this.count = newcount;
        })
        .catch((error) => {
          // do something with the Err() response string
          console.log(error);
        });
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}

.counter {
  font-size: 3rem;
}

.incr {
  border: 1px solid black;
  border-radius: 5px;
  display: block;
  padding: 1rem;
  font-size: 2rem;
  font-weight: bold;
  margin-top: 20px;
  cursor: pointer;
}
</style>
