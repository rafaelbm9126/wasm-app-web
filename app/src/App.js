import React from 'react';
import logo from './logo.svg';
import './App.css';

import * as wasm from "wasm-game-of-life";

// wasm.greet();

let foo = new wasm.Foo(10);

console.log(foo);
console.log(foo.get());
console.log(foo.set(12));
console.log(foo.get());

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
