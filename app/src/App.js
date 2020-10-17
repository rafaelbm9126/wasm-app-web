import React from 'react';
import logo from './logo.svg';
import './App.css';

import * as wasm from "wasm-game-of-life";

let form = new wasm.Form("Rafael", 10);

console.log( form );
console.log( form.get_name );
console.log( form.get_age );

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
