import("../pkg/index.js")
  .catch(console.error)
  .then(module => module.run_app());
