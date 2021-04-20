import React from "react"
import { createGlobalStyle } from "styled-components"
import reset from "styled-reset"

import { Sheet } from "./Sheet"

const GlobalStyle = createGlobalStyle`
  ${reset}
  
  html {
    font-size: 16px;
    font-family: sans-serif;
  }
`

export function App() {
  return (
    <div>
      <GlobalStyle />
      <Sheet />
    </div>
  )
}
