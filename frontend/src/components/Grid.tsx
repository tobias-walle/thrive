import { range } from "rambda"
import React, { useEffect, useLayoutEffect, useMemo, useRef } from "react"
import styled from "styled-components"

import {
  useVisibleViewPort,
  VisibleViewPort,
} from "../hooks/useVisibleViewPort"
import { Cell } from "./Cell"

interface GridProps {}

export function Grid(props: GridProps) {
  const outerRef = useRef<HTMLDivElement | null>(null)
  const viewPort: VisibleViewPort = useVisibleViewPort(outerRef) ?? {
    left: 0,
    right: 0,
    bottom: 0,
    top: 0,
  }

  const buffer = 2
  const gridSize = 10_000
  const cellWidthInPx = 120
  const cellHeightInPx = 40
  const firstCol = Math.max(
    0,
    Math.floor(viewPort.left / cellWidthInPx) - buffer
  )
  const lastCol = Math.ceil(viewPort.right / cellWidthInPx) + buffer
  const firstRow = Math.max(
    0,
    Math.floor(viewPort.top / cellHeightInPx) - buffer
  )
  const lastRow = Math.ceil(viewPort.bottom / cellHeightInPx) + buffer

  const cells = useMemo(() => {
    return range(firstRow, lastRow).map((row) =>
      range(firstCol, lastCol).map((col) => {
        const name = `${row + 1}.${col + 1}`
        return (
          <Cell
            key={name}
            x={cellWidthInPx * col}
            y={cellHeightInPx * row}
            width={cellWidthInPx}
            height={cellHeightInPx}
            value={name}
          />
        )
      })
    )
  }, [firstRow, lastRow, firstCol, lastCol, cellHeightInPx, cellWidthInPx])

  return (
    <Outer ref={outerRef}>
      <Inner
        style={{
          height: cellHeightInPx * gridSize,
          width: cellWidthInPx * gridSize,
        }}
      >
        {cells}
      </Inner>
    </Outer>
  )
}

const Outer = styled.div`
  position: relative;
  height: 100%;
  width: 100%;
  overflow: scroll;
`

const Inner = styled.div`
  position: relative;
  overflow: hidden;
`
