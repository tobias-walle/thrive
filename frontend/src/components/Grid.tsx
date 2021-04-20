import { range } from "rambda"
import React from "react"

import { Cell } from "./Cell"

interface GridProps {}

export function Grid(props: GridProps) {
  const height = 30
  const width = 20
  const cellWidthInPx = 120
  const cellHeightInPx = 40
  return (
    <div
      css={`
        position: relative;
      `}
    >
      {range(0, height).map((row) =>
        range(0, width).map((col) => {
          return (
            <Cell
              x={cellWidthInPx * col}
              y={cellHeightInPx * row}
              width={cellWidthInPx}
              height={cellHeightInPx}
              value={`${row + 1}.${col + 1}`}
            />
          )
        })
      )}
    </div>
  )
}
