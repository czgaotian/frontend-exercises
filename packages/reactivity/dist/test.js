const {effect, reactive} = require("./reactivity.cjs")

const state = reactive({name: 'gao', age: 18})
console.log(state)
effect(() => {
  console.log(state.name)
})
state.name = 'tian'