import React from "react"
import styled from "styled-components"

interface CellProps {
  x: number
  y: number
  width: number
  height: number
  value?: string
}

export function Cell(props: CellProps) {
  return <Wrapper cell={props}>{props.value}</Wrapper>
}

const Wrapper = styled.div<{ cell: CellProps }>`
  position: absolute;
  left: ${(p) => p.cell.x}px;
  top: ${(p) => p.cell.y}px;
  display: flex;
  justify-content: center;
  align-items: center;
  height: ${(p) => p.cell.height}px;
  width: ${(p) => p.cell.width}px;
  box-sizing: border-box;
  border-left: 1px solid black;
  border-top: 1px solid black;
`
