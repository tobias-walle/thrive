import { RefObject, useEffect } from "react"

export function useHTMLElementEventListener<
  K extends keyof HTMLElementEventMap
>(
  ref: RefObject<HTMLElement | null | undefined>,
  event: K,
  listener: (event: HTMLElementEventMap[K]) => void
): void {
  useEffect(() => {
    if (!ref.current) return

    ref.current.addEventListener(event, listener)
    return () => ref.current?.removeEventListener(event, listener)
  }, [ref, event, listener])
}
