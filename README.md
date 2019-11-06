<h1><code>spell-checker-blueheart</code></h1>

  <strong>A simple spell checker <small> - *do not use in production!</strong>
  <h3>Built with 🦀🕸</h3>
  <p>
  Check <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a> for more info
  </p>
  <h3>Use example</h3>

  ```javascript

  import { Trie } from "spell-checker-blueheart";
  const trie = Trie.new();
  async function loadDict(file) {
    const dict = await  fetch(file)
      .then(t => t.text()).then(t => trie.load(t));
  }
  loadDict("checker.txt").then(t =>
    {
      console.log(trie.search("memory"));
      console.log(trie.search("dog"));
      console.log(trie.search("resolution"));
      let result = "the foz/jumps\\the-killer!train and ctches fyre".split(/[^\w']+/);
      result = new Set(result);
      result = [...result].filter(w => !trie.search(w));
      console.log(result);
  });
  ```
