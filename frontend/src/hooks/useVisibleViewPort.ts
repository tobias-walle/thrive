import { RefObject, useCallback, useEffect, useState } from "react"

import { useHTMLElementEventListener } from "./useHTMLElementEventListener"

export interface VisibleViewPort {
  top: number
  right: number
  bottom: number
  left: number
}

export function useVisibleViewPort(
  ref: RefObject<HTMLElement>
): VisibleViewPort | null {
  const [viewPort, setViewPort] = useState<VisibleViewPort | null>(null)

  const updateViewPort = useCallback(
    () => ref.current && setViewPort(getCurrentVisibleViewPort(ref.current)),
    [setViewPort]
  )

  useHTMLElementEventListener(ref, "resize", updateViewPort)
  useHTMLElementEventListener(ref, "scroll", updateViewPort)
  useEffect(() => void updateViewPort(), [updateViewPort])

  return viewPort
}

function getCurrentVisibleViewPort(element: HTMLElement): VisibleViewPort {
  const left = element.scrollLeft
  const top = element.scrollTop
  const right = left + element.offsetWidth
  const bottom = top + element.offsetHeight
  return {
    top,
    right,
    bottom,
    left,
  }
}
