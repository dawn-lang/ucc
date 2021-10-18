// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

import init, { Ucci } from "./pkg/ucci_web.js";

const PROMPT = "\n>>> ";

async function run() {
  await init();
  let textarea = document.createElement("textarea");
  document.body.appendChild(textarea);
  textarea.focus();
  textarea.setAttribute('autocapitalize', 'off');
  textarea.setAttribute('autocomplete', 'off');
  textarea.setAttribute('autocorrect', 'off');
  textarea.setAttribute('spellcheck', 'false');
  document.body.style = `
    margin: 0;
    border: 0;
    padding: 0;
  `;
  textarea.style = `
    margin: 0;
    border: 0;
    outline: none;
    padding: 0;
    position: absolute;
    top: 10px;
    left: 10px;
    width: calc(100vw - 20px);
    height: calc(100vh - 20px);
    resize: none;
    font-family: monospace,monospace;
    font-size: 10pt;
    autocorrect: off;
    autocapitalize: none;
    spellcheck: false;
  `;

  textarea.value =
    `\
Untyped Concatenative Calculus Interpreter (UCCI)
Type ":help" to see the available commands.
` + PROMPT;

  let ucci = new Ucci();
  textarea.addEventListener("keydown", (ev) => {
    if (ev.key == "Enter") {
      if (
        textarea.selectionStart === textarea.selectionEnd &&
        textarea.selectionEnd === textarea.value.length
      ) {
        ev.preventDefault();
        let input = textarea.value.slice(
          textarea.value.lastIndexOf(PROMPT) + PROMPT.length,
          textarea.selectionEnd
        );
        ucci.interp(
          input,
          (output) => (textarea.value += "\n" + output + PROMPT)
        );
        textarea.scrollTop = textarea.scrollHeight;
      }
    }
  });
}

run();
