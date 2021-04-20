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

type WrapperProps = { cell: CellProps }
const Wrapper = styled.div.attrs<WrapperProps>((p) => ({
  style: {
    left: p.cell.x,
    top: p.cell.y,
    width: p.cell.width,
    height: p.cell.height,
  },
}))<WrapperProps>`
  position: absolute;
  display: flex;
  justify-content: center;
  align-items: center;
  box-sizing: border-box;
  border-left: 1px solid black;
  border-top: 1px solid black;
`
