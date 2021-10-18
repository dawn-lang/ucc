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
  textarea.setAttribute("autocapitalize", "off");
  textarea.setAttribute("autocomplete", "off");
  textarea.setAttribute("autocorrect", "off");
  textarea.setAttribute("spellcheck", "false");
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

  function write_output(output) {
    textarea.value += output;
    textarea.scrollTop = textarea.scrollHeight;
  }

  let ucci = new Ucci();

  function step() {
    if (ucci.is_done()) {
      textarea.value += PROMPT;
      textarea.scrollTop = textarea.scrollHeight;
    } else {
      ucci.interp_step(write_output);
      setTimeout(step);
    }
  }

  textarea.addEventListener("keydown", (ev) => {
    if (ev.key == "Enter") {
      if (
        textarea.selectionStart === textarea.selectionEnd &&
        textarea.selectionEnd === textarea.value.length
      ) {
        ev.preventDefault();
        textarea.value += "\n";
        let input = textarea.value.slice(
          textarea.value.lastIndexOf(PROMPT) + PROMPT.length,
          textarea.selectionEnd
        );
        ucci.interp_start(input, write_output);
        setTimeout(step);
      }
    }
    if (
      ev.ctrlKey &&
      ev.key == "c" &&
      textarea.selectionStart === textarea.selectionEnd &&
      textarea.selectionEnd === textarea.value.length
    ) {
      ucci.interp_start("", write_output);
      setTimeout(step);
    }
  });
}

run();
